// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn smallest_multiple(a: u64, b: u64) -> u64 {
    let mut result = 1;
    for n in a..b {
        result = lcm(result, n);
    }
    result
}
