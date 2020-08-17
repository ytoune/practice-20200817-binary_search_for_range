use binary_search_for_range::*;

// https://atcoder.jp/contests/abc174/submissions/15971483

fn main() {
  let n: usize;
  let k: i64;
  let a: Vec<i64>;

  text_io::scan!("{} {}", n, k);
  a = (0..n).map(|_| text_io::read!()).collect();

  use std::cmp::Ordering::*;
  let ans = (0..1_000_000_001)
    .binary_search_by(|m| {
      if 1 == *m {
        // m が 1 未満になることはない
        return Equal;
      };
      if k >= a.iter().map(|a| (a + m - 1) / m - 1).sum() {
        Greater
      } else {
        Less
      }
    })
    .and_then(|q: i64| -> Result<i64, i64> { Err(q) })
    .err();
  println!("{}", ans.unwrap());
}
