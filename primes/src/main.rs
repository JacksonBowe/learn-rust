

fn main() {
    let n: i32 = 100;
    println!("Running with n = {}", n);
    
    // Find all primes of n
    let p = primes(n);

    // How many primes
    println!("Number of primes: {}", p.len());

    // Largest prime
    println!("Largest prime: {:?}", p.iter().max().unwrap());

    // Primes
    println!("Primes: {:?}", p)

}

fn primes(n: i32) -> Vec<i32> {
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
