/// Returns the largest prime factor of a given number.
///
/// This function performs prime‑factorization using trial division.
/// It repeatedly divides the input number by the smallest possible
/// factor until the remaining value is itself prime.
///
/// # How it works
/// - Start with the smallest prime factor (`2`)
/// - While `factor * factor <= number`:
///     - If `factor` divides `number`, divide it out
///     - Otherwise, increment `factor`
/// - When the loop ends, the remaining `number` is the largest prime factor
///
/// # Example
/// ```
/// let result = largest_prime_factor(600851475143);
/// assert_eq!(result, 6857);
/// ```
///
/// # Notes
/// - This algorithm is efficient for numbers up to ~10¹⁴
/// - It avoids storing factors and only keeps dividing until the largest
///   prime factor remains
///
/// # Project Euler
/// This solves Project Euler Problem 3:
/// *“What is the largest prime factor of 600851475143?”*
///
/// # Arguments
/// * `number` – The integer to factorize
///
/// # Returns
/// The largest prime factor of the input number.

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
