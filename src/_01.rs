/// Computes the sum of all natural numbers below a given limit
/// that are divisible by **3** or **5**.
///
/// # Arguments
/// * `range` — The upper bound (exclusive). All integers from `1` up to
///   `range - 1` are checked.
///
/// # Returns
/// The total sum of all integers less than `range` that are multiples of
/// `3` or `5`.
///
/// # Examples
/// ```
/// let result = multiples_sum(10);
/// assert_eq!(result, 23); // 3 + 5 + 6 + 9
/// ```
///
/// # Notes
/// This implementation first collects all matching values into a vector
/// and then sums them. It is correct but not the most memory‑efficient
/// approach; the logic can be expressed more idiomatically using iterators.

pub fn multiples_sum(range: i32) -> i32 {
    let mut multiples: Vec<i32> = Vec::new();
    let mut result = 0;
    for i in 1..=range - 1 {
        if i % 3 == 0 || i % 5 == 0 {
            multiples.push(i);
        }
    }
    for i in &multiples {
        result += i
    }
    result
}
