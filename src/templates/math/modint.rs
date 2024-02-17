pub mod modint {
    pub fn qpow(mut x: i32, mut y: i32, p: i32) -> i32 {
        if y == 0 {
            return 1;
        }
        let mut ans: i32 = 1;
        while y != 0 {
            if (y & 1) == 1 {
                ans = (ans as i64 * x as i64 % p as i64) as i32;
            }
            x = (x as i64 * x as i64 % p as i64) as i32;
            y >>= 1;
        }
        return ans;
    }
    #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
    pub struct ModInt<const P: i32>(i32);
    impl<const P: i32> ModInt<P> {
        pub fn new<T>(value: T) -> Self
        where
            T: std::convert::TryInto<i32> + std::ops::Rem<Output = T> + From<i32> + PartialOrd,
        {
            if value >= 0.into() && value < P.into() {
                return ModInt(value.try_into().ok().unwrap());
            }
            let result = (value % P.into()).try_into().ok().unwrap();
            ModInt(if result < 0 { result + P } else { result })
        }
        pub fn inv(self) -> Self {
            ModInt(qpow(self.0, P - 2, P))
        }
        pub fn pow(self, y: Self) -> Self {
            ModInt(qpow(self.0, y.0, P))
        }
    }
    impl<const P: i32> std::fmt::Display for ModInt<P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<const P: i32> std::ops::Add for ModInt<P> {
        type Output = ModInt<P>;

        fn add(self, rhs: Self) -> Self::Output {
            let result = self.0 + rhs.0;
            ModInt(if result >= P { result - P } else { result })
        }
    }
    impl<const P: i32> std::ops::AddAssign for ModInt<P> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl<const P: i32> std::ops::Sub for ModInt<P> {
        type Output = ModInt<P>;

        fn sub(self, rhs: Self) -> Self::Output {
            let result = self.0 - rhs.0;
            ModInt(if result < 0 { result + P } else { result })
        }
    }
    impl<const P: i32> std::ops::SubAssign for ModInt<P> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
    impl<const P: i32> std::ops::Neg for ModInt<P> {
        type Output = ModInt<P>;

        fn neg(self) -> Self::Output {
            ModInt(if self.0 == 0 { 0 } else { P - self.0 })
        }
    }
    impl<const P: i32> std::ops::Mul for ModInt<P> {
        type Output = ModInt<P>;

        fn mul(self, rhs: Self) -> Self::Output {
            ModInt((self.0 as i64 * rhs.0 as i64 % P as i64) as i32)
        }
    }
    impl<const P: i32> std::ops::MulAssign for ModInt<P> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }
    impl<const P: i32> Into<i32> for ModInt<P> {
        fn into(self) -> i32 {
            self.0
        }
    }
    impl<const P: i32> Into<i64> for ModInt<P> {
        fn into(self) -> i64 {
            self.0 as i64
        }
    }
    impl<const P: i32> From<i32> for ModInt<P> {
        fn from(value: i32) -> Self {
            ModInt::new(value)
        }
    }
    impl<const P: i32> From<i64> for ModInt<P> {
        fn from(value: i64) -> Self {
            ModInt::new((value % P as i64) as i32)
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
        assert_eq!(mint::new(10).pow(2.into()), 7.into());
        x += y;
        assert_eq!(x, 22.into());
        let z: mint = 10_i64.into();
        let a = mint::new(1e16 as i64);
        assert_eq!(z, 10.into());
        assert_eq!(a, 10.into());
    }
}
