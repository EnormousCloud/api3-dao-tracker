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
