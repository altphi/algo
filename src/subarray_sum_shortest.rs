
use std::error;
use std::io;
use std::str::FromStr;

fn subarray_sum_shortest(nums: Vec<i32>, target: i32) -> i32 {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut shortest_length = nums.len() + 1;

    while p2 <= nums.len() {
        if nums[p1..p2].iter().sum::<i32>() < target {
            p2 += 1;
        } else {
            shortest_length = shortest_length.min(p2-p1);
            if shortest_length == 1 {
                return 1;
            }
            p1 += 1;
        }
    }

    shortest_length as i32
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
    let res = subarray_sum_shortest(nums, target);
    println!("{}", res);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::subarray_sum_shortest;

    #[test]
    fn example_case() {
        let nums = vec![1, 4, 1, 7, 3, 0, 2, 5];
        assert_eq!(subarray_sum_shortest(nums, 10), 2);
    }

    #[test]
    fn single_element_meets_target() {
        let nums = vec![1, 2, 10, 3];
        assert_eq!(subarray_sum_shortest(nums, 10), 1);
    }

    #[test]
    fn whole_array_needed() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(subarray_sum_shortest(nums, 4), 4);
    }

    #[test]
    fn shortest_at_start() {
        let nums = vec![6, 5, 1, 1, 1];
        assert_eq!(subarray_sum_shortest(nums, 10), 2);
    }

    #[test]
    fn shortest_in_middle() {
        let nums = vec![1, 2, 3, 7, 2, 1];
        assert_eq!(subarray_sum_shortest(nums, 9), 2);
    }

    #[test]
    fn shortest_at_end() {
        let nums = vec![1, 1, 1, 8, 3];
        assert_eq!(subarray_sum_shortest(nums, 11), 2);
    }

    #[test]
    fn includes_zeroes() {
        let nums = vec![2, 0, 2, 0, 6];
        assert_eq!(subarray_sum_shortest(nums, 6), 1);
    }

    #[test]
    fn zeros_force_window_expansion() {
        let nums = vec![0, 0, 0, 5, 5];
        assert_eq!(subarray_sum_shortest(nums, 10), 2);
    }

    #[test]
    fn shrinking_window_multiple_steps() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(subarray_sum_shortest(nums, 11), 3);
    }

    #[test]
    fn many_small_values_then_large() {
        let nums = vec![1, 1, 1, 1, 9];
        assert_eq!(subarray_sum_shortest(nums, 9), 1);
    }

    #[test]
    fn exact_match_requires_shrinking() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(subarray_sum_shortest(nums, 7), 2);
    }

    #[test]
    fn target_met_with_minimal_overlap() {
        let nums = vec![5, 1, 3, 5, 10, 7];
        assert_eq!(subarray_sum_shortest(nums, 15), 2);
    }

    #[test]
    fn repeated_values() {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(subarray_sum_shortest(nums, 6), 3);
    }
}
