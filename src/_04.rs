/// Computes the largest palindromic number formed by the product of two
/// integers within the given range.
///
/// A palindromic number reads the same forwards and backwards.
/// This function searches all products `i * n` where both `i` and `n`
/// fall within the range `[min, max)`, and returns the largest product
/// that is a palindrome.
///
/// # How it works
/// - Iterate through all pairs `(i, n)` in the range `min..max`
/// - Compute their product
/// - Convert the product to a string and compare it with its reverse
/// - Track the largest palindromic product found
///
/// # Example
/// ```
/// // The largest palindrome from two 3‑digit numbers is 906609 (993 × 913)
/// let result = largest_palindromic_number(100, 1000);
/// assert_eq!(result, 906609);
/// ```
///
/// # Notes
/// - This solves Project Euler Problem 4.
/// - The algorithm is brute‑force and runs in O(n²) time.
/// - For 3‑digit ranges, performance is acceptable without optimization.
///
/// # Arguments
/// * `min` – The inclusive lower bound of the multiplicand range
/// * `max` – The exclusive upper bound of the multiplicand range
///
/// # Returns
/// The largest palindromic product found within the range.

pub fn largest_palindromic_number(min: u32, max: u32) -> u32 {
    let mut largest_palindrome = 0;
    for i in min..max {
        for n in min..max {
            let number = i * n;
            if number.to_string() == number.to_string().chars().rev().collect::<String>()
                && number > largest_palindrome
            {
                largest_palindrome = number
            }
        }
    }
    largest_palindrome
}
