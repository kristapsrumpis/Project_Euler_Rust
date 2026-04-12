/// Computes the sum of all prime numbers below a given upper bound.
///
/// This function iterates through the range `1..number` and uses the
/// [`is_prime`] helper function (imported from module `_07`) to test
/// each value for primality. Every prime encountered is added to an
/// accumulating total, which is returned at the end.
///
/// # Parameters
/// - `number`: The exclusive upper limit. All primes strictly less
///   than this value are included in the sum.
///
/// # Returns
/// A `u64` representing the total sum of all prime numbers below `number`.
///
/// # Algorithm
/// - Start with an accumulator set to zero.
/// - Loop from `1` up to (but not including) `number`.
/// - For each value `i`, check if it is prime using [`is_prime`].
/// - If it is prime, add it to the accumulator.
/// - Return the final accumulated sum.
///
/// # Complexity
/// - Time complexity depends on the implementation of [`is_prime`].
///   For a simple trial‑division prime check, the overall complexity
///   is approximately *O(n √n)*.
/// - Space complexity is *O(1)*.
///
/// # Examples
/// ```
/// use crate::_10::sum_of_primes;
///
/// assert_eq!(sum_of_primes(10), 17); // primes: 2 + 3 + 5 + 7
/// ```
use crate::_07::is_prime;

pub fn sum_of_primes(number: u64) -> u64 {
    let mut answer = 0;
    for i in 1..number {
        if is_prime(i) {
            answer += i;
        }
    }
    answer
}
