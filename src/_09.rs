/// Searches for a Pythagorean triple `(a, b, c)` such that:
///
/// - `a + b + c = sum_total`
/// - `a² + b² = c²`
///
/// This function performs a brute‑force search over possible values of `a` and `b`.
/// For each pair, it computes `c` as the remaining value needed to reach `sum_total`.
///
/// # Arguments
/// * `sum_total` – The total sum that the triple `(a, b, c)` must add up to.
///
/// # Returns
/// * `Some((a, b, c))` if a valid Pythagorean triple is found.
/// * `None` if no such triple exists.
///
/// # Example
/// ```
/// let triple = find_pythagorean_triple(1000);
/// assert_eq!(triple, Some((200, 375, 425)));
/// ```

fn find_pythagorean_triple(sum_total: u64) -> Option<(u64, u64, u64)> {
    for a in 1..sum_total / 2 {
        for b in 1..sum_total / 2 {
            let c = sum_total - a - b;
            if a * a + b * b == c * c {
                return Some((a, b, c));
            }
        }
    }
    None
}

pub fn product(number: u64) -> u64 {
    if let Some((a, b, c)) = find_pythagorean_triple(number) {
        a * b * c
    } else {
        0
    }
}
