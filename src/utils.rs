pub fn is_numeric(n: &str) -> bool {
    n.parse::<f64>().is_ok()
}
