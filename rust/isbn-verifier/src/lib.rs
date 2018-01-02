/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = as_integer_vec(&isbn);
    let mut factor = 10;
    let mut product = 0;

    for num in isbn {
        product += num * factor;
        factor -= 1;
        if factor == 0 { return false }
    }
    if product % 11 == 0 {
        true
    } else {
        false
    }
}

/// Removes hyphens and turns the chars into integers (with X being turned into 10)
fn as_integer_vec(isbn: &str) -> Vec<u32> {
    let without_hyphens: Vec<char> = isbn.chars().filter(|&c| c != '-').collect();
    without_hyphens.iter().map(|&c| {
        if c == 'X' && &'X' == without_hyphens.last().unwrap() {
            10
        } else {
            c.to_digit(10).unwrap_or(0)
        }
    }).collect()
}
