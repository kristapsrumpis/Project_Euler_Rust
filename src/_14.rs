/// Computes the length of the Collatz sequence starting from a given number.
///
/// The Collatz sequence is defined as:
/// - If `n` is even, the next term is `n / 2`.
/// - If `n` is odd, the next term is `3 * n + 1`.
/// The sequence always terminates at `1` for all tested positive integers.
///
/// # Parameters
/// - `number`: The starting positive integer for which the Collatz sequence
///   length will be computed.
///
/// # Returns
/// The total number of terms in the sequence, including the starting number
/// and the final `1`.
///
/// # Examples
/// ```
/// let len = collatz_sequence_length(13);
/// assert_eq!(len, 10); // 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
/// ```
///
/// # Complexity
/// Worst‑case time complexity is proportional to the number of steps in the
/// Collatz sequence for the given input, which varies unpredictably.

fn collatz_sequence_length(mut number: u64) -> u64 {
    let mut length = 1;
    while number != 1 {
        number = if number % 2 == 0 {
            number / 2
        } else {
            3 * number + 1
        };
        length += 1;
    }
    length
}

pub fn longest_collatz_sequence(limit: u64) -> u64 {
    let mut longest_chain = 0;
    let mut starting_number = 0;

    for i in 1..=limit {
        let chain_lenght = collatz_sequence_length(i);
        if chain_lenght > longest_chain {
            longest_chain = chain_lenght;
            starting_number = i;
        }
    }
    starting_number
}
