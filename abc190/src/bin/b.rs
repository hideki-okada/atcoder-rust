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
    S: u32,
    D: u32,
    XY: [(u32, u32); N]
    }
    if XY.iter().find(|x| x.0 < S && x.1 > D).is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
