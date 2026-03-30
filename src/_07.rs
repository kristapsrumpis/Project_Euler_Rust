// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6'th prime is 13.
// What is the 10001'st prime number?

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn nth_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut num = 1;
    while count < n {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}
