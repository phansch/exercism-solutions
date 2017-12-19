pub fn reply(message: &str) -> &str {
    let chars: Vec<char> = message.chars().collect();
    // if all uppercase: Whoa, chill out!
    // if question: Sure.
    // empty string: Fine, be that way!
    // Whatever to anything else
    if chars.to_vec().into_iter().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() == 0 {
        "Whoa, chill out!"
    } else if chars.to_vec().into_iter().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }

}
