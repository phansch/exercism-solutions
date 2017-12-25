pub fn find() -> Option<u32> {
    let mut c: u32;
    let sum = 1000;

    // Because a < b < c we can also say that a < sum / 3:
    // Because if a was bigger than 333.33, there would be no way for b and c to be greater than a.
    // The same can be said for b < sum / 2:
    // If b was bigger than 500, there would be no way for c to be bigger than b.
    for a in 1..(sum / 3) {
        for b in a..(sum / 2) {
            c = sum - a - b;

            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    None
}
