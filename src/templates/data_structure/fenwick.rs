pub mod fenwick {
    #[derive(Default, Debug)]
    pub struct Fenwick<T> {
        sum: Vec<T>,
        n: usize,
    }

    impl<T> Fenwick<T>
    where
        T: Default
            + Clone
            + Copy
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
    {
        pub fn new(n: usize) -> Self {
            Self {
                sum: vec![T::default(); n + 1],
                n,
            }
        }

        pub fn add(&mut self, mut x: usize, v: T) {
            while x <= self.n {
                self.sum[x] += v;
                x += x & x.wrapping_neg();
            }
        }

        pub fn get(&self, mut x: usize) -> T {
            let mut ans = T::default();
            while x > 0 {
                ans += self.sum[x];
                x -= x & x.wrapping_neg();
            }
            ans
        }

        pub fn add_range(&mut self, l: usize, r: usize, v: T) {
            assert!(l <= r);
            self.add(l, v);
            self.add(r + 1, -v);
        }

        pub fn get_range(&self, l: usize, r: usize) -> T {
            assert!(l <= r);
            self.get(r) - self.get(l - 1)
        }
    }
}

mod tests {
    #[test]
    fn test_fenwick() {
        use super::fenwick::Fenwick;

        let mut fen = Fenwick::<i64>::new(10);
        for i in 1..=10 {
            fen.add(i, i as i64);
        }

        assert_eq!(fen.get_range(1, 10), 55);
        assert_eq!(fen.get_range(3, 8), 33);

        use crate::math::modint::modint::mint998244353;
        let mut fen = Fenwick::<mint998244353>::new(10);
        for i in 1..=10 {
            fen.add(i, (i as i64).into());
        }

        assert_eq!(fen.get_range(1, 10), 55.into());
        assert_eq!(fen.get_range(3, 8), 33.into());
    }
}
