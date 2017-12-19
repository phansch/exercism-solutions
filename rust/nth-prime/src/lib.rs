pub fn nth(num: i32) -> Result<i32, &'static str> {
    if num == 0 {
        return Err("There is no 0th prime")
    }

    let mut primes = vec![2, 3];
    let mut candidate = primes.last().unwrap().clone();

    while primes.len() < num as usize {
        candidate += 2;

        let has_candidate = primes.iter().cloned().any(|prime| candidate % prime == 0);

        if !has_candidate {
            primes.push(candidate);
        }
    }

    Ok(primes[(num - 1) as usize])
}
