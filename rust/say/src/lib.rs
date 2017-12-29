pub fn encode(num: u64) -> String {
    if num == 0 { return String::from("zero") }
    let (num, mut string) = big_numbers(num);
    string.push_str(small_numbers(num).as_str());
    string
}

fn big_numbers(mut num: u64) -> (u64, String) {
    let mut string = String::new();
    let orders = vec![
        (1_000_000_000_000_000_000, "quintillion"),
        (1_000_000_000_000_000, "quadrillion"),
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1_000, "thousand"),
        (100, "hundred"),
    ];

    for (limit, name) in orders {
        if num > limit - 1 {
            let (quotient, rest) = divmod(num, limit);
            num = rest;
            if quotient > 0 {
                string.push_str(encode(quotient).as_str());
                string.push(' ');
                string.push_str(name);
                if num > 0 { string.push(' ') }
            }
        }
    }

    (num, string)
}

fn small_numbers(num: u64) -> String {
    let ones = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = vec!["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = vec!["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let mut string = String::new();

    match num {
        1...9 => string.push_str(ones[num as usize - 1]),
        10...19 => string.push_str(teens[num as usize - 10]),
        20...99 => {
            let (quotient, rest) = divmod(num, 10);
            string.push_str(tens[quotient as usize - 2]);
            if rest != 0 {
                string.push('-');
                string.push_str(ones[rest as usize - 1]);
            }
        },
        _ => string.push_str("")
    }
    string
}

pub fn divmod(dividend: u64, divisor: u64) -> (u64, u64) {
    (dividend / divisor, dividend % divisor)
}
