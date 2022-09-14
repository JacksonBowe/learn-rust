

pub fn quotient(n: i32) -> Vec<i32> {
    // Print n
    let mut found: Vec<i32> = Vec::new();

    for i in 2..n+1  {
        let check: Vec<i32> = found.iter().map(|x| i % x).collect();

        if check.contains(&0) {
            continue;
        }
        // println!("found prime: {}", i);
        found.push(i);
    }

    // println!("Primes: {:?}", found);
    return found
}

pub fn seive(n: i32) -> Vec<i32> {
    // Initialise a vector of len n with all true values
    let mut start: Vec<bool> = vec![true; n as usize];

    for i in 2..n+1 {

        // Start at i^2, break if out of bounds
        if i * i > n { 
            break;
        }

        // If the value at start[i] is true check if prime
        if start[i as usize] {
            let mut j = 0;
            // check all multiples of start[i] starting from i^2
            while (i*i + j*i) < n {
                start[(i*i + j*i) as usize] = false;
                j += 1;
            }
        }
    }

    let mut found: Vec<i32> = Vec::new();
    for (idx, e) in start.iter().enumerate() {
        if idx < 2 { continue }
        if *e == true {
            found.push(idx as i32)
        }
    }

    return found
}