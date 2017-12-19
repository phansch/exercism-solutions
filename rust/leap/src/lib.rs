pub fn is_leap_year(year: i32) -> bool {
    is_vanilla(year) && (!is_century(year) || is_exceptional(year))

}

fn is_vanilla(year: i32) -> bool {
    year % 4 == 0
}

fn is_century(year: i32) -> bool {
    year % 100 == 0
}

fn is_exceptional(year: i32) -> bool {
    year % 400 == 0
}
