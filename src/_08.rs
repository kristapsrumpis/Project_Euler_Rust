/// Computes the largest product of any contiguous substring of digits within a
/// numeric string.
///
/// # Parameters
///
/// - `s`: A string slice containing only digit characters (`'0'`–`'9'`).
/// - `i`: The size of the sliding window used to compute each product.
///
/// # Returns
///
/// The maximum product of `i` consecutive digits found in the input string.
/// If the string is shorter than `i`, the function returns `0`.
///
/// # Behavior
///
/// - Non‑digit characters are ignored.
/// - The function converts each character to a `u64`, then iterates over all
///   windows of length `i`, computing the product of each window.
/// - The largest product encountered is returned.
///
/// # Examples
///
/// ```rust
/// let digits = "73167176531330624919225119674426574742355349194934";
/// let result = largest_product_in_a_series(digits, 6);
/// assert_eq!(result, 23520);
/// ```
///
/// ```rust
/// // Window larger than the string
/// assert_eq!(largest_product_in_a_series("1234", 10), 0);
/// ```

pub fn largest_product_in_a_series(s: &str, i: usize) -> u64 {
    let mut result = 0;
    let nums: Vec<u64> = s
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
        .collect();
    for chunk in nums.windows(i) {
        let product = chunk.iter().product::<u64>();
        if product > result {
            result = product;
        }
    }
    result
}
