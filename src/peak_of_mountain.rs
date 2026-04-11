


fn peak_of_mountain_array(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len()-1;

    while left <= right {
        let mid = left + (right-left)/2;

        if mid == left {
          return mid as i32
        }

        if arr[mid-1] < arr[mid]  && arr[mid] < arr[mid+1] {
            left = mid;
        } else if arr[mid-1] > arr[mid] && arr[mid] > arr[mid+1] {
            right = mid;
        }
        if arr[mid-1] < arr[mid] && arr[mid] > arr[mid+1] {
            return mid as i32
        }
    }


    left as i32
}



fn main() { }


#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn test_middle() {
    let input = vec![0, 1, 2, 3, 2, 1, 0];
    let result = peak_of_mountain_array(input);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_front() {
    let input = vec![10, 11, 12, 11, 10, 9, 8, 7, 6];
    let result = peak_of_mountain_array(input);
    assert_eq!(result, 2);
  }

}



