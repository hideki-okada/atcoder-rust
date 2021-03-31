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
        L: [u32; N]
    }
    println!(
        "{}",
        if L.iter().sum::<u32>() > 2 * L.iter().max().unwrap() {
            "Yes"
        } else {
            "No"
        }
    )
}
