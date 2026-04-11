use std::collections::HashSet;
use std::error;
use std::io;

fn longest_substring_without_repeating_characters(s: String) -> i32 {

    fn has_repeats(s1: &str) -> bool {
        let mut char_flags = HashSet::new();

        for x in s1.chars() {
            if !char_flags.insert(x) {
                return false;
            }
        }

        true
    }


    let mut p1 = 0;
    let mut p2 = 0;
    let mut longest_nonrepeating = 0;

    while p2 <= s.len() {
        if has_repeats(&s[p1..p2]) {
            longest_nonrepeating = longest_nonrepeating.max(p2-p1);
            if p1 > 0 {
                p1 -= 1;
            } else {
                p2 += 1;
            }
        } else {
            p1 += 1;
            p2 += 1;
        }
    }

    longest_nonrepeating as i32
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    s = s.trim_end().to_string();
    let res = longest_substring_without_repeating_characters(s);
    println!("{}", res);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::longest_substring_without_repeating_characters;

    #[test]
    fn empty_string() {
        assert_eq!(
            longest_substring_without_repeating_characters("".to_string()),
            0
        );
    }

    #[test]
    fn single_character() {
        assert_eq!(
            longest_substring_without_repeating_characters("a".to_string()),
            1
        );
    }

    #[test]
    fn all_unique_characters() {
        assert_eq!(
            longest_substring_without_repeating_characters("abcdef".to_string()),
            6
        );
    }

    #[test]
    fn all_same_characters() {
        assert_eq!(
            longest_substring_without_repeating_characters("bbbbbb".to_string()),
            1
        );
    }

    #[test]
    fn basic_example() {
        assert_eq!(
            longest_substring_without_repeating_characters("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn repeated_character_forces_left_pointer_jump() {
        assert_eq!(
            longest_substring_without_repeating_characters("abba".to_string()),
            2
        );
    }

    #[test]
    fn duplicate_after_long_unique_run() {
        assert_eq!(
            longest_substring_without_repeating_characters("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn duplicate_immediately_repeated_then_unique_tail() {
        assert_eq!(
            longest_substring_without_repeating_characters("dvdf".to_string()),
            3
        );
    }

    #[test]
    fn longest_window_in_middle() {
        assert_eq!(
            longest_substring_without_repeating_characters("aabcbcdbca".to_string()),
            4
        );
    }

    #[test]
    fn longest_window_at_end() {
        assert_eq!(
            longest_substring_without_repeating_characters("bbbbcdef".to_string()),
            5
        );
    }

    #[test]
    fn alternating_repeats() {
        assert_eq!(
            longest_substring_without_repeating_characters("abababab".to_string()),
            2
        );
    }

    #[test]
    fn mixed_letters_and_digits() {
        assert_eq!(
            longest_substring_without_repeating_characters("a1b2c3a4".to_string()),
            7
        );
    }

    #[test]
    fn spaces_count_as_characters() {
        assert_eq!(
            longest_substring_without_repeating_characters("a b c a".to_string()),
            3
        );
    }

    #[test]
    fn case_sensitive_characters() {
        assert_eq!(
            longest_substring_without_repeating_characters("aAbBcC".to_string()),
            6
        );
    }
}
