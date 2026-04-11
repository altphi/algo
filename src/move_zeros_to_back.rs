

pub fn move_zeros(nums: &mut Vec<i32>) {
    let mut p1: usize = 0;

    while nums[p1] != 0 { p1 += 1; }

    for p2 in p1+1..nums.len() {
        if nums[p2] != 0 {
          nums.swap(p1, p2);
          p1 += 1;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut input: Vec<i32> = vec![0, 1, 2, 0, 7];
        move_zeros(&mut input);
        assert_eq!(input, vec![1, 2, 7, 0, 0]);
    }
}

