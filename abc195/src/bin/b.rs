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
        A: i32,
        B: i32,
        W: i32
    }
    if (W * 1000 / A) == (W * 1000 / B) {
        println!("UNSATISFIABLE")
    } else {
        println!(
            "{} {}",
            (W as f64 * 1000.0 / B as f64).ceil(),
            (W as f64 * 1000.0 / A as f64).floor()
        )
    }
}
