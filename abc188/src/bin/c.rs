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
        N: u32,
        A: [u64; 2usize.pow(N)]
    }
    let mut stack = A.into_iter().enumerate().collect::<Vec<_>>();
    while stack.len() > 2 {
        stack = stack
            .chunks(2)
            .into_iter()
            .map(|s| if s[0].1 > s[1].1 { s[0] } else { s[1] })
            .collect::<Vec<_>>();
    }
    println!(
        "{}",
        if stack[0].1 > stack[1].1 {
            stack[1].0 + 1
        } else {
            stack[0].0 + 1
        }
    )
}
