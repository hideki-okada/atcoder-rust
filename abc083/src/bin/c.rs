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
        mut X: usize,
        Y: usize
    }
    let mut ans = 0;
    while X <= Y {
        X *= 2;
        ans += 1;
    }
    println!("{}", ans)
}
