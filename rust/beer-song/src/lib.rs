pub fn verse(n: i32) -> String {
    let next_bottle = (n - 1) % 100;
    let current = pluralize("bottle", n);
    let next = pluralize("bottle", next_bottle);
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else {
        format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", current, current, subject(n), next)
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut songs: Vec<String> = vec![];
    for x in (end..start + 1).rev() {
        songs.push(verse(x))
    }
    songs.join("\n")
}

fn pluralize(input: &str, n: i32) -> String {
    if n > 1 {
        format!("{} {}s", n, input)
    } else if n == 0 {
        format!("no more bottles")
    } else {
        format!("{} {}", n, input)
    }
}

fn subject(n: i32) -> String {
    if n == 1 {
        String::from("it")
    } else {
        String::from("one")
    }
}
