use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix_sums: Vec<i32> = vec![];
    let mut suffix_sums: Vec<i32> = vec![];

    let mut r_index = nums.len().saturating_sub(1);
    for (i, x) in nums.iter().take(nums.len().saturating_sub(1)).enumerate() {
        if i == 0 {
            prefix_sums.push(1);
            suffix_sums.push(1);
        }

        prefix_sums.push(prefix_sums.last().unwrap() * x);
        suffix_sums.push(suffix_sums.last().unwrap() * nums[r_index]);
        r_index -= 1;
    }

    let mut results: Vec<i32> = vec![];
    r_index = prefix_sums.len();
    for x in prefix_sums {
        r_index -= 1;
        results.push(x * suffix_sums[r_index]);
    }

    results
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
    let nums = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let res = product_of_array_except_self(nums);
    print_words(&res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::product_of_array_except_self;

    #[test]
    fn basic_example() {
        let nums = vec![1, 2, 3, 4];
        // 1 1 2 6
        // 1 4 12 24   (via 24 12 4 1)
        assert_eq!(product_of_array_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn single_zero() {
        let nums = vec![1, 2, 0, 4];
        assert_eq!(product_of_array_except_self(nums), vec![0, 0, 8, 0]);
    }

    #[test]
    fn multiple_zeroes() {
        let nums = vec![0, 2, 0, 4];
        assert_eq!(product_of_array_except_self(nums), vec![0, 0, 0, 0]);
    }

    #[test]
    fn all_ones() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(product_of_array_except_self(nums), vec![1, 1, 1, 1]);
    }

    #[test]
    fn includes_negatives() {
        let nums = vec![-1, 2, -3, 4];
        assert_eq!(product_of_array_except_self(nums), vec![-24, 12, -8, 6]);
    }

    #[test]
    fn single_negative() {
        let nums = vec![1, 2, 3, -4];
        assert_eq!(product_of_array_except_self(nums), vec![-24, -12, -8, 6]);
    }

    #[test]
    fn two_elements() {
        let nums = vec![5, 10];
        assert_eq!(product_of_array_except_self(nums), vec![10, 5]);
    }

    #[test]
    fn repeated_values() {
        let nums = vec![2, 2, 2, 2];
        assert_eq!(product_of_array_except_self(nums), vec![8, 8, 8, 8]);
    }

    #[test]
    fn zero_at_start() {
        let nums = vec![0, 1, 2, 3];
        assert_eq!(product_of_array_except_self(nums), vec![6, 0, 0, 0]);
    }

    #[test]
    fn zero_at_end() {
        let nums = vec![1, 2, 3, 0];
        assert_eq!(product_of_array_except_self(nums), vec![0, 0, 0, 6]);
    }

    #[test]
    fn alternating_signs() {
        let nums = vec![-1, 1, -1, 1];
        assert_eq!(product_of_array_except_self(nums), vec![-1, 1, -1, 1]);
    }

    #[test]
    fn larger_values() {
        let nums = vec![2, 3, 5, 7];
        assert_eq!(product_of_array_except_self(nums), vec![105, 70, 42, 30]);
    }
}
