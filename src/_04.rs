// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 times 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

pub fn largest_palindromic_number(min: u32, max: u32) -> u32 {
    let mut largest_palindrome = 0;
    for i in min..max {
        for n in min..max {
            let number = i * n;
            if number.to_string() == number.to_string().chars().rev().collect::<String>()
                && number > largest_palindrome
            {
                largest_palindrome = number
            }
        }
    }
    largest_palindrome
}
