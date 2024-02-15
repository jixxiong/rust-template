use std::io::{self, Write};

#[allow(unused)]
static mut CIN: Vec<String> = Vec::<String>::new();
static mut COUT: Option<io::BufWriter<io::Stdout>> = None;
#[rustfmt::skip]
#[allow(unused)]
macro_rules! cout { ($($arg:tt)*) => {{ match unsafe { &mut COUT } { Some(out) => out.write_fmt(format_args!($($arg)*)).unwrap(), None => panic!(), } } }; }
#[rustfmt::skip]
#[allow(unused)]
fn cin<T: std::str::FromStr>() -> T { unsafe { loop { if let Some(token) = CIN.pop() { return token.parse().ok().expect("Failed parse"); } let mut input = String::new(); io::stdin().read_line(&mut input).expect("Failed read"); CIN = input.split_whitespace().rev().map(String::from).collect(); } } }

fn solve() {
    todo!()
}

fn main() {
    unsafe { COUT = Some(io::BufWriter::new(io::stdout())) };
    let multiple_test = false; // true
    for _ in 1..=if multiple_test { cin::<i32>() } else { 1 } {
        solve();
    }
    match unsafe { &mut COUT } {
        Some(out) => out.flush().unwrap(),
        None => panic!(),
    }
}
