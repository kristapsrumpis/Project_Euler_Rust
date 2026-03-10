// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

// Reusable function to get sum of multiples
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
