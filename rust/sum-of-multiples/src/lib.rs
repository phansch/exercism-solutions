pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = vec![];

    for num in 1..limit {
        for factor in factors {
            if num % factor == 0 && !multiples.iter().any(|&m| m == num) {
                multiples.push(num);
            }
        }
    }

    multiples.iter().sum()
}
