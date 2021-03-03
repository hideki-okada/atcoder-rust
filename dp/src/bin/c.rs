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
        ABC: [(u32, u32, u32); N]
    }
    // dp[i][j] i日目にj(0~2がa~cに対応)を選んだ時の最大値
    let mut dp = vec![[0; 3]; N + 1];
    for i in 1..=N {
        dp[i][0] = dp[i - 1][1].max(dp[i - 1][2]) + ABC[i - 1].0;
        dp[i][1] = dp[i - 1][2].max(dp[i - 1][0]) + ABC[i - 1].1;
        dp[i][2] = dp[i - 1][0].max(dp[i - 1][1]) + ABC[i - 1].2;
    }
    println!("{}", dp[N].iter().max().unwrap())
}
