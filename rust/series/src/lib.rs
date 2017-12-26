pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits: String = digits.to_string();
    let mut result: Vec<String> = vec![];

    for start in 0..digits.len() + 1 {
        if start + len > digits.len() { break }
        result.push(digits[start..start+len].to_string());
    }

    result
}
