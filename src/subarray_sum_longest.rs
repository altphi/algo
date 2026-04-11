
use std::error;
use std::io;
use std::str::FromStr;

fn subarray_sum_longest(nums: Vec<i32>, target: i32) -> i32 {
    let mut p1 = 0;
    let mut p2 = 1;
    let mut longest_w = 0;

    while p2 <= nums.len() {
        if nums[p1..p2].iter().sum::<i32>() <= target {
            longest_w = longest_w.max(p2-p1);
            if p1 > 0 {
                p1 -= 1;
            } else {
                p2 += 1;
            }
        } else {
            p1 += 1;
            p2 += 1;
        }
    }

    longest_w as i32
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let nums = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let target = line.trim_end().parse()?;
    let res = subarray_sum_longest(nums, target);
    println!("{}", res);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::subarray_sum_longest;

    #[test]
    fn example_case() {
        let nums = vec![1, 6, 3, 1, 2, 4, 5];
        assert_eq!(subarray_sum_longest(nums, 10), 4);
    }

    #[test]
    fn whole_array_fits() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(subarray_sum_longest(nums, 10), 4);
    }

    #[test]
    fn only_single_element_windows_fit() {
        let nums = vec![9, 8, 7, 6];
        assert_eq!(subarray_sum_longest(nums, 7), 1);
    }

    #[test]
    fn no_elements_fit() {
        let nums = vec![5, 6, 7];
        assert_eq!(subarray_sum_longest(nums, 4), 0);
    }

    #[test]
    fn longest_window_at_start() {
        let nums = vec![2, 1, 1, 10, 10];
        assert_eq!(subarray_sum_longest(nums, 4), 3);
    }

    #[test]
    fn longest_window_in_middle() {
        let nums = vec![9, 2, 3, 1, 2, 9];
        assert_eq!(subarray_sum_longest(nums, 8), 4);
    }

    #[test]
    fn longest_window_at_end() {
        let nums = vec![8, 8, 1, 2, 3];
        assert_eq!(subarray_sum_longest(nums, 6), 3);
    }

    #[test]
    fn includes_zeroes() {
        let nums = vec![0, 0, 0, 5, 0];
        assert_eq!(subarray_sum_longest(nums, 5), 5);
    }

    #[test]
    fn all_zeroes_with_zero_target() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(subarray_sum_longest(nums, 0), 4);
    }

    #[test]
    fn target_zero_with_positive_numbers() {
        let nums = vec![1, 2, 3];
        assert_eq!(subarray_sum_longest(nums, 0), 0);
    }

    #[test]
    fn repeated_values() {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(subarray_sum_longest(nums, 6), 3);
    }

    #[test]
    fn shrinking_window_multiple_times() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(subarray_sum_longest(nums, 7), 3);
    }

    #[test]
    fn exact_target_is_allowed() {
        let nums = vec![4, 1, 1, 1, 4];
        assert_eq!(subarray_sum_longest(nums, 7), 4);
    }

    #[test]
    fn long_run_of_small_values_beats_short_large_values() {
        let nums = vec![5, 1, 1, 1, 1, 1, 5];
        assert_eq!(subarray_sum_longest(nums, 5), 5);
    }
}

