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
        W: usize,
        WV: [(usize, u64); N]
    }
    // dp[i][j]をi番目までの品物を重さjを超えないように買う場合の価値の最大値
    let mut dp = vec![vec![0; W + 1]; N + 1];
    for i in 1..=N {
        // 以下のやり方はj - WV[i-1].0が0未満になった瞬間にpanicになるのでダメ
        // for j in 1..=W {
        // if j - WV[i-1].0 >=0 {
        //      dp更新;
        //      }
        // }
        for j in 1..WV[i - 1].0 {
            dp[i][j] = dp[i - 1][j];
        }
        for j in WV[i - 1].0..=W {
            dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - WV[i - 1].0] + WV[i - 1].1);
        }
    }
    println!("{}", dp[N][W])
}
