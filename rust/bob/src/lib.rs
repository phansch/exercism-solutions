pub fn reply(message: &str) -> &str {
    let chars: Vec<char> = message.chars().collect();

    let chars = without_whitespace(chars);

    if is_screaming(&chars) {
        "Whoa, chill out!"
    } else if is_asking(&chars) {
        "Sure."
    } else if is_empty_string(&chars) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }

}

fn is_asking(chars: &Vec<char>) -> bool {
    chars.last().unwrap_or(&' ') == &'?'
}

fn is_screaming(chars: &Vec<char>) -> bool {
    let uppercases: Vec<bool> = chars.iter().filter(|c| c.is_alphabetic()).map(|c| c.is_uppercase()).collect();

    // Because 'all' returns true on an empty collection.
    if uppercases.len() > 0 {
        uppercases.into_iter().all(|c| c == true)
    } else {
        return false
    }
}

fn without_whitespace(chars: Vec<char>) -> Vec<char> {
    chars.into_iter().filter(|c| !c.is_whitespace()).collect::<Vec<char>>()
}

fn is_empty_string(chars: &Vec<char>) -> bool {
    chars.len() == 0
}
