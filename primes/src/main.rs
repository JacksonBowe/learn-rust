use std::time::Instant;

mod primes;

fn main() {
    // let n: i32 = 100;
    for n in [10, 100, 1000, 10000, 100000] {
        println!("Running with n = {}", n);
        
        // Find all primes of n
        // let p = primes::quotient(n);
        
        // // How many primes
        // println!("Number of primes: {}", p.len());

        // // Largest prime
        // println!("Largest prime: {:?}", p.iter().max().unwrap());

        // // Primes
        // println!("Primes: {:?}", p)

        let tick_1 = Instant::now();
        primes::quotient(n);
        let toc_1 = tick_1.elapsed();

        let tick_2 = Instant::now();
        primes::seive(n);
        let toc_2 = tick_2.elapsed();

        println!("Time to run quotient: {:.2?}", toc_1);
        println!("Time to run seive: {:.2?}", toc_2);
        println!();
    }

}
