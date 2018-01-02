extern crate itertools;
use itertools::Itertools;

pub fn encode(input: &str) -> String {
    if input.is_empty() { return "".to_string() }

    let mut output = String::new();

    for (key, group) in &input.chars().group_by(|el| *el) {
        let char_count = group.count();
        if char_count > 1 {
            output.push_str(char_count.to_string().as_str());
        }
        output.push(key);
    }

    output
}

pub fn decode(input: &str) -> String {
    let mut nums = String::new();

    input.chars().map(|e| {
        if e.is_numeric() {
            nums.push(e);
            String::new()
        } else {
            let num = nums.parse().unwrap_or(1);
            nums.clear();
            e.to_string().repeat(num)
        }
    }).collect()
}
