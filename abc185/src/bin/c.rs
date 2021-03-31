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
        L: usize
    }
    // L-1C11
    // dp[i][j] = iCj
    let mut dp = vec![vec![0u64; 12]; L];
    dp[1][1] = 1;
    for i in 0..L {
        dp[i][0] = 1;
    }
    for i in 1..L {
        for j in 1..=11 {
            if j > i {
                break;
            }
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        }
    }
    println!("{}", dp[L - 1][11])
}
