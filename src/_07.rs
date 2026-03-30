/// Determines whether a given number is prime.
///
/// This function uses an optimized trial‑division algorithm:
/// - It rejects numbers ≤ 1
/// - It accepts 2 and 3 as primes
/// - It eliminates multiples of 2 and 3 early
/// - It then checks potential factors of the form `6k ± 2`
///
/// This significantly reduces the number of iterations compared to
/// checking every integer.
///
/// # Example
/// ```
/// assert!(is_prime(13));
/// assert!(!is_prime(100));
/// ```
///
/// # Arguments
/// * `n` – The number to test for primality
///
/// # Returns
/// `true` if `n` is prime, otherwise `false`.

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Returns the `n`‑th prime number (1‑indexed).
///
/// This function iterates through natural numbers, counting primes
/// until it reaches the `n`‑th one. It uses [`is_prime`] to test
/// each candidate.
///
/// # How it works
/// - Start from `num = 1`
/// - Increment `num` and test if it is prime
/// - Each time a prime is found, increment `count`
/// - Stop when `count == n`
///
/// # Example
/// ```
/// // The 6th prime is 13
/// assert_eq!(nth_prime(6), 13);
///
/// // Project Euler Problem 7:
/// // The 10001st prime is 104743
/// assert_eq!(nth_prime(10001), 104743);
/// ```
///
/// # Notes
/// - This is a straightforward algorithm suitable for moderate values of `n`
/// - For very large `n`, a sieve‑based approach is more efficient
///
/// # Arguments
/// * `n` – The index of the prime to retrieve (1‑indexed)
///
/// # Returns
/// The `n`‑th prime number.

pub fn nth_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut num = 1;
    while count < n {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}
