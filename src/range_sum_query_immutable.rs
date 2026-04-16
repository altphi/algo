use std::collections::HashMap;
use std::error;
use std::io;
use std::str::FromStr;

fn range_sum_query_immutable(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let mut sums: HashMap<usize, i32> = HashMap::new();
    let mut current_sum = 0;

    for (i, x) in nums.iter().enumerate() {
        current_sum += x;
        sums.insert(i, current_sum);
    }

    let r: i32 = sums[&(right as usize)];
    let mut l: i32 = 0;
    if (left - 1) >= 0 {
        l = sums[&(left as usize - 1)];
    }
    r-l
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
    let left = line.trim_end().parse()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let right = line.trim_end().parse()?;
    let res = range_sum_query_immutable(nums, left, right);
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::range_sum_query_immutable;

    #[test]
    fn basic_example() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(range_sum_query_immutable(nums, 1, 3), 9);
    }

    #[test]
    fn single_element_array() {
        let nums = vec![5];
        assert_eq!(range_sum_query_immutable(nums, 0, 0), 5);
    }

    #[test]
    fn whole_array() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(range_sum_query_immutable(nums, 0, 4), 15);
    }

    #[test]
    fn left_boundary_single_index() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(range_sum_query_immutable(nums, 0, 0), 1);
    }

    #[test]
    fn right_boundary_single_index() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(range_sum_query_immutable(nums, 4, 4), 5);
    }

    #[test]
    fn middle_range() {
        let nums = vec![4, 6, 1, 3, 8];
        assert_eq!(range_sum_query_immutable(nums, 1, 3), 10);
    }

    #[test]
    fn includes_negative_numbers() {
        let nums = vec![10, -2, 7, 1];
        assert_eq!(range_sum_query_immutable(nums, 1, 2), 5);
    }

    #[test]
    fn all_negative_numbers() {
        let nums = vec![-1, -2, -3, -4];
        assert_eq!(range_sum_query_immutable(nums, 1, 3), -9);
    }

    #[test]
    fn zeros_in_array() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(range_sum_query_immutable(nums, 1, 2), 0);
    }

    #[test]
    fn two_element_range() {
        let nums = vec![9, 1, 5, 2];
        assert_eq!(range_sum_query_immutable(nums, 2, 3), 7);
    }

    #[test]
    fn repeated_values() {
        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(range_sum_query_immutable(nums, 1, 4), 12);
    }

    #[test]
    fn query_entire_negative_to_positive_span() {
        let nums = vec![-5, 2, 4, -1, 3];
        assert_eq!(range_sum_query_immutable(nums, 0, 4), 3);
    }
}
