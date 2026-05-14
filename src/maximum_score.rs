use std::error;
use std::io;
use std::str::FromStr;

fn maximum_score(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut total = 0;
    let mut segment_tot1: i64 = 0;
    let mut segment_tot2: i64 = 0;

    while p1 < arr1.len() && p2 < arr2.len() {
        let val1 = arr1.get(p1).unwrap_or(&0);
        let val2 = arr2.get(p2).unwrap_or(&0);

        if val1 > val2 {
            segment_tot2 += *val2 as i64;
            p2 += 1;
        } else if val1 < val2 {
            segment_tot1 += *val1 as i64;
            p1 += 1;
        } else {
            // we found a portal!
            total += segment_tot1.max(segment_tot2);
            total += *val1 as i64;  // add the portal value itself, once

            segment_tot1 = 0;
            segment_tot2 = 0;
            p1 += 1;
            p2 += 1;
        }
    }

    while p1 < arr1.len() {
        segment_tot1 += *(arr1.get(p1).unwrap_or(&0)) as i64;
        p1 += 1;
    }
    while p2 < arr2.len() {
        segment_tot2 += *(arr2.get(p2).unwrap_or(&0)) as i64;
        p2 += 1;
    }

    total += segment_tot1.max(segment_tot2);

    (total % 1_000_000_007) as i32
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let arr1 = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let arr2 = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let res = maximum_score(arr1, arr2);
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(maximum_score(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]), 30);
    }

    #[test]
    fn switches_to_take_larger_middle_segment() {
        assert_eq!(maximum_score(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
    }

    #[test]
    fn no_common_values_choose_larger_total_array() {
        assert_eq!(maximum_score(vec![1, 2, 3], vec![4, 5, 6]), 15);
    }

    #[test]
    fn arrays_are_identical_counts_each_value_once() {
        assert_eq!(maximum_score(vec![1, 2, 3, 4], vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn common_value_at_beginning() {
        assert_eq!(maximum_score(vec![1, 2, 10], vec![1, 3, 4]), 13);
    }

    #[test]
    fn common_value_at_end() {
        assert_eq!(maximum_score(vec![1, 2, 9], vec![3, 4, 9]), 16);
    }

    #[test]
    fn multiple_crossovers_choose_best_segment_each_time() {
        assert_eq!(
            maximum_score(vec![1, 4, 5, 6, 8, 10], vec![2, 4, 6, 7, 8, 9]),
            42
        );
    }

    #[test]
    fn one_array_has_large_tail_after_last_common_value() {
        assert_eq!(maximum_score(vec![1, 3, 5], vec![1, 3, 5, 100, 200]), 309);
    }

    #[test]
    fn handles_single_element_arrays_same_value() {
        assert_eq!(maximum_score(vec![7], vec![7]), 7);
    }

    #[test]
    fn handles_single_element_arrays_different_values() {
        assert_eq!(maximum_score(vec![7], vec![11]), 11);
    }

    #[test]
    fn result_is_modded() {
        assert_eq!(
            maximum_score(
                vec![1_000_000_000, 1_000_000_001],
                vec![1_000_000_000, 1_000_000_002]
            ),
            999_999_995
        );
    }

    #[test]
    fn handles_max_allowed_values_without_overflow() {
        assert_eq!(
            maximum_score(
                vec![9_999_998, 9_999_999, 10_000_000],
                vec![9_999_997, 9_999_999, 10_000_000]
            ),
            29_999_997
        );
    }

    #[test]
    fn large_arrays_no_common_values_choose_larger_sum() {
        let arr1: Vec<i32> = (1..=50_000).collect();
        let arr2: Vec<i32> = (50_001..=100_000).collect();

        // sum 50_001..=100_000 = 3_750_025_000
        // mod 1_000_000_007 = 750_024_979
        assert_eq!(maximum_score(arr1, arr2), 750_024_979);
    }

    #[test]
    fn large_arrays_all_common_values_count_once() {
        let arr1: Vec<i32> = (1..=50_000).collect();
        let arr2: Vec<i32> = (1..=50_000).collect();

        // sum 1..=50_000 = 1_250_025_000
        // mod 1_000_000_007 = 250_024_993
        assert_eq!(maximum_score(arr1, arr2), 250_024_993);
    }

    #[test]
    fn large_arrays_with_regular_crossovers() {
        let arr1: Vec<i32> = (1..=50_000).collect();
        let arr2: Vec<i32> = (25_000..=74_999).collect();

        // Best path is:
        // arr1: 1..=24_999
        // shared/common path through 25_000..=50_000
        // arr2 tail: 50_001..=74_999
        //
        // Equivalent to sum 1..=74_999
        // = 2_812_462_500
        // mod 1_000_000_007 = 812_462_486
        assert_eq!(maximum_score(arr1, arr2), 812_462_486);
    }
}
