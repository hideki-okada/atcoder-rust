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
        n: usize,
        h: [i32; n]
    }
    // 足場iにたどり着くまでの最小コスト
    let mut dp = vec![1000000000; n + 1];
    dp[0] = 0;
    dp[1] = 0;
    // 配るdp
    for i in 1..n {
        dp[i + 1] = dp[i + 1].min(dp[i] + (h[i] - h[i - 1]).abs());
        if i < n - 1 {
            dp[i + 2] = dp[i + 2].min(dp[i] + (h[i + 1] - h[i - 1]).abs())
        }
    }
    println!("{}", dp[n])
}
