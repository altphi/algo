use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let nums_sans_zero: Vec<i32> = nums.iter().filter(|&&x| x != 0).cloned().collect();
    let zero_count = nums.len() - nums_sans_zero.len();

    if zero_count >= 2 {
        return vec![0; nums.len()];
    }

    let product: i32 = nums_sans_zero.iter().product();
    let mut results: Vec<i32> = vec![];

    for x in nums.iter() {
        if zero_count == 1 {
            if *x == 0 {
                results.push(product);
            } else {
                results.push(0)
            }
        } else {
          results.push(product / *x);
        }
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

