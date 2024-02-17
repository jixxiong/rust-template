use std::io::{self, Write};

fn solve() {
    todo!()
}

fn main() {
    unsafe { COUT = Some(io::BufWriter::new(io::stdout())) };
    let multiple_tests = false; // true
    for _ in 1..=if multiple_tests { cin::<i32>() } else { 1 } {
        solve();
    }
    unsafe { &mut COUT }.as_mut().unwrap().flush().unwrap();
}

#[rustfmt::skip]
#[allow(unused)]
fn cin<T: std::str::FromStr>() -> T { unsafe { loop { if let Some(token) = CIN.pop() { return token.parse().ok().expect("Failed parse"); } let mut input = String::new(); io::stdin().read_line(&mut input).expect("Failed read"); CIN = input.split_whitespace().rev().map(String::from).collect(); } } }
#[allow(unused)]
#[macro_export]
macro_rules! cout { ($($arg:tt)*) => {{ unsafe { &mut COUT }.as_mut().unwrap().write_fmt(format_args!($($arg)*)).unwrap() }}; }
static mut CIN: Vec<String> = Vec::<String>::new();
static mut COUT: Option<io::BufWriter<io::Stdout>> = None;
