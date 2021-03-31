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
        N: u64,
        M: usize,
        T: u64,
        AB: [(u64, u64); M]
    }
    let mut current_battery = N.clone();
    let mut current_time = 0;
    for i in 0..M {
        if current_battery <= AB[i].0 - current_time {
            break;
        } else {
            current_battery -= AB[i].0 - current_time;
        }
        current_battery += AB[i].1 - AB[i].0;
        if current_battery > N {
            current_battery = N.clone();
        }
        current_time = AB[i].1;
    }
    if current_battery > T - current_time {
        println!("Yes");
    } else {
        println!("No");
    }
}
