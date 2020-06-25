pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper = upper_bound + 1;
    let mut numbers: Vec<u64> = (0..upper).map(u64::from).collect();
    let mut p = 2;
    for num in 0..upper {
        if num < 2 {
            numbers[num as usize] = 0;
            continue;
        }
       
        let mut j = 2*p;
        while j <= upper {
            
            if j < upper {
                numbers[j as usize] = 0;
            }
            j += p;
            
        }
        p = num;
        while ((p as usize) < numbers.len()) && numbers[p as usize] == 0 {
            p += 1;
        }
    }

    let mut v = Vec::new();
    for prime in &numbers {
        v.push(match prime {
            0 | 1 => None,
            _ => Some(prime.clone()),
        });
    };
    
    let v: Vec<u64> = v.into_iter().filter_map(|x| x).collect();
    
    v
}