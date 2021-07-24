use web3::types::U256;

pub fn with_commas(s: &str) -> String {
    if s.len() <= 3 {
        return s.to_string();
    }
    let count = s.len() / 3 + 1;
    let mut result = String::with_capacity(s.len() + count);
    for (index, digit) in s.chars().enumerate() {
        result.push(digit);
        if (s.len() - index) % 3 == 1 && index != s.len() - 1 {
            result.push(',');
        }
    }
    result
}

pub fn amount(src: U256, decimals: usize) -> String {
    let str = format!("{}", src);
    if src == U256::from(0) {
        return "0".to_owned();
    }
    if str.len() > decimals {
        let before_dot: String = str.chars().take(str.len() - decimals).collect();
        let right_rev: String = str.chars().rev().take(decimals).collect();
        let after_dot: String = right_rev.chars().rev().collect();
        return format!("{}.{}", with_commas(&before_dot), after_dot);
    }
    let pad_size = decimals - str.len();
    let pad = (0..pad_size).map(|_| " ").collect::<String>();
    let right_rev: String = str.chars().rev().take(str.len()).collect();
    let after_dot: String = right_rev.chars().rev().collect();
    return format!("0.{}{}", pad, after_dot);
}

pub fn ceil(src: U256, decimals: usize) -> String {
    let str = format!("{}", src);
    if src == U256::from(0) {
        return "0".to_owned();
    }
    if str.len() > decimals {
        let s: String = str.chars().take(str.len() - decimals).collect();
        return with_commas(&s);
    }
    return "0".to_owned();
}

pub fn int<T>(src: T) -> String
where
    T: std::fmt::Display,
{
    with_commas(format!("{}", src).as_str())
}

pub fn dec<T>(src: T, decimals: usize) -> f64
where
    T: std::fmt::Display,
{
    let str = format!("{}", src);
    if str.len() > decimals {
        let before_dot: String = str.chars().take(str.len() - decimals).collect();
        return before_dot.parse().unwrap();
    }
    0f64
}

// this is actually cutting decimals,
// so it is far from being accurate
pub fn pct_of(amt: U256, total: U256, decimals: usize) -> String {
    let prec = decimals + 2;
    let value: f64 = 100.0 * dec(amt, prec) / dec(total, prec);
    format!("{:.2}", value)
}

// getting APY from APR
pub fn apy(apr: f64) -> f64 {
    (1.0 + apr / 52.0).powf(52.0) - 1.0
}

pub fn date(tm: u64) -> String {
    let dt = chrono::NaiveDateTime::from_timestamp(tm as i64, 0);
    format!("{:?}", dt).replace("T", " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    pub fn test_amount_under_1() -> Result<(), String> {
        let val = U256::from_str("5843424da37c000").unwrap();
        assert_eq!(amount(val, 18), "0.397500000000000000");
        Ok(())
    }

    #[test]
    pub fn test_amount_over_1() -> Result<(), String> {
        let val = U256::from_str("aaa4f9440299734000").unwrap();
        assert_eq!(amount(val, 18), "3,147.834100000000000000");
        Ok(())
    }

    #[test]
    pub fn test_thousands() {
        assert_eq!(with_commas("12833279"), "12,833,279");
        assert_eq!(with_commas("2689"), "2,689");
        assert_eq!(with_commas("689"), "689");
        assert_eq!(with_commas("68"), "68");
        assert_eq!(with_commas("6"), "6");
    }
}
