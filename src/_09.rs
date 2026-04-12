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
