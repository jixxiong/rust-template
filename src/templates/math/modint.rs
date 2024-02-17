pub mod modint {
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
    #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
    pub struct ModInt<const P: i64>(i64);
    impl<const P: i64> ModInt<P> {
        pub fn new<T>(value: T) -> Self
        where
            T: Into<i64>,
        {
            let result = value.into() % P;
            ModInt(if result < 0 { result + P } else { result })
        }
        pub fn inv(self) -> Self {
            ModInt(qpow(self.0, P - 2, P))
        }
        pub fn pow<T>(self, y: T) -> Self
        where
            T: Into<i64>,
        {
            ModInt(qpow(self.0, y.into(), P))
        }
    }
    impl<const P: i64> std::fmt::Display for ModInt<P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<const P: i64> std::ops::Add for ModInt<P> {
        type Output = ModInt<P>;

        fn add(self, rhs: Self) -> Self::Output {
            let result = self.0 + rhs.0;
            ModInt(if result > P { result - P } else { result })
        }
    }
    impl<const P: i64> std::ops::AddAssign for ModInt<P> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl<const P: i64> std::ops::Sub for ModInt<P> {
        type Output = ModInt<P>;

        fn sub(self, rhs: Self) -> Self::Output {
            let result = self.0 - rhs.0;
            ModInt(if result < 0 { result + P } else { result })
        }
    }
    impl<const P: i64> std::ops::SubAssign for ModInt<P> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
    impl<const P: i64> std::ops::Neg for ModInt<P> {
        type Output = ModInt<P>;

        fn neg(self) -> Self::Output {
            ModInt(P - self.0)
        }
    }
    impl<const P: i64> std::ops::Mul for ModInt<P> {
        type Output = ModInt<P>;

        fn mul(self, rhs: Self) -> Self::Output {
            ModInt(self.0 * rhs.0 % P)
        }
    }
    impl<const P: i64> std::ops::MulAssign for ModInt<P> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }
    impl<const P: i64> Into<i64> for ModInt<P> {
        fn into(self) -> i64 {
            self.0
        }
    }
    impl<const P: i64> Into<i32> for ModInt<P> {
        fn into(self) -> i32 {
            self.0 as i32
        }
    }
    impl<const P: i64> From<i64> for ModInt<P> {
        fn from(value: i64) -> Self {
            ModInt::new(value)
        }
    }
    impl<const P: i64> From<i32> for ModInt<P> {
        fn from(value: i32) -> Self {
            ModInt::new(value as i64)
        }
    }
    #[allow(non_camel_case_types)]
    pub type mint998244353 = ModInt<998244353>;
    #[allow(non_camel_case_types)]
    pub type mint1000000007 = ModInt<1000000007>;
}

#[cfg(test)]
mod tests {
    use super::modint::*;

    #[test]
    fn test_qpow() {
        #[allow(non_camel_case_types)]
        type mint = ModInt<31>;
        let mut x: mint = 10.into();
        let y: mint = 12.into();
        assert_eq!(x + y, 22.into());
        assert_eq!(x * y, 27.into());
        assert_eq!(x - y, 29.into());
        assert_eq!(x.inv(), 28.into());
        x += y;
        assert_eq!(x, 22.into());
    }
}
