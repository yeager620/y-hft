

pub const PRICE_SCALE_FACTOR: u64 = 1_000_000; 
pub const QUANTITY_SCALE_FACTOR: u32 = 1000; 

pub fn float_to_scaled_price(price: f64) -> Result<u64, String> {
    if price < 0.0 || !price.is_finite() {
        return Err(format!("Invalid price: {}", price));
    }
    Ok((price * PRICE_SCALE_FACTOR as f64) as u64)
}

pub fn scaled_price_to_float(price: u64) -> f64 {
    price as f64 / PRICE_SCALE_FACTOR as f64
}

pub fn float_to_scaled_quantity(quantity: f64) -> Result<u32, String> {
    if quantity < 0.0 || !quantity.is_finite() {
        return Err(format!("Invalid quantity: {}", quantity));
    }
    Ok((quantity * QUANTITY_SCALE_FACTOR as f64) as u32)
}

pub fn scaled_quantity_to_float(quantity: u32) -> f64 {
    quantity as f64 / QUANTITY_SCALE_FACTOR as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_conversion() {
        let price = 123.456789;
        let scaled = float_to_scaled_price(price).unwrap();
        assert_eq!(scaled, 123456789); 

        let converted_back = scaled_price_to_float(scaled);
        assert!((converted_back - price).abs() < 0.000001); 
    }

    #[test]
    fn test_quantity_conversion() {
        let quantity = 10.5;
        let scaled = float_to_scaled_quantity(quantity).unwrap();
        assert_eq!(scaled, 10500); 

        let converted_back = scaled_quantity_to_float(scaled);
        assert!((converted_back - quantity).abs() < 0.001);
    }

    #[test]
    fn test_invalid_price() {
        assert!(float_to_scaled_price(-1.0).is_err());
        assert!(float_to_scaled_price(f64::INFINITY).is_err());
        assert!(float_to_scaled_price(f64::NAN).is_err());
    }

    #[test]
    fn test_invalid_quantity() {
        assert!(float_to_scaled_quantity(-1.0).is_err());
        assert!(float_to_scaled_quantity(f64::INFINITY).is_err());
        assert!(float_to_scaled_quantity(f64::NAN).is_err());
    }
}