pub mod qpow {
    pub fn qpow(mut x: i64, mut y: i64, p: i64) -> i64 {
        if y == 0 {
            return 1;
        }
        let mut ans = 1;
        while y != 0 {
            if (y & 1) == 1 {
                ans = ans * x % p;
            }
            x = x * x % p;
            y >>= 1;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::qpow::*;

    #[test]
    fn test_qpow() {
        // Test case 1
        let result = qpow(2, 3, 5);
        assert_eq!(result, 3); // Expected: 2^3 % 5 = 3

        // Test case 2
        let result = qpow(5, 0, 10);
        assert_eq!(result, 1); // Expected: 5^0 % 10 = 1

        // Add more test cases here...
    }
}
