pub mod order_converter;
pub mod response_converter;

pub use order_converter::FixOrderConverter;
pub use response_converter::FixResponseConverter;

use crate::fix::error::{FixError, BusinessError};
use crate::fix::messages::{NewOrderSingle, FixMessage};
use crate::fix::validation::BusinessValidator;
use crate::order::{Order, OrderType, Side, TimeInForce};
use crate::matching_engine::TradeExecutionResult;

pub struct FixOrderBridge {
    converter: FixOrderConverter,
    response_converter: FixResponseConverter,
    validator: BusinessValidator,
}

impl FixOrderBridge {
    pub fn new() -> Self {
        Self {
            converter: FixOrderConverter::new(),
            response_converter: FixResponseConverter::new(),
            validator: BusinessValidator::new(),
        }
    }

    pub fn process_fix_message(&mut self, message: FixMessage) -> Result<Option<Order>, FixError> {
        match message {
            FixMessage::NewOrderSingle(order) => {
                self.validator.validate_new_order(&order)?;
                let internal_order = self.converter.convert_new_order_single(order)?;
                Ok(Some(internal_order))
            }
            FixMessage::OrderCancelRequest(cancel) => {
                self.validator.validate_cancel_request(&cancel.orig_cl_ord_id, &cancel.cl_ord_id)?;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn convert_trade_result(&mut self, result: &TradeExecutionResult, cl_ord_id: &str) -> Result<FixMessage, FixError> {
        self.response_converter.convert_trade_result(result, cl_ord_id)
    }

    pub fn add_symbol(&mut self, symbol: String) {
        self.validator.add_symbol(symbol);
    }

    pub fn complete_order(&mut self, cl_ord_id: &str) {
        self.validator.complete_order(cl_ord_id);
    }
}

impl Default for FixOrderBridge {
    fn default() -> Self {
        Self::new()
    }
}