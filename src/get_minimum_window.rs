use std::collections::HashMap;
use std::error;
use std::io;

// Given two strings, original and check, return the shortest substring of original that contains
// every character in check, including duplicates. If multiple valid substrings have the same
// length, return the lexicographically smallest one.

#[derive(Default, Debug)]
struct State {
    check_count: HashMap<char, i32>,
    found_letter_count: HashMap<char, i32>,
    satisfied_letter_count: i32,
}

enum Change {
    Inc(),
    Dec(),
}

impl State {
    fn init_check_count(&mut self, ch: String) {
        for x in ch.chars() {
            self.check_count
                .entry(x)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    fn change_counts(&mut self, c: &char, change_type: Change) {
        let amount = match change_type {
            Change::Inc() => 1,
            Change::Dec() => -1,
        };

        let is_previously_satisfied = self.found_letter_count.get(c) >= self.check_count.get(c);

        self.found_letter_count
            .entry(*c)
            .and_modify(|counter| *counter += amount)
            .or_insert(1);

        let is_currently_satisfied = self.found_letter_count.get(c) >= self.check_count.get(c);

        if is_currently_satisfied != is_previously_satisfied {
            self.satisfied_letter_count += amount;
        }
    }

    fn add_letter(&mut self, c: &char) {
        self.change_counts(c, Change::Inc());
    }

    fn subtract_letter(&mut self, c: &char) {
        self.change_counts(c, Change::Dec());
    }

    fn is_satisfied(&self) -> bool {
        self.satisfied_letter_count == self.check_count.len() as i32
    }
}

// expand when unsatisfied
// shrink when satisfied
// end when r at end of string and l is
fn get_minimum_window(original: String, check: String) -> String {
    if check.len() > original.len() {
        return "".to_string();
    }

    let o: Vec<char> = original.chars().collect();
    let mut shortest_string: String = String::new();

    let mut state = State::default();
    state.init_check_count(check);

    let mut l = 0;
    let mut r = 0;

    while r < o.len() {
        state.add_letter(&o[r]);

        while state.is_satisfied() {
            let candidate = original[l..=r].to_string();
            if shortest_string.is_empty()
                || candidate.len() < shortest_string.len()
                || (candidate.len() == shortest_string.len() && candidate < shortest_string)
            {
                shortest_string = candidate;
            };

            state.subtract_letter(&o[l]);
            l += 1;
        }

        r += 1;
    }

    shortest_string
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut original = String::new();
    io::stdin().read_line(&mut original)?;
    original = original.trim_end().to_string();
    let mut check = String::new();
    io::stdin().read_line(&mut check)?;
    check = check.trim_end().to_string();
    let res = get_minimum_window(original, check);
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_minimum_window;

    #[test]
    fn equal_case() {
        let original = "abc".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "abc");
    }

    #[test]
    fn simple_case() {
        let original = "rcdbxay".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "cdbxa");
    }

    #[test]
    fn smaller_in_middle_case() {
        let original = "rcdbxcbayy".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "cba");
    }

    #[test]
    fn example_lexicographic_tiebreak() {
        let original = "cdbaebaecd".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "baec");
    }

    #[test]
    fn exact_match_whole_string() {
        let original = "abc".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "abc");
    }

    #[test]
    fn single_character_match() {
        let original = "zzza".to_string();
        let check = "a".to_string();
        assert_eq!(get_minimum_window(original, check), "a");
    }

    #[test]
    fn no_possible_window() {
        let original = "abcdef".to_string();
        let check = "xyz".to_string();
        assert_eq!(get_minimum_window(original, check), "");
    }

    #[test]
    fn minimum_window_at_start() {
        let original = "abcxxxx".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "abc");
    }

    #[test]
    fn minimum_window_at_end() {
        let original = "xxxxabc".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "abc");
    }

    #[test]
    fn duplicate_required_characters() {
        let original = "bbaaac".to_string();
        let check = "aba".to_string();
        assert_eq!(get_minimum_window(original, check), "baa");
    }

    #[test]
    fn duplicate_requirement_not_just_membership() {
        let original = "abcbcba".to_string();
        let check = "aabc".to_string();
        assert_eq!(get_minimum_window(original, check), "abcbcba");
    }

    #[test]
    fn case_sensitive_characters() {
        let original = "aAbBcC".to_string();
        let check = "ABC".to_string();
        assert_eq!(get_minimum_window(original, check), "AbBcC");
    }

    #[test]
    fn case_sensitive_valid_window() {
        let original = "xaAbBCy".to_string();
        let check = "AbB".to_string();
        assert_eq!(get_minimum_window(original, check), "AbB");
    }

    #[test]
    fn lexicographic_tiebreak_same_length() {
        let original = "bacaacba".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "acb");
    }

    #[test]
    fn repeated_irrelevant_characters_inside_window() {
        let original = "aaabdecfab".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "cfab");
    }

    #[test]
    fn check_longer_than_original() {
        let original = "ab".to_string();
        let check = "abc".to_string();
        assert_eq!(get_minimum_window(original, check), "");
    }
}
