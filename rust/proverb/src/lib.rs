pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0 { return String::new() }

    let mut output = vec![];
    let first = list[0];
    let mut previous: &str;

    for (index, item) in list.iter().enumerate() {
        if item == &first { continue; }

        previous = list[index - 1];
        output.push(format!("For want of a {} the {} was lost.", previous, item));
    }
    output.push(format!("And all for the want of a {}.", first));
    output.join("\n")
}
