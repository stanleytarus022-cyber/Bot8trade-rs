use chrono::{DateTime, Utc};

pub fn format_price(price: f64, decimals: usize) -> String {
    format!("{:.prec$}", price, prec = decimals)
}

pub fn calculate_percentage_change(old: f64, new: f64) -> f64 {
    if old == 0.0 {
        return 0.0;
    }
    ((new - old) / old) * 100.0
}

pub fn timestamp_to_string(ts: DateTime<Utc>) -> String {
    ts.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn validate_price(price: f64) -> bool {
    price > 0.0 && price.is_finite()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_change() {
        assert_eq!(calculate_percentage_change(100.0, 110.0), 10.0);
        assert_eq!(calculate_percentage_change(100.0, 90.0), -10.0);
    }

    #[test]
    fn test_validate_price() {
        assert!(validate_price(100.0));
        assert!(!validate_price(-100.0));
        assert!(!validate_price(f64::NAN));
    }
}
