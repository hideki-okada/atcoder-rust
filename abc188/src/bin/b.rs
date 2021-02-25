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
        N: usize,
        A: [i32; N],
        B: [i32; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans += A[i] * B[i]
    }
    if ans == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
