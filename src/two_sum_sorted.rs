use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn two_sum_sorted(arr: Vec<i32>, target: i32) -> Vec<i32> {
    if arr.is_empty() { return vec![]; }

    let mut p1: usize = 0;
    let mut p2: usize = arr.len() - 1;

    while p2 > p1 {
        while arr[p2] == arr[p2-1] && p2-1 > p1 { p2 -= 1; }  // skip duplicates
        let sum = arr[p2] + arr[p1];

        if sum == target {
            return vec![p1 as i32, p2 as i32];
        }

        if sum < target {
            p1 += 1;
        } else {
            p2 -= 1;
        }
    }

    Vec::new()
}

fn print_words<T: Display>(v: &[T]) {
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
    let res = two_sum_sorted(arr, target);
    print_words(&res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::two_sum_sorted;

    #[test]
    fn basic_case() {
        assert_eq!(two_sum_sorted(vec![1, 2, 3, 4, 6], 6), vec![1, 3]);
    }

    #[test]
    fn minimal_pair_exists() {
        assert_eq!(two_sum_sorted(vec![1, 2], 3), vec![0, 1]);
    }

    #[test]
    fn minimal_pair_does_not_exist() {
        assert_eq!(two_sum_sorted(vec![1, 2], 4), Vec::<i32>::new());
    }

    #[test]
    fn negatives_work() {
        assert_eq!(two_sum_sorted(vec![-5, -3, -1, 0, 2, 4], -4), vec![1, 2]);
    }

    #[test]
    fn negative_and_positive_work() {
        assert_eq!(two_sum_sorted(vec![-10, -5, -2, 3, 7], 5), vec![2, 4]);
    }

    #[test]
    fn zeros_work() {
        assert_eq!(two_sum_sorted(vec![0, 0, 3, 4], 0), vec![0, 1]);
    }

    #[test]
    fn duplicates_can_form_answer() {
        assert_eq!(two_sum_sorted(vec![1, 1, 1, 2, 3], 2), vec![0, 1]);
    }

    #[test]
    fn all_identical_values() {
        assert_eq!(two_sum_sorted(vec![2, 2, 2, 2, 2], 4), vec![0, 1]);
    }

    #[test]
    fn no_solution() {
        assert_eq!(two_sum_sorted(vec![1, 2, 3, 9], 8), Vec::<i32>::new());
    }

    #[test]
    fn single_element() {
        assert_eq!(two_sum_sorted(vec![5], 5), Vec::<i32>::new());
    }

    #[test]
    fn empty_array() {
        assert_eq!(two_sum_sorted(vec![], 5), Vec::<i32>::new());
    }

    #[test]
    fn uses_first_and_last_elements() {
        assert_eq!(two_sum_sorted(vec![1, 3, 5, 7, 9], 10), vec![0, 4]);
    }

    #[test]
    fn pair_at_end() {
        assert_eq!(two_sum_sorted(vec![1, 2, 3, 4, 5], 9), vec![3, 4]);
    }

    #[test]
    fn large_magnitude_values() {
        assert_eq!(
            two_sum_sorted(vec![-2_147_483_648, 0, 2_147_483_647], -1),
            vec![0, 2]
        );
    }

    // We can assume that there will only be one solution.
    //#[test]
    //fn repeated_middle_values() {
    //    let result = two_sum_sorted(vec![1, 2, 2, 2, 3], 4);
    //    assert!(
    //        result == vec![1, 2] || result == vec![1, 3] || result == vec![2, 3],
    //        "got unexpected result: {:?}",
    //        result
    //    );
    //}

    #[test]
    fn multiple_valid_answers_are_allowed() {
        let result = two_sum_sorted(vec![1, 2, 3, 4, 5], 6);
        assert!(
            result == vec![0, 4] || result == vec![1, 3],
            "got unexpected result: {:?}",
            result
        );
    }
}
