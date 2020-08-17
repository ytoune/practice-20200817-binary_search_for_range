pub trait BinarySearchForRange<T> {
  fn binary_search_by<F>(&self, f: F) -> Result<T, T>
  where
    F: FnMut(&T) -> std::cmp::Ordering;
}

impl<
    T: PartialOrd
      + std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Div<Output = T>
      + Copy
      + From<u8>,
  > BinarySearchForRange<T> for std::ops::Range<T>
{
  fn binary_search_by<F>(&self, mut f: F) -> Result<T, T>
  where
    F: FnMut(&T) -> std::cmp::Ordering,
  {
    use std::cmp::Ordering::*;
    if self.end <= self.start {
      return Err(self.start);
    }
    let mut base = self.start;
    let mut size = self.end - base;
    loop {
      let half = size / T::from(2);
      let mid = base + half;
      let cmp = f(&mid);
      match cmp {
        Less => base = mid,
        Equal => return Ok(mid),
        _ => {}
      }
      if size <= half || T::from(0) == half {
        return Err(base + T::from((cmp == Less) as u8));
      }
      size = size - half;
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    use crate::*;
    assert_eq!((0..10).binary_search_by(|m| m.cmp(&4)), Ok(4));
    assert_eq!((0..10).binary_search_by(|m| m.cmp(&9)), Ok(9));
    assert_eq!((0..10).binary_search_by(|m| m.cmp(&10)), Err(10));
    assert_eq!((0..10).binary_search_by(|m| m.cmp(&-1)), Err(0));
  }
}
