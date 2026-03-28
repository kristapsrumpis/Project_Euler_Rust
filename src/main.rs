use std::time::Instant; // import time module for script speed test
mod _01; // Euler problem 01
use _01::multiples_sum; //declears wich function use from  mod file
mod _02;
use _02::even_fib_sum;
mod _03;
use _03::largest_prime_factor;

fn main() {
    let start_01 = Instant::now(); // set start point for timer for Euler problem 01

    // let sum_of_10 = multiples_sum(10); // run script for test multiple bellow 10
    // println!("Sum of multibles below 10: {}", sum_of_10);

    let sum_of_1000 = multiples_sum(1000); // Run script to get sum of multiples below 1000
    println!("Euler problem 01: {}", sum_of_1000);

    // gets and print in console script duration time for euler problem 01
    let duration_01 = start_01.elapsed();
    println!("Time: {:?}", duration_01); // print in terminal script run time

    let start_02 = Instant::now();

    // let sum_of_10 = even_fib_sum(10);
    // println!("Even Fibonachi sum of 10: {}", sum_of_10);

    let sum_of_4_000_000 = even_fib_sum(4_000_000);
    println!("Even Fibonachi sum of 4_000_000: {}: ", sum_of_4_000_000);

    let duration_02 = start_02.elapsed();
    println!("Time: {:?}", duration_02);

    let start_03 = Instant::now();

    let n = 600851475143;
    println!(
        "Largest prim factore from {} is : {}",
        n,
        largest_prime_factor(n)
    );

    let duration_03 = start_03.elapsed();
    println!("Time: {:?}", duration_03)
}
