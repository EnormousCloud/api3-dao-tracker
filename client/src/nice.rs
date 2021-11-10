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
    let pad = (0..pad_size).map(|_| "0").collect::<String>();
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

pub fn shifted_float(f: f64, decimals: usize) -> anyhow::Result<U256> {
    let str = format!("{}", f);
    let after_dot: String = match str.find(".") {
        Some(dotpos) => str.chars().skip(dotpos + 1).collect(),
        None => "".to_owned(),
    };
    let fill: String = (0..decimals - after_dot.len()).map(|_| "0").collect();
    let out = format!("{}{}{}", f as i64, after_dot, fill);
    U256::from_dec_str(&out).map_err(|x| anyhow::Error::new(x))
}

pub fn multiplied(i: U256, v: U256, decimals: usize) -> U256 {
    let str = format!("{}", i * v);
    let out: String = str.chars().take(str.len() - decimals).collect();
    U256::from_dec_str(&out).unwrap()
}

pub fn float<T>(src: T, decimals: usize, precision: usize) -> f64
where
    T: std::fmt::Display,
{
    let str = format!("{}", src);
    let (before, after) = if str.len() > decimals {
        let before_dot: String = str.chars().take(str.len() - decimals).collect();
        let right_rev: String = str.chars().rev().take(decimals).collect();
        let after_dot: String = right_rev.chars().rev().take(precision).collect();
        (before_dot, after_dot)
    } else {
        ("0".to_owned(), str.chars().take(precision).collect())
    };
    let combined = if precision > 0 {
        format!("{}.{}", before, after)
    } else {
        before
    };
    combined.parse().unwrap()
}

// this is actually cutting decimals,
// so it is far from being accurate
pub fn pct_of(amt: U256, total: U256, decimals: usize) -> String {
    let prec = decimals + 2;
    let value: f64 = 100.0 * dec(amt, prec) / dec(total, prec);
    format!("{:.2}", value)
}

pub fn pct3_of(amt: U256, total: U256, decimals: usize) -> String {
    let prec = decimals - 2;
    let value: f64 = 100.0 * dec(amt, prec) / dec(total, prec);
    format!("{:.3}", value)
}

pub fn pct4_of(amt: U256, total: U256, decimals: usize) -> String {
    let prec = decimals - 2;
    let value: f64 = 100.0 * dec(amt, prec) / dec(total, prec);
    format!("{:.4}", value)
}

pub fn pct_val(amt: U256, total: U256, decimals: usize) -> f64 {
    let prec = decimals;
    dec(amt, prec) / dec(total, prec)
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

    #[test]
    pub fn test_shifted_float_1() {
        let expected = U256::from_dec_str("1000000000000000000").unwrap();
        assert_eq!(shifted_float(1.0, 18).unwrap(), expected);
    }

    #[test]
    pub fn test_shifted_float() {
        let input = 2126.4424673902;
        let expected = U256::from_dec_str("21264424673902").unwrap();
        assert_eq!(shifted_float(input, 10).unwrap(), expected);
    }

    #[test]
    pub fn test_shifted_float_below1() {
        let input = 0.21264424673902;
        let expected = U256::from_dec_str("21264424673902").unwrap();
        assert_eq!(shifted_float(input, 14).unwrap(), expected);
    }

    #[test]
    pub fn test_multiplied() {
        let decimals = 18;
        let val = U256::from_dec_str("129350385688932754").unwrap();
        let one = shifted_float(1.0, decimals).unwrap();
        assert_eq!(multiplied(val, one, decimals), val);
    }

    #[test]
    pub fn test_float() {
        let val = U256::from_dec_str("129350385688932754").unwrap();
        assert_eq!(float(val, 18, 3), 0.129);
        let val2 = U256::from_dec_str("5129350385688932754").unwrap();
        assert_eq!(float(val2, 18, 3), 5.129);
    }
}
