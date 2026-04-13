use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn subarray_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut prefix_sum: HashMap<i32, usize> = HashMap::new();
    let mut current_sum = 0;
    let mut complement: i32;

    for (i, x) in arr.iter().enumerate() {
        current_sum += x;

        if current_sum == target {
            return vec![0, (i as i32) + 1];
        }

        complement = current_sum - target;
        if let Some(c) = prefix_sum.get(&complement) {
            return vec![*c as i32 + 1, (i as i32) + 1];
        }

        prefix_sum.insert(current_sum, i);
    }

    Vec::new()
}

fn print_words<T: Display>(v: &Vec<T>) {
    let mut iter = v.iter();
    if let Some(val) = iter.next() {
        print!("{}", val);
        for val in iter {
            print!(" {}", val);
        }
    }
    println!();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let arr = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let target = line.trim_end().parse()?;
    let res = subarray_sum(arr, target);
    print_words(&res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::subarray_sum;

    #[test]
    fn example_case() {
        let arr = vec![1, -20, -3, 30, 5, 4];
        assert_eq!(subarray_sum(arr, 7), vec![1, 4]);
    }

    #[test]
    fn subarray_at_start() {
        let arr = vec![2, 3, 1, 4];
        assert_eq!(subarray_sum(arr, 5), vec![0, 2]);
    }

    #[test]
    fn subarray_at_end() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(subarray_sum(arr, 7), vec![2, 4]);
    }

    #[test]
    fn whole_array_matches() {
        let arr = vec![1, 2, 3];
        assert_eq!(subarray_sum(arr, 6), vec![0, 3]);
    }

    #[test]
    fn single_element_match() {
        let arr = vec![5, 1, 2];
        assert_eq!(subarray_sum(arr, 1), vec![1, 2]);
    }

    #[test]
    fn includes_negative_numbers() {
        let arr = vec![3, -1, -2, 5, 1];
        assert_eq!(subarray_sum(arr, 2), vec![0, 2]);
    }

    #[test]
    fn negative_prefix_then_match() {
        let arr = vec![-5, 2, 3, 1];
        assert_eq!(subarray_sum(arr, 5), vec![1, 3]);
    }

    #[test]
    fn multiple_possible_answers_return_first() {
        let arr = vec![1, 2, 3, 2, 1];
        assert_eq!(subarray_sum(arr, 5), vec![1, 3]);
    }

    #[test]
    fn zero_target_with_zeroes() {
        let arr = vec![1, -1, 0, 2];
        assert_eq!(subarray_sum(arr, 0), vec![0, 2]);
    }

    #[test]
    fn zero_target_single_zero() {
        let arr = vec![5, 0, 3];
        assert_eq!(subarray_sum(arr, 0), vec![1, 2]);
    }

    #[test]
    fn negative_target() {
        let arr = vec![2, -3, 1, -4, 2];
        assert_eq!(subarray_sum(arr, -2), vec![1, 3]);
    }

    #[test]
    fn later_match_not_shorter_but_valid() {
        let arr = vec![1, 2, -1, 2, 3];
        assert_eq!(subarray_sum(arr, 4), vec![0, 4]);
    }

    #[test]
    fn exact_match_in_middle() {
        let arr = vec![4, 2, -1, 1, 2];
        assert_eq!(subarray_sum(arr, 2), vec![1, 2]);
    }
}
