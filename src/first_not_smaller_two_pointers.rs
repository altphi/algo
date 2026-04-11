

fn find_first_not_smaller(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len()-1;
    let mut boundary_index = -1;

    while left <= right {
        if left == right {
            boundary_index = left as i32;
            break
        }
        let mid = left + (right-left)/2;

        if arr[mid] >= target {
            right = mid;
            boundary_index = right as i32;
        } else {
            left = mid+1;
        }
    }

    boundary_index
}


fn main() { }


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_middle() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_not_smaller(input, 4);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_firstel() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_not_smaller(input, 0);
    assert_eq!(result, 0);
  }

  #[test]
  fn test_lastel() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_not_smaller(input, 8);
    assert_eq!(result, 7);
  }

  #[test]
  fn test_repeats() {
    let input = vec![1, 2, 2, 2, 2, 2, 2, 3, 5, 8, 8, 10];
    let result = find_first_not_smaller(input, 2);
    assert_eq!(result, 1);
  }

}

