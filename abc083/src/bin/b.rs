use proconio::{fastout, input};
#[allow(unused_macros)]
macro_rules! debug {
      ($($a:expr),*) => {
          eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
      }
  }
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: u32,
        A: u32,
        B: u32
    }
    let mut total = 0;
    for n in 1..(N + 1) {
        let mut ketanowa = 0;
        let mut i = n;
        while i > 0 {
            ketanowa += i % 10;
            i /= 10;
        }
        if A <= ketanowa && ketanowa <= B {
            total += n;
        }
    }
    println!("{}", total)
}
