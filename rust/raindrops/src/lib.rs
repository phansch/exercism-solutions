pub fn raindrops(n: usize) -> String {
    let mut output = String::from("");

    if n % 3 == 0 {
        output += "Pling";
    }
    if n % 5 == 0 {
        output += "Plang";
    }
    if n % 7 == 0 {
        output += "Plong";
    }
    if output.len() == 0 {
        return n.to_string()
    }

    output
}
