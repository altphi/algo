

fn find_min_rotated(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len()-1;
    let mut boundary_index: i32 = 0;

    let is_feasible = |x| x < arr[0];

    while left <= right {
        let mid = left + (right-left)/2;

        if mid == left {
            if is_feasible(arr[left]) {
                boundary_index = left as i32;
            } else if is_feasible(arr[right]) {
                boundary_index = right as i32;
            }
            break
        }

        if is_feasible(arr[mid]) {
            right = mid;
            boundary_index = mid as i32;
        } else {
            left = mid;
        }
    }

    boundary_index
}


#[cfg(test)]
mod tests {

use super::*;


  #[test]
  fn test_single() {
    let input = vec![3, 4, 5, 6, 2];
    let result = find_min_rotated(input);
    assert_eq!(result, 4);
  }

  #[test]
  fn test_triple() {
    let input = vec![3, 4, 5, 6, 1, 2, 3];
    let result = find_min_rotated(input);
    assert_eq!(result, 4);
  }

}



