use crate::fix::error::FixError;
use crate::fix::messages::{
    FixMessage, ExecutionReport, StandardHeader, Trailer, MessageType,
    execution_report::{ExecType, OrdStatus},
};
use crate::matching_engine::TradeExecutionResult;
use crate::order::{OrderStatus, OrderType, Side};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FixResponseConverter {
    next_exec_id: u64,
}

impl FixResponseConverter {
    pub fn new() -> Self {
        Self {
            next_exec_id: 1,
        }
    }

    pub fn convert_trade_result(&mut self, result: &TradeExecutionResult, cl_ord_id: &str) -> Result<FixMessage, FixError> {
        if result.rejected {
            return self.create_rejection_execution_report(cl_ord_id, "Order rejected");
        }

        if !result.trades.is_empty() {
            self.create_trade_execution_report(result, cl_ord_id)
        } else if result.remaining_order.is_some() {
            self.create_new_execution_report(result, cl_ord_id)
        } else {
            self.create_rejection_execution_report(cl_ord_id, "No action taken")
        }
    }

    fn create_trade_execution_report(&mut self, result: &TradeExecutionResult, cl_ord_id: &str) -> Result<FixMessage, FixError> {
        let trade = &result.trades[0];
        
        let remaining_order = result.remaining_order.as_ref()
            .or_else(|| result.filled_orders.first())
            .ok_or_else(|| FixError::Parse(crate::fix::error::ParseError::InvalidFormat))?;
        
        let order = remaining_order.read();
        
        let exec_type = if order.is_filled() { 
            ExecType::Fill 
        } else { 
            ExecType::PartialFill 
        };
        
        let ord_status = if order.is_filled() { 
            OrdStatus::Filled 
        } else { 
            OrdStatus::PartiallyFilled 
        };

        let header = self.create_standard_header(MessageType::ExecutionReport)?;
        let trailer = Trailer { checksum: 0 };

        let execution_report = ExecutionReport {
            header,
            order_id: order.id.to_string(),
            cl_ord_id: cl_ord_id.to_string(),
            orig_cl_ord_id: None,
            exec_id: self.next_exec_id().to_string(),
            exec_type: exec_type.to_char(),
            ord_status: ord_status.to_char(),
            account: None,
            symbol: order.symbol.clone(),
            side: self.convert_side_to_char(order.side),
            order_qty: order.quantity,
            ord_type: self.convert_order_type_to_char(order.order_type),
            price: if matches!(order.order_type, OrderType::Limit | OrderType::StopLimit) {
                Some(order.price as f64 / 10000.0)
            } else {
                None
            },
            stop_px: order.stop_price.map(|p| p as f64 / 10000.0),
            time_in_force: Some(self.convert_time_in_force_to_char(order.time_in_force)),
            last_qty: Some(trade.quantity),
            last_px: Some(trade.price as f64 / 10000.0),
            leaves_qty: order.remaining_quantity(),
            cum_qty: order.filled_quantity,
            avg_px: Some(trade.price as f64 / 10000.0),
            transact_time: self.get_utc_timestamp(),
            text: None,
            trailer,
        };

        Ok(FixMessage::ExecutionReport(execution_report))
    }

    fn create_new_execution_report(&mut self, result: &TradeExecutionResult, cl_ord_id: &str) -> Result<FixMessage, FixError> {
        let remaining_order = result.remaining_order.as_ref()
            .ok_or_else(|| FixError::Parse(crate::fix::error::ParseError::InvalidFormat))?;
        
        let order = remaining_order.read();

        let header = self.create_standard_header(MessageType::ExecutionReport)?;
        let trailer = Trailer { checksum: 0 };

        let execution_report = ExecutionReport {
            header,
            order_id: order.id.to_string(),
            cl_ord_id: cl_ord_id.to_string(),
            orig_cl_ord_id: None,
            exec_id: self.next_exec_id().to_string(),
            exec_type: ExecType::New.to_char(),
            ord_status: OrdStatus::New.to_char(),
            account: None,
            symbol: order.symbol.clone(),
            side: self.convert_side_to_char(order.side),
            order_qty: order.quantity,
            ord_type: self.convert_order_type_to_char(order.order_type),
            price: if matches!(order.order_type, OrderType::Limit | OrderType::StopLimit) {
                Some(order.price as f64 / 10000.0)
            } else {
                None
            },
            stop_px: order.stop_price.map(|p| p as f64 / 10000.0),
            time_in_force: Some(self.convert_time_in_force_to_char(order.time_in_force)),
            last_qty: None,
            last_px: None,
            leaves_qty: order.remaining_quantity(),
            cum_qty: order.filled_quantity,
            avg_px: None,
            transact_time: self.get_utc_timestamp(),
            text: None,
            trailer,
        };

        Ok(FixMessage::ExecutionReport(execution_report))
    }

    fn create_rejection_execution_report(&mut self, cl_ord_id: &str, reason: &str) -> Result<FixMessage, FixError> {
        let header = self.create_standard_header(MessageType::ExecutionReport)?;
        let trailer = Trailer { checksum: 0 };

        let execution_report = ExecutionReport {
            header,
            order_id: "0".to_string(),
            cl_ord_id: cl_ord_id.to_string(),
            orig_cl_ord_id: None,
            exec_id: self.next_exec_id().to_string(),
            exec_type: ExecType::Rejected.to_char(),
            ord_status: OrdStatus::Rejected.to_char(),
            account: None,
            symbol: "".to_string(),
            side: '1',
            order_qty: 0,
            ord_type: '2',
            price: None,
            stop_px: None,
            time_in_force: None,
            last_qty: None,
            last_px: None,
            leaves_qty: 0,
            cum_qty: 0,
            avg_px: None,
            transact_time: self.get_utc_timestamp(),
            text: Some(reason.to_string()),
            trailer,
        };

        Ok(FixMessage::ExecutionReport(execution_report))
    }

    fn create_standard_header(&self, msg_type: MessageType) -> Result<StandardHeader, FixError> {
        Ok(StandardHeader {
            begin_string: "FIX.4.4".to_string(),
            body_length: 0, 
            msg_type,
            sender_comp_id: "EXCHANGE".to_string(),
            target_comp_id: "CLIENT".to_string(),
            msg_seq_num: 1, 
            sending_time: self.get_utc_timestamp(),
            poss_dup_flag: None,
            poss_resend: None,
            secure_data_len: None,
            secure_data: None,
        })
    }

    fn convert_side_to_char(&self, side: Side) -> char {
        match side {
            Side::Buy => '1',
            Side::Sell => '2',
        }
    }

    fn convert_order_type_to_char(&self, order_type: OrderType) -> char {
        match order_type {
            OrderType::Market => '1',
            OrderType::Limit => '2',
            OrderType::StopMarket => '3',
            OrderType::StopLimit => '4',
            OrderType::Iceberg => '2',
        }
    }

    fn convert_time_in_force_to_char(&self, time_in_force: crate::order::TimeInForce) -> char {
        match time_in_force {
            crate::order::TimeInForce::Day => '0',
            crate::order::TimeInForce::GTC => '1',
            crate::order::TimeInForce::IOC => '3',
            crate::order::TimeInForce::FOK => '4',
            crate::order::TimeInForce::GTD => '6',
        }
    }

    fn get_utc_timestamp(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        format!("{:04}{:02}{:02}-{:02}:{:02}:{:02}",
            2024, 1, 1, 
            (now / 3600) % 24,
            (now / 60) % 60,
            now % 60
        )
    }

    fn next_exec_id(&mut self) -> u64 {
        let id = self.next_exec_id;
        self.next_exec_id += 1;
        id
    }
}

impl Default for FixResponseConverter {
    fn default() -> Self {
        Self::new()
    }
}