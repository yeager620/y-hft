use crate::fix::error::{FixError, BusinessError};
use crate::fix::messages::NewOrderSingle;
use crate::order::{Order, OrderType, Side, TimeInForce};

pub struct FixOrderConverter;

impl FixOrderConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_new_order_single(&self, fix_order: NewOrderSingle) -> Result<Order, FixError> {
        let side = self.convert_side(fix_order.side)?;
        let order_type = self.convert_order_type(fix_order.ord_type)?;
        let time_in_force = self.convert_time_in_force(fix_order.time_in_force);
        
        let price = self.convert_price(fix_order.price, fix_order.ord_type)?;
        let stop_price = self.convert_stop_price(fix_order.stop_px, fix_order.ord_type)?;
        
        let user_id = self.extract_user_id(&fix_order.header.sender_comp_id);
        
        let mut order = Order::new(
            fix_order.symbol,
            side,
            order_type,
            price,
            fix_order.order_qty,
            user_id,
        );

        order.time_in_force = time_in_force;
        order.stop_price = stop_price;
        
        if let Some(account) = fix_order.account {
            if !account.is_empty() {
                order.user_id = self.extract_user_id(&account);
            }
        }

        Ok(order)
    }

    fn convert_side(&self, fix_side: char) -> Result<Side, BusinessError> {
        match fix_side {
            '1' => Ok(Side::Buy),
            '2' => Ok(Side::Sell),
            _ => Err(BusinessError::InvalidSymbol {
                symbol: format!("Invalid side: {}", fix_side),
            }),
        }
    }

    fn convert_order_type(&self, fix_ord_type: char) -> Result<OrderType, BusinessError> {
        match fix_ord_type {
            '1' => Ok(OrderType::Market),
            '2' => Ok(OrderType::Limit),
            '3' => Ok(OrderType::StopMarket),
            '4' => Ok(OrderType::StopLimit),
            _ => Err(BusinessError::InvalidSymbol {
                symbol: format!("Invalid order type: {}", fix_ord_type),
            }),
        }
    }

    fn convert_time_in_force(&self, fix_tif: Option<char>) -> TimeInForce {
        match fix_tif {
            Some('0') => TimeInForce::Day,
            Some('1') => TimeInForce::GTC,
            Some('3') => TimeInForce::IOC,
            Some('4') => TimeInForce::FOK,
            _ => TimeInForce::GTC, 
        }
    }

    fn convert_price(&self, fix_price: Option<f64>, ord_type: char) -> Result<u64, BusinessError> {
        match ord_type {
            '2' | '4' => {
                match fix_price {
                    Some(price) => {
                        if price <= 0.0 || !price.is_finite() {
                            return Err(BusinessError::InvalidPrice {
                                price: (price * 10000.0) as u64,
                            });
                        }
                        Ok((price * 10000.0) as u64)
                    }
                    None => Err(BusinessError::InvalidPrice { price: 0 }),
                }
            }
            _ => Ok(0),
        }
    }

    fn convert_stop_price(&self, fix_stop_px: Option<f64>, ord_type: char) -> Result<Option<u64>, BusinessError> {
        match ord_type {
            '3' | '4' => {
                match fix_stop_px {
                    Some(price) => {
                        if price <= 0.0 || !price.is_finite() {
                            return Err(BusinessError::InvalidPrice {
                                price: (price * 10000.0) as u64,
                            });
                        }
                        Ok(Some((price * 10000.0) as u64))
                    }
                    None => Err(BusinessError::InvalidPrice { price: 0 }),
                }
            }
            _ => Ok(None),
        }
    }

    fn extract_user_id(&self, comp_id: &str) -> u64 {
        comp_id
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(1)
    }
}

impl Default for FixOrderConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fix::messages::{StandardHeader, Trailer, MessageType};

    #[test]
    fn test_convert_limit_buy_order() {
        let converter = FixOrderConverter::new();
        
        let header = StandardHeader {
            begin_string: "FIX.4.4".to_string(),
            body_length: 100,
            msg_type: MessageType::NewOrderSingle,
            sender_comp_id: "CLIENT123".to_string(),
            target_comp_id: "EXCHANGE".to_string(),
            msg_seq_num: 1,
            sending_time: "20240101-12:00:00".to_string(),
            poss_dup_flag: None,
            poss_resend: None,
            secure_data_len: None,
            secure_data: None,
        };

        let trailer = Trailer { checksum: 123 };

        let fix_order = NewOrderSingle {
            header,
            cl_ord_id: "ORDER123".to_string(),
            account: None,
            handl_inst: '1',
            symbol: "AAPL".to_string(),
            side: '1',
            transact_time: "20240101-12:00:00".to_string(),
            order_qty: 100,
            ord_type: '2',
            price: Some(150.50),
            stop_px: None,
            time_in_force: Some('1'),
            exec_inst: None,
            trailer,
        };

        let order = converter.convert_new_order_single(fix_order).unwrap();
        
        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, Side::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 100);
        assert_eq!(order.price, 1505000); // 150.50 * 10000
        assert_eq!(order.time_in_force, TimeInForce::GTC);
        assert_eq!(order.stop_price, None);
    }

    #[test]
    fn test_convert_stop_limit_sell_order() {
        let converter = FixOrderConverter::new();
        
        let header = StandardHeader {
            begin_string: "FIX.4.4".to_string(),
            body_length: 100,
            msg_type: MessageType::NewOrderSingle,
            sender_comp_id: "CLIENT456".to_string(),
            target_comp_id: "EXCHANGE".to_string(),
            msg_seq_num: 2,
            sending_time: "20240101-12:00:00".to_string(),
            poss_dup_flag: None,
            poss_resend: None,
            secure_data_len: None,
            secure_data: None,
        };

        let trailer = Trailer { checksum: 124 };

        let fix_order = NewOrderSingle {
            header,
            cl_ord_id: "ORDER456".to_string(),
            account: None,
            handl_inst: '1',
            symbol: "TSLA".to_string(),
            side: '2',
            transact_time: "20240101-12:00:00".to_string(),
            order_qty: 50,
            ord_type: '4',
            price: Some(200.00),
            stop_px: Some(195.00),
            time_in_force: Some('4'),
            exec_inst: None,
            trailer,
        };

        let order = converter.convert_new_order_single(fix_order).unwrap();
        
        assert_eq!(order.symbol, "TSLA");
        assert_eq!(order.side, Side::Sell);
        assert_eq!(order.order_type, OrderType::StopLimit);
        assert_eq!(order.quantity, 50);
        assert_eq!(order.price, 2000000); // 200.00 * 10000
        assert_eq!(order.stop_price, Some(1950000)); // 195.00 * 10000
        assert_eq!(order.time_in_force, TimeInForce::FOK);
    }
}