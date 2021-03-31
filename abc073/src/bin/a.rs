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
        N: u32
    }
    if N / 10 == 9 {
        println!("Yes")
    } else if N - (N / 10) * 10 == 9 {
        println!("Yes")
    } else {
        println!("No")
    }
}
