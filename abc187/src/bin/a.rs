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
        A: u32,
        B: u32,
    }
    let sum_A = keta_sum(A);
    let sum_B = keta_sum(B);
    if sum_A >= sum_B {
        println!("{}", sum_A)
    } else {
        println!("{}", sum_B)
    }
}

fn keta_sum(num: u32) -> u32 {
    let mut i = num;
    let mut ans = 0;
    while i > 0 {
        ans += i % 10;
        i /= 10;
    }
    ans
}
