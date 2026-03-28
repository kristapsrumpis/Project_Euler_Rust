// What is the largest prime factor of the number 600851475143
// The prime factors of 13195 are 5, 7, 13 and 29.

pub fn largest_prime_factor(mut number: u64) -> u64 {
    let mut factor = 2;
    while factor * factor <= number {
        if number % factor == 0 {
            number /= factor;
        } else {
            factor += 1;
        }
    }
    number
}
