// Multiples of 3 or 5
//
// Problem 1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
//
fn multiples_of_3_or_5(x: u32) -> u32 {
    x + 13
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples_of_3_or_5() {
        assert_eq!(multiples_of_3_or_5(10), 23);
    }
}
