pub fn check(input: &str) -> bool {
    let input = input.to_lowercase();
    let mut deduped: Vec<char> = String::from(input)
        .chars()
        .filter(|&c| c != '-' && c != ' ')
        .collect::<Vec<char>>();
    let mut copy = deduped.clone();

    copy.sort();
    deduped.sort();
    deduped.dedup();

    if copy == deduped {
        true
    } else {
        false
    }
}
