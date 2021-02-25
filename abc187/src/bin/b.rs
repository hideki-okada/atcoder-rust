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
        XY: [(i32, i32);N]
    }
    let mut count = 0;
    for i in 0..N - 1 {
        for j in i + 1..N {
            let d_x = XY[i].0 - XY[j].0;
            let d_y = XY[i].1 - XY[j].1;
            if d_x == 0 {
                continue;
            }
            if d_y.abs() <= d_x.abs() {
                count += 1
            }
        }
    }
    println!("{}", count)
}
