

fn square_root(n: i32) -> i32 {
    let mut left = 0;
    let mut right = n;
    let mut mid: i32 = 0;

    while left <= right {
        mid = left + (right-left)/2;
        let mid_squared = mid.pow(2);

        if mid == left {
            if right.pow(2) == n {
               return right
            }
            break
        }

        if mid_squared == n {
            return mid
        } else if mid_squared < n {
            left = mid;
        } else if mid_squared > n {
            right = mid;
        }
    }

  mid
}


fn main() { }


#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn test_sixteen() {
    assert_eq!(square_root(16), 4);
  }

  #[test]
  fn test_sixtyfour() {
    assert_eq!(square_root(64), 8);
  }

  #[test]
  fn test_eight() {
    assert_eq!(square_root(8), 2);
  }

  #[test]
  fn test_one() {
    assert_eq!(square_root(1), 1);
  }

  #[test]
  fn test_zero() {
    assert_eq!(square_root(0), 0);
  }
}

