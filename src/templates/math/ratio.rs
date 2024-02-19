pub mod ratio {
    use std::ops::*;
    #[derive(Copy, Clone, PartialEq, Eq, Ord, Debug)]
    pub struct Ratio {
        num: i64,
        den: i64,
    }
    pub fn gcd(x: i64, y: i64) -> i64 {
        match y {
            0 => x,
            y => gcd(y, x % y),
        }
    }
    pub fn simplify(x: i64, y: i64) -> (i64, i64) {
        match x {
            0 => (0, 1),
            x => {
                let g = gcd(x, y);
                (x / g, y / g)
            }
        }
    }
    impl Ratio {
        pub fn new(num: i64, den: i64) -> Self {
            assert!(den != 0.into());
            let (num, den) = simplify(num, den);
            if den < 0.into() {
                Ratio {
                    num: -num,
                    den: -den,
                }
            } else {
                Ratio { num, den }
            }
        }
    }
    impl Default for Ratio {
        fn default() -> Self {
            Self { num: 0, den: 1 }
        }
    }
    impl Add for Ratio {
        type Output = Ratio;

        fn add(self, rhs: Self) -> Self::Output {
            Ratio::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
        }
    }
    impl AddAssign for Ratio {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Sub for Ratio {
        type Output = Ratio;

        fn sub(self, rhs: Self) -> Self::Output {
            Ratio::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
        }
    }
    impl SubAssign for Ratio {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
    impl Mul for Ratio {
        type Output = Ratio;

        fn mul(self, rhs: Self) -> Self::Output {
            Ratio::new(self.num * rhs.num, self.den * rhs.den)
        }
    }
    impl MulAssign for Ratio {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }
    impl Div for Ratio {
        type Output = Ratio;

        fn div(self, rhs: Self) -> Self::Output {
            assert!(rhs.num != 0.into());
            Ratio::new(self.num * rhs.den, self.den * rhs.num)
        }
    }
    impl DivAssign for Ratio {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }
    impl Neg for Ratio {
        type Output = Ratio;
        fn neg(self) -> Self::Output {
            Self::Output {
                num: -self.num,
                den: self.den,
            }
        }
    }
    impl<T: Into<i64>> From<T> for Ratio {
        fn from(value: T) -> Self {
            Self::new(value.into(), 1)
        }
    }
    impl PartialOrd for Ratio {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            (self.num * other.den).partial_cmp(&(other.num * self.den))
        }
    }
    impl std::fmt::Display for Ratio {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ratio() {
        type Ratio64 = super::ratio::Ratio;
        let x = Ratio64::new(3, 4);
        let y = Ratio64::new(4, 5);
        assert_eq!(x + y, Ratio64::new(31, 20));
        assert_eq!(x - y, Ratio64::new(-1, 20));
        assert_eq!(x * y, Ratio64::new(3, 5));
        assert_eq!(x + 2.into(), Ratio64::new(11, 4));
        assert_eq!(x / y, Ratio64::new(15, 16));
        assert_eq!(x / 2.into(), Ratio64::new(3, 8));
        assert_eq!(-x, Ratio64::new(-3, 4));
        assert!(x < y);
    }
}
