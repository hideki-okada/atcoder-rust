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
        K: usize,
        H: [i32; N]
    }
    // dp[i] 足場iにたどり着くまでの最小コスト
    let mut dp = vec![1000000000; N + 1];
    dp[0] = 0;
    dp[1] = 0;
    // 配るdpで考える
    for i in 1..N {
        for j in 1..K + 1 {
            if i + j <= N {
                dp[i + j] = dp[i + j].min(dp[i] + (H[i + j - 1] - H[i - 1]).abs())
            }
        }
    }
    println!("{}", dp[N])
}
