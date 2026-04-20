use std::error;

fn has_cycle(nodes: Vec<i32>) -> bool {
    let mut f: usize = 0;
    let mut s: usize = 0;

    while f < nodes.len() {
        if nodes[f] == f as i32 || nodes[s] == s as i32 {
            return true;
        }

        if nodes[f] as usize > nodes.len()-1 { return false; }
        f = nodes[f] as usize;

        if nodes[f] as usize > nodes.len()-1 { return false; }
        f = nodes[f] as usize;

        s = nodes[s] as usize;
        if f < nodes.len() && s >= f { return true; }
    }

    false
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::has_cycle;

    #[test]
    fn simple_cycle() {
        let nums = vec![1, 2, 3, 4, 5, 2];
        assert!(has_cycle(nums));
    }

    #[test]
    fn no_cycle_out_of_bounds() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert!(!has_cycle(nums));
    }

    #[test]
    fn cycle_to_self() {
        let nums = vec![0];
        assert!(has_cycle(nums));
    }

    #[test]
    fn cycle_early() {
        let nums = vec![1, 2, 1];
        assert!(has_cycle(nums));
    }

    #[test]
    fn dead_end_midway() {
        let nums = vec![2, 3, 10, 4];
        assert!(!has_cycle(nums));
    }

    #[test]
    fn larger_cycle() {
        let nums = vec![1, 3, 0, 4, 2];
        assert!(has_cycle(nums));
    }

    #[test]
    fn negative_index_breaks() {
        let nums = vec![1, -1, 2];
        assert!(!has_cycle(nums));
    }

    #[test]
    fn single_no_cycle() {
        let nums = vec![1];
        assert!(!has_cycle(nums));
    }
}
