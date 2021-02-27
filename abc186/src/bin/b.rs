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
        H: usize,
        W: usize,
        A: [usize; H*W]
    }
    println!(
        "{}",
        A.iter().sum::<usize>() - A.iter().min().unwrap() * H * W
    )
}
