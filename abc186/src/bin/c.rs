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
    // 10進法で絞る
    let mut counter = 0;
    for n in 1..=N {
        if !format!("{}", n).contains('7') && !format!("{:o}", n).contains('7') {
            counter += 1;
        }
    }
    println!("{}", counter)
}
