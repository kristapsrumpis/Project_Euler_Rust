/// Computes the greatest common divisor (GCD) of two numbers using the
/// Euclidean algorithm.
///
/// This function repeatedly applies the rule:
/// `gcd(a, b) = gcd(b, a % b)`
/// until `b` becomes zero. The remaining value of `a` is the GCD.
///
/// # Example
/// ```
/// assert_eq!(gcd(54, 24), 6);
/// ```

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp
    }
    a
}

/// Computes the least common multiple (LCM) of two numbers.
///
/// The LCM is calculated using the relationship:
/// `lcm(a, b) = a / gcd(a, b) * b`
///
/// # Example
/// ```
/// assert_eq!(lcm(4, 6), 12);
/// ```

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// Returns the smallest positive number that is evenly divisible by all
/// integers in the range `[a, b)`.
///
/// This function computes the least common multiple (LCM) of every number
/// in the given range. The result is the smallest number divisible by all
/// values from `a` up to (but not including) `b`.
///
/// # How it works
/// - Start with `result = 1`
/// - For each number `n` in the range:
///     - Update `result = lcm(result, n)`
/// - The final `result` is divisible by every number in the range
///
/// # Example
/// ```
/// // Solves Project Euler Problem 5:
/// // The smallest number divisible by all numbers from 1 to 20 is 232792560.
/// let result = smallest_multiple(1, 21);
/// assert_eq!(result, 232792560);
/// ```
///
/// # Notes
/// - The upper bound `b` is **exclusive**, so use `21` to include `20`.
/// - This approach is efficient and avoids brute‑force searching.
///
/// # Arguments
/// * `a` – Inclusive lower bound of the range
/// * `b` – Exclusive upper bound of the range
///
/// # Returns
/// The smallest number evenly divisible by all integers in the range.

pub fn smallest_multiple(a: u64, b: u64) -> u64 {
    let mut result = 1;
    for n in a..b {
        result = lcm(result, n);
    }
    result
}
