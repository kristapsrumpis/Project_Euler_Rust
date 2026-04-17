/// Counts the total number of positive divisors of a given `u64` integer.
///
/// # Overview
/// This function implements an efficient `O(√n)` divisor‑pairing algorithm.
/// Instead of checking all numbers up to `n`, it only iterates up to
/// `sqrt(n)` and counts divisor pairs `(i, n/i)`.
///
/// # Behavior
/// - Returns `0` for input `0` (defined explicitly for practical use).
/// - For each `i` in `1..=sqrt(number)`:
///     - If `i` divides `number`, then:
///         - Adds **1** when `i * i == number` (perfect square)
///         - Adds **2** otherwise (`i` and its paired divisor)
///
/// # Examples
/// ```
/// assert_eq!(count_divisors(1), 1);   // {1}
/// assert_eq!(count_divisors(6), 4);   // {1,2,3,6}
/// assert_eq!(count_divisors(36), 9);  // perfect square
/// ```
///
/// # Complexity
/// - Time: `O(√n)`
/// - Space: `O(1)`

fn count_divisors(number: u64) -> u64 {
    if number == 0 {
        return 0;
    }

    let mut count = 0;

    for i in 1..=(number as f64).sqrt() as u64 {
        if number % i == 0 {
            if number / i == i {
                count += 1;
            } else {
                count += 2;
            }
        }
    }
    count
}

pub fn find_triangle_number_with_divisors_v2(limit: u64) -> u64 {
    let mut n = 1;

    loop {
        let (a, b) = if n % 2 == 0 {
            (n / 2, n + 1)
        } else {
            (n, (n + 1) / 2)
        };

        let divisors = count_divisors(a) * count_divisors(b);

        if divisors > limit {
            return n * (n + 1) / 2;
        }

        n += 1;
    }
}
