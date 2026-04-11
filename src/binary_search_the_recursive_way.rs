

// sorted array?
// get mid point
// determine if left half or right half should be followed
// imagine 2 items, middle is index ?
//   if 1, and we discard to right, infinite loop
//   if 0, and we discard to left, infinite loop
//   2 items could have special treatment

fn find_first_not_smaller(arr: &Vec<i32>, b: usize, e:usize, target: i32) -> i32 {
    if b == e {
        return b as i32
    }
    let segment_len = (e-b)+1;
    println!("b is {}", b);
    println!("e is {}", e);

    if segment_len == 2 {
        if arr[b] >= target {
            return b as i32
        } else if arr[e] >= target {
            return e as i32
        } else {
            return -1
        }
    }

    let mid: usize = b + (segment_len / 2);
    println!("mid is {}", mid);

    if arr[mid] >= target {
        find_first_not_smaller(arr, b, mid, target)
    } else if arr[mid] < target {
        find_first_not_smaller(arr, mid+1, e, target)
    } else {
        -1
    }
}


fn main() {

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_not_smaller(&input, 0, input.len()-1, 4);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_run_firstel() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_not_smaller(&input, 0, input.len()-1, 0);
    assert_eq!(result, 0);
  }

  #[test]
  fn test_run_lastel() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = find_first_not_smaller(&input, 0, input.len()-1, 8);
    assert_eq!(result, 7);
  }

  #[test]
  fn test_run_repeats() {
    let input = vec![1, 2, 2, 2, 2, 2, 2, 3, 5, 8, 8, 10];
    let result = find_first_not_smaller(&input, 0, input.len()-1, 2);
    assert_eq!(result, 1);
  }

}

