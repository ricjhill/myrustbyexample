/*

In mathematics, the Sieve of Eratosthenes is a simple, ancient algorithm for
finding all prime numbers up to any given limit.
To find all the prime numbers less than or equal to a given integer n by Eratosthenes' method:

    Create a list of consecutive integers from 2 through n: (2, 3, 4, ..., n).
    Initially, let p equal 2, the smallest prime number.
    Enumerate the multiples of p by counting in increments of p from 2p to n,
    mark them in the list (these will be 2p, 3p, 4p, ...; the p itself should not be marked).
    Find the first number greater than p in the list that is not marked.
    If there was no such number, stop.
    Otherwise, let p now equal this new number (which is the next prime),
    and repeat from step 3.
    When the algorithm terminates,
    the numbers remaining not marked in the list are all the primes below n.

*/


extern crate primal;
use primal::Sieve;

fn numofdivisors(n: usize, primes: &Sieve ) -> Option<usize> {
    match primes.factor(n) {
        Ok(factors) => Some(factors.into.iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}

fn main(){
    println!("24 days of Rust - primal (day 2)");
    let  sieve = Sieve::new(10000);
    let suspect = 5273;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));
    let not_a_prime = 1024;
    let  n = 1000;
    match sieve.primes_from(0).nth(n-1){
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I don't know anything about {}th prime.", n),
    }
    println!("{:?}", sieve.factor(2610));
    println!("{:?}", numofdivisors(2610, &sieve));
}
