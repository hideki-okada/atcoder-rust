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
        WV: [(usize, usize); N]
    }
    let V = WV.iter().map(|&o| o.1).sum();
    // dp[i][v]をi番目までの品物から価値の総和がvになるように選んだ時の重さの総和の最小値
    // let mut dp = vec![vec![100000000001; V + 1]; N];
    // dp[0][0] = 0;
    // for i in 1..=N {
    //     //for v in 0..=V {
    //     //    if v >= WV[i - 1].1 {
    //     //        dp[i][v] = dp[i - 1][v].min(dp[i - 1][v - WV[i - 1].1] + WV[i - 1].0);
    //     //    } else {
    //     //        dp[i][v] = dp[i - 1][v];
    //     //    }
    //     for v in 0..WV[i - 1].1 {
    //         dp[i][v] = dp[i - 1][v];
    //     }
    //     for v in WV[i - 1].1..=V {
    //         dp[i][v] = dp[i - 1][v].min(dp[i - 1][v - WV[i - 1].1] + WV[i - 1].0);
    //     }
    // }
    // let mut max_v = 0;
    // for j in (0..=V).rev() {
    //     if dp[N][j] <= W {
    //         max_v = j;
    //         break;
    //     }
    // }

    // 省メモリバージョン
    // dp[v]を価値の総和がvになる時の重さの総和の最小値とする
    let mut dp = vec![100000000001; V + 1];
    dp[0] = 0;
    for i in 0..N {
        for v in (WV[i].1..=V).rev() {
            dp[v] = dp[v].min(dp[v - WV[i].1] + WV[i].0);
        }
    }
    let mut max_v = 0;
    for v in (0..=V).rev() {
        if dp[v] <= W {
            max_v = v;
            break;
        }
    }
    println!("{}", max_v);
}
