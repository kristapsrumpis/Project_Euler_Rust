use std::time::Instant; // import time module for script speed test
mod _01;
use _01::multiples_sum;
mod _02;
use _02::even_fib_sum;
mod _03;
use _03::largest_prime_factor;
mod _04;
use _04::largest_palindromic_number;
mod _05;
use _05::smallest_multiple;

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
    println!("Time: {:?}", duration_03);

    let start_04 = Instant::now();

    let min_number = 100;
    let max_number = 1000;
    println!(
        "The largest palindrome made from 3-digit numbers: {}",
        largest_palindromic_number(min_number, max_number)
    );

    let duration_04 = start_04.elapsed();
    println!("Time: {:?}", duration_04);

    let start_05 = Instant::now();

    let a = 1;
    let b = 20;
    println!("Smallest Multiple: {}", smallest_multiple(a, b));

    let duration_05 = start_05.elapsed();
    println!("Time: {:?}", duration_05);
}
