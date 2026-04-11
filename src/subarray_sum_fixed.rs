use std::error;
use std::io;
use std::str::FromStr;

fn subarray_sum_fixed(nums: Vec<i32>, k: i32) -> i32 {
    let mut largest_sum = 0;

    for w in nums.windows(k as usize) {
        largest_sum = largest_sum.max(w.iter().sum());
    }

    largest_sum
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
    let k = line.trim_end().parse()?;
    let res = subarray_sum_fixed(nums, k);
    println!("{}", res);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::subarray_sum_fixed;

    #[test]
    fn example_case() {
        let nums = vec![1, 2, 3, 7, 4, 1];
        assert_eq!(subarray_sum_fixed(nums, 3), 14);
    }

    #[test]
    fn window_size_one() {
        let nums = vec![4, 2, 9, 1, 5];
        assert_eq!(subarray_sum_fixed(nums, 1), 9);
    }

    #[test]
    fn window_size_equals_array_length() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(subarray_sum_fixed(nums, 4), 10);
    }

    #[test]
    fn all_zeroes() {
        let nums = vec![0, 0, 0, 0, 0];
        assert_eq!(subarray_sum_fixed(nums, 3), 0);
    }

    #[test]
    fn repeated_values() {
        let nums = vec![5, 5, 5, 5, 5];
        assert_eq!(subarray_sum_fixed(nums, 2), 10);
    }

    #[test]
    fn max_window_at_start() {
        let nums = vec![9, 8, 1, 1, 1];
        assert_eq!(subarray_sum_fixed(nums, 2), 17);
    }

    #[test]
    fn max_window_in_middle() {
        let nums = vec![1, 4, 9, 2, 3];
        assert_eq!(subarray_sum_fixed(nums, 3), 15);
    }

    #[test]
    fn max_window_at_end() {
        let nums = vec![1, 1, 2, 8, 9];
        assert_eq!(subarray_sum_fixed(nums, 2), 17);
    }

    #[test]
    fn includes_zeroes_between_values() {
        let nums = vec![3, 0, 5, 0, 2];
        assert_eq!(subarray_sum_fixed(nums, 3), 8);
    }

    #[test]
    fn large_single_peak_not_enough_without_window_support() {
        let nums = vec![1, 1, 100, 1, 1];
        assert_eq!(subarray_sum_fixed(nums, 2), 101);
    }

    #[test]
    fn k_near_array_length() {
        let nums = vec![2, 1, 3, 4, 5];
        assert_eq!(subarray_sum_fixed(nums, 4), 13);
    }

    #[test]
    fn two_element_windows() {
        let nums = vec![7, 2, 4, 6, 1];
        assert_eq!(subarray_sum_fixed(nums, 2), 10);
    }
}
