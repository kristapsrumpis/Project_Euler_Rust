use project_euler_rust::_01::multiples_sum;
use project_euler_rust::_02::even_fib_sum;
use project_euler_rust::_03::largest_prime_factor;
use project_euler_rust::_04::largest_palindromic_number;
use project_euler_rust::_05::smallest_multiple;
use project_euler_rust::_06::sum_squere_difference;
use project_euler_rust::_07::nth_prime;
use project_euler_rust::_08::largest_product_in_a_series;
use project_euler_rust::_09::product;
use project_euler_rust::_10::sum_of_primes;
use project_euler_rust::_11::largest_product_in_grid;
use project_euler_rust::_12::find_triangle_number_with_divisors_v2;
use project_euler_rust::_13::large_sum;
use std::time::Instant; // import time module for script speed test

fn main() {
    let start_01 = Instant::now(); // set start point for timer for Euler problem 01

    let sum_of_1000 = multiples_sum(1000); // Run script to get sum of multiples below 1000
    println!("Euler problem 01: {}", sum_of_1000);

    let duration_01 = start_01.elapsed();
    println!("Time: {:?}", duration_01);

    let start_02 = Instant::now();

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

    let start_06 = Instant::now();

    let number = 100;
    println!(
        "Sum Square Difference of {}: {}",
        &number,
        sum_squere_difference(number)
    );

    let duration_06 = start_06.elapsed();
    println!("Time: {:?}", duration_06);

    let start_07 = Instant::now();

    let n = 10001;
    println!("{}st prime is: {}", n, nth_prime(n));

    let duration_07 = start_07.elapsed();
    println!("Time: {:?}", duration_07);

    let start_08 = Instant::now();

    let number: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let adjacent: usize = 13;
    println!(
        "Largest Product in a Series: {}",
        largest_product_in_a_series(number, adjacent)
    );

    let duration_08 = start_08.elapsed();
    println!("Time: {:?}", duration_08);

    let start_09 = Instant::now();

    let number = 1000;
    println!("Product: {}", product(number));

    let duration_09 = start_09.elapsed();
    println!("Time: {:?}", duration_09);

    let start_10 = Instant::now();

    let limit = 2000000;
    println!(
        "Sum of Primes of limit {} is : {}",
        limit,
        sum_of_primes(limit)
    );

    let duration_10 = start_10.elapsed();
    println!("Time: {:?}", duration_10);

    let start_11 = Instant::now();

    println!(
        "Largest product in 20x20 grid: {}",
        largest_product_in_grid()
    );

    let duration_11 = start_11.elapsed();
    println!("Time: {:?}", duration_11);

    let start_12 = Instant::now();

    let limit: u64 = 500;

    println!(
        "Highly Divisible Triangular Number of {} : {}",
        limit,
        find_triangle_number_with_divisors_v2(limit)
    );

    let duration_12 = start_12.elapsed();
    println!("Time: {:?}", duration_12);

    let start_13 = Instant::now();

    println!("Large Summ: {}", large_sum());

    let duration_13 = start_13.elapsed();
    println!("Time: {:?}", duration_13);
}
