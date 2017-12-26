pub fn factors(num: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut num = num.clone();
    let mut divisor = 2;

    while num > 1 {
        if num % divisor == 0 {
            num = num / divisor;
            result.push(divisor);
        }
        else {
            divisor += 1;
        }
    }

    result
}
