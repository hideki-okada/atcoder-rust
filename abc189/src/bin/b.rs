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
    X: u32,
    Drinks: [(u32, u32); N]
    }
    let mut current_alc = 0;
    for i in 0..N {
        current_alc += Drinks[i].0 * Drinks[i].1;
        if current_alc > X * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1")
}
