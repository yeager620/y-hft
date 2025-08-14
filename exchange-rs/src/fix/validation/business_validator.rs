use crate::fix::error::BusinessError;
use crate::fix::messages::NewOrderSingle;
use std::collections::HashSet;

pub struct BusinessValidator {
    active_cl_ord_ids: HashSet<String>,
    valid_symbols: HashSet<String>,
}

impl BusinessValidator {
    pub fn new() -> Self {
        let mut valid_symbols = HashSet::new();
        valid_symbols.insert("AAPL".to_string());
        valid_symbols.insert("GOOGL".to_string());
        valid_symbols.insert("MSFT".to_string());
        valid_symbols.insert("TSLA".to_string());
        valid_symbols.insert("NVDA".to_string());
        
        Self {
            active_cl_ord_ids: HashSet::new(),
            valid_symbols,
        }
    }

    pub fn validate_new_order(&mut self, order: &NewOrderSingle) -> Result<(), BusinessError> {
        self.validate_symbol(&order.symbol)?;
        self.validate_quantity(order.order_qty)?;
        self.validate_price(order.price, order.ord_type)?;
        self.validate_stop_price(order.stop_px, order.ord_type)?;
        self.validate_duplicate_cl_ord_id(&order.cl_ord_id)?;
        
        self.active_cl_ord_ids.insert(order.cl_ord_id.clone());
        Ok(())
    }

    pub fn validate_cancel_request(&self, orig_cl_ord_id: &str, new_cl_ord_id: &str) -> Result<(), BusinessError> {
        if !self.active_cl_ord_ids.contains(orig_cl_ord_id) {
            return Err(BusinessError::OrderNotFound {
                cl_ord_id: orig_cl_ord_id.to_string(),
            });
        }

        if self.active_cl_ord_ids.contains(new_cl_ord_id) {
            return Err(BusinessError::DuplicateClOrdId {
                cl_ord_id: new_cl_ord_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn add_symbol(&mut self, symbol: String) {
        self.valid_symbols.insert(symbol);
    }

    pub fn remove_symbol(&mut self, symbol: &str) {
        self.valid_symbols.remove(symbol);
    }

    pub fn complete_order(&mut self, cl_ord_id: &str) {
        self.active_cl_ord_ids.remove(cl_ord_id);
    }

    fn validate_symbol(&self, symbol: &str) -> Result<(), BusinessError> {
        if !self.valid_symbols.contains(symbol) {
            return Err(BusinessError::InvalidSymbol {
                symbol: symbol.to_string(),
            });
        }
        Ok(())
    }

    fn validate_quantity(&self, quantity: u32) -> Result<(), BusinessError> {
        if quantity == 0 {
            return Err(BusinessError::InvalidQuantity { quantity });
        }
        
        if quantity > 1_000_000 {
            return Err(BusinessError::InvalidQuantity { quantity });
        }
        
        Ok(())
    }

    fn validate_price(&self, price: Option<f64>, ord_type: char) -> Result<(), BusinessError> {
        match ord_type {
            '2' | '4' => {
                if let Some(p) = price {
                    if p <= 0.0 || !p.is_finite() {
                        return Err(BusinessError::InvalidPrice {
                            price: (p * 10000.0) as u64,
                        });
                    }
                    
                    if p > 1_000_000.0 {
                        return Err(BusinessError::InvalidPrice {
                            price: (p * 10000.0) as u64,
                        });
                    }
                } else {
                    return Err(BusinessError::InvalidPrice { price: 0 });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_stop_price(&self, stop_px: Option<f64>, ord_type: char) -> Result<(), BusinessError> {
        match ord_type {
            '3' | '4' => {
                if let Some(p) = stop_px {
                    if p <= 0.0 || !p.is_finite() {
                        return Err(BusinessError::InvalidPrice {
                            price: (p * 10000.0) as u64,
                        });
                    }
                    
                    if p > 1_000_000.0 {
                        return Err(BusinessError::InvalidPrice {
                            price: (p * 10000.0) as u64,
                        });
                    }
                } else {
                    return Err(BusinessError::InvalidPrice { price: 0 });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_duplicate_cl_ord_id(&self, cl_ord_id: &str) -> Result<(), BusinessError> {
        if self.active_cl_ord_ids.contains(cl_ord_id) {
            return Err(BusinessError::DuplicateClOrdId {
                cl_ord_id: cl_ord_id.to_string(),
            });
        }
        Ok(())
    }
}

impl Default for BusinessValidator {
    fn default() -> Self {
        Self::new()
    }
}