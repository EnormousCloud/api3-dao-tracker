use web3::types::U256;

pub fn amount(src: U256, decimals: usize) -> String {
    let str = format!("{}", src);
    if str.len() > decimals {
        let before_dot: String = str.chars().take(str.len() - decimals).collect();
        let after_dot: String = str.chars().rev().take(decimals).collect();
        return format!("{}.{}", before_dot, after_dot);
    }
    let pad_size = decimals - str.len();
    let pad = (0..pad_size).map(|_| " ").collect::<String>();
    let after_dot: String = str.chars().rev().take(str.len()).collect();
    return format!("0.{}{}", pad, after_dot);
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::str::FromStr;
//     #[test]
//     pub fn test_amount() -> Result<(), String> {
//         let val = U256::from_dec_str("65958493526413174640938858").unwrap();
//         println!("val= {:?}", val);
//         assert_eq!(
//             amount(val, 18),
//             "65958493.526413174640938858");
//         Ok(())
//     }

//     #[test]
//     pub fn test_pct_of() -> Result<(), String>{
//         assert_eq!(
//             pct_of(
//                 U256::from_dec_str("5013331425976394168029756").unwrap(),
//                 U256::from_dec_str("65958493526413174640938858").unwrap(),
//                 18,
//             ),
//             "100");
//         Ok(())
//     }
// }
