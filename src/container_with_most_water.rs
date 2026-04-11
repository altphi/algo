
use std::error;
use std::io;
use std::str::FromStr;

fn container_with_most_water(arr: Vec<i32>) -> i32 {
    let mut p1 = 0;
    let mut p2 = arr.len()-1;
    let mut highest_area: i32 = 0;

    fn calc_area(h1: i32, h2: i32, d: i32) -> i32 {
        let lower = h2.min(h1);
        lower * d
    }

    while p2 > p1 {
        let area = calc_area(arr[p1], arr[p2], (p2 - p1) as i32);
        highest_area = highest_area.max(area);

        if arr[p2] > arr[p1] {
            p1 += 1;
        } else {
            p2 -= 1;
        }
    }

    highest_area
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let arr = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let res = container_with_most_water(arr);
    println!("{}", res);
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::container_with_most_water;

    #[test]
    fn basic_example() {
        let arr = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(container_with_most_water(arr), 49);
    }

    #[test]
    fn minimal_case_two_elements() {
        let arr = vec![1, 1];
        assert_eq!(container_with_most_water(arr), 1);
    }

    #[test]
    fn increasing_heights() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(container_with_most_water(arr), 6);
    }

    #[test]
    fn decreasing_heights() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(container_with_most_water(arr), 6);
    }

    #[test]
    fn all_same_height() {
        let arr = vec![3, 3, 3, 3];
        assert_eq!(container_with_most_water(arr), 9);
    }

    #[test]
    fn contains_zeroes() {
        let arr = vec![0, 2, 0, 4, 0];
        assert_eq!(container_with_most_water(arr), 4);
    }

    #[test]
    fn wide_short_vs_narrow_tall() {
        let arr = vec![1, 100, 1, 1, 100, 1];
        assert_eq!(container_with_most_water(arr), 300);
    }

    #[test]
    fn peak_in_middle() {
        let arr = vec![1, 2, 10, 2, 1];
        assert_eq!(container_with_most_water(arr), 4);
    }

    #[test]
    fn large_values() {
        let arr = vec![10000, 1, 10000];
        assert_eq!(container_with_most_water(arr), 20000);
    }

    #[test]
    fn alternating_heights() {
        let arr = vec![1, 3, 2, 5, 25, 24, 5];
        assert_eq!(container_with_most_water(arr), 24);
    }
}
