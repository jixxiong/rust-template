#[allow(unused)]
pub mod fenwick {
    struct Fenwich {
        sum: Vec<i64>,
        n: i32,
    }
    impl Fenwich {
        fn new(n: usize) -> Self {
            Self {
                sum: vec![0; n + 1],
                n: n as i32,
            }
        }
        fn add(&mut self, mut x: i32, v: i64) {
            while x <= self.n {
                self.sum[x as usize] += v;
                x += x & -x;
            }
        }
        fn get(&self, mut x: i32) -> i64 {
            let mut ans = 0;
            while x > 0 {
                ans += self.sum[x as usize];
                x -= x & -x;
            }
            ans
        }
        fn add_range(&mut self, l: i32, r: i32, v: i64) {
            assert!(l <= r);
            self.add(l, v);
            self.add(r + 1, -v);
        }
        fn get_range(&self, l: i32, r: i32) -> i64 {
            assert!(l <= r);
            self.get(r) - self.get(l - 1)
        }
    }
}
