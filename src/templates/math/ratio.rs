pub mod ratio {
    use std::ops::*;
    pub trait PrimitiveOps<T>:
        Add<Output = T>
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Neg<Output = T>
        + Rem<Output = T>
        + RemAssign
        + Copy
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Default
        + From<i64>
        + std::fmt::Display
    {
    }
    impl PrimitiveOps<i64> for i64 {}
    impl PrimitiveOps<i128> for i128 {}

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
    pub struct Ratio<T> {
        num: T,
        den: T,
    }

    pub fn gcd<T: PrimitiveOps<T>>(x: T, y: T) -> T {
        if y == 0.into() {
            x
        } else {
            gcd(y, x % y)
        }
    }
    pub fn simplify<T: PrimitiveOps<T>>(x: T, y: T) -> (T, T) {
        if x == 0.into() {
            return (0.into(), 1.into());
        }
        let g = gcd(x, y);
        (x / g, y / g)
    }
    impl<T: PrimitiveOps<T>> Ratio<T> {
        pub fn new(num: T, den: T) -> Self {
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
    impl<T: PrimitiveOps<T>> Add for Ratio<T> {
        type Output = Ratio<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Ratio::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
        }
    }
    impl<T: PrimitiveOps<T>> AddAssign for Ratio<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl<T: PrimitiveOps<T>> Sub for Ratio<T> {
        type Output = Ratio<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Ratio::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
        }
    }
    impl<T: PrimitiveOps<T>> SubAssign for Ratio<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
    impl<T: PrimitiveOps<T>> Mul for Ratio<T> {
        type Output = Ratio<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            Ratio::new(self.num * rhs.num, self.den * rhs.den)
        }
    }
    impl<T: PrimitiveOps<T>> MulAssign for Ratio<T> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }
    impl<T: PrimitiveOps<T>> Div for Ratio<T> {
        type Output = Ratio<T>;

        fn div(self, rhs: Self) -> Self::Output {
            assert!(rhs.num != 0.into());
            Ratio::new(self.num * rhs.den, self.den * rhs.num)
        }
    }
    impl<T: PrimitiveOps<T>> DivAssign for Ratio<T> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }
    impl<T: PrimitiveOps<T>> Neg for Ratio<T> {
        type Output = Ratio<T>;
        fn neg(self) -> Self::Output {
            Self::Output {
                num: -self.num,
                den: self.den,
            }
        }
    }
    impl<T: PrimitiveOps<T>> From<T> for Ratio<T> {
        fn from(value: T) -> Self {
            Self::new(value, 1.into())
        }
    }
    impl<T: PrimitiveOps<T>> std::fmt::Display for Ratio<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ratio() {
        type Ratio64 = super::ratio::Ratio<i64>;
        let x = Ratio64::new(3, 4);
        let y = Ratio64::new(4, 5);
        assert_eq!(x + y, Ratio64::new(31, 20));
        assert_eq!(x - y, Ratio64::new(-1, 20));
        assert_eq!(x * y, Ratio64::new(3, 5));
        assert_eq!(x + 2.into(), Ratio64::new(11, 4));
        assert_eq!(x / y, Ratio64::new(15, 16));
        assert_eq!(x / 2.into(), Ratio64::new(3, 8));
        assert_eq!(-x, Ratio64::new(-3, 4));
    }
}
