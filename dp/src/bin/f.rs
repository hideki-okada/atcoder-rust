use proconio::marker::Chars;
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
        S: Chars,
        T: Chars
    }
    // dp[i][j] sのi文字目までとtのj文字目までの文字の最長部分列の長さ
    let mut dp = vec![vec![0; T.len() + 1]; S.len() + 1];
    dp[0][0] = 0;
    for i in 0..S.len() {
        for j in 0..T.len() {
            if S[i] == T[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    let mut answers = vec![];
    let mut current_i = S.len();
    let mut current_j = T.len();
    while current_i > 0 && current_j > 0 {
        if S[current_i - 1] == T[current_j - 1] {
            answers.push(S[current_i - 1]);
            current_i -= 1;
            current_j -= 1;
        } else {
            if dp[current_i - 1][current_j] == dp[current_i][current_j] {
                current_i -= 1;
            } else {
                current_j -= 1;
            }
        }
    }
    answers.reverse();
    if !answers.is_empty() {
        println!("{}", answers.iter().collect::<String>());
    }
}
