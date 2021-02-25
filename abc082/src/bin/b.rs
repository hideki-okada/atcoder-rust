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
        S: String,
        T: String,
    }
    let mut S_Chars = S.chars().collect::<Vec<_>>();
    let mut T_Chars = T.chars().collect::<Vec<_>>();
    S_Chars.sort();
    T_Chars.sort();
    T_Chars.reverse();
    let S = S_Chars.iter().collect::<String>();
    let T = T_Chars.iter().collect::<String>();
    if S < T {
        println!("Yes")
    } else {
        println!("No")
    }
}
