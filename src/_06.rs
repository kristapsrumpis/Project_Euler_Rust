/// Computes the difference between:
/// 1. The sum of the squares of the first `num` natural numbers, and
/// 2. The square of the sum of the first `num` natural numbers.
///
/// This corresponds to Project Euler Problem 6.
///
/// # Mathematical Background
/// For a given `num`, this function evaluates:
///
/// - **Sum of squares:**
///   `1² + 2² + ... + num²`
///
/// - **Square of the sum:**
///   `(1 + 2 + ... + num)²`
///
/// It then returns:
///
/// `square_of_the_sum - sum_of_the_squares`
///
/// # Example
/// ```
/// // For the first 10 natural numbers:
/// // Sum of squares = 385
/// // Square of the sum = 3025
/// // Difference = 2640
/// let result = sum_squere_difference(10);
/// assert_eq!(result, 2640);
/// ```
///
/// # Notes
/// - Runs in O(n) time using a simple loop.
/// - `num` is inclusive: passing `100` computes the result for 1 through 100.
/// - This implementation uses integer arithmetic and avoids floating‑point operations.
///
/// # Arguments
/// * `num` – The upper bound of the natural number range (inclusive)
///
/// # Returns
/// The difference between the square of the sum and the sum of the squares.

pub fn sum_squere_difference(num: u64) -> u64 {
    let mut the_sum_of_squers = 0;
    let mut squere_of_the_sum = 0;
    for i in 1..num + 1 {
        the_sum_of_squers += i * i;
        squere_of_the_sum += i;
    }
    (squere_of_the_sum * squere_of_the_sum) - the_sum_of_squers
}
