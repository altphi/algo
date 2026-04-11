

fn find_first_occurrence(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len()-1;
    let mut index = -1;

    while left <= right {
        let mid = left + (right-left) / 2;

        if mid == left {
            if arr[left] == target {
                index = left as i32;
            } else if arr[right] == target {
                index = right as i32;
            }
            break
        }

        //println!("l {}", left);
        //println!("r {}", right);
        //println!("m {}", mid);

        if arr[mid] == target {
            right = mid;
            index = mid as i32;
        } else if arr[mid] < target {
            left = mid;
        } else if arr[mid] > target {
            right = mid;
        }
    }

    index
}


fn main() { }


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_early() {
    let input = vec![1, 3, 3, 3, 3, 6, 10, 10, 10, 100];
    let result = find_first_occurrence(input, 3);
    assert_eq!(result, 1);
  }

  #[test]
  fn test_notfound() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_occurrence(input, 9);
    assert_eq!(result, -1);
  }

  #[test]
  fn test_late() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 8];
    let result = find_first_occurrence(input, 8);
    assert_eq!(result, 7);
  }

  #[test]
  fn test_repeats() {
    let input = vec![1, 2, 2, 2, 2, 2, 2, 3, 5, 8, 8, 10];
    let result = find_first_occurrence(input, 2);
    assert_eq!(result, 1);
  }

}

