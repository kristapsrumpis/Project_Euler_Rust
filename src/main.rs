use std::time::Instant; // import time module for script speed test
mod _01; // Euler problem 01
use _01::multiples_sum; //declears wich function use from  mod file

fn main() {
    let start_01 = Instant::now(); // set start point for timer for Euler problem 01

    // let sum_of_10 = multiples_sum(10); // run script for test multiple bellow 10
    // println!("Sum of multibles below 10: {}", sum_of_10);

    let sum_of_1000 = multiples_sum(1000); // Run script to get sum of multiples below 1000
    println!("Euler problem 01: {}", sum_of_1000);

    // gets and print in console script duration time for euler problem 01
    let duration_01 = start_01.elapsed();
    println!("Time: {:?}", duration_01); // print in terminal script run time
}
