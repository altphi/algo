
use std::collections::HashMap;
use std::{char, error};
use std::fmt::Display;
use std::io;

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {

    fn is_anagram(s1: &Vec<char>, s2: &Vec<char>) -> bool {
        let mut char_counts: HashMap<char, i32> = HashMap::new();

        if s1.len() != s2.len() {
            return false;
        }

        for x in s1 {
            char_counts.entry(*x).and_modify(|count| *count += 1).or_insert(1);
        }

        for x in s2 {
            char_counts.entry(*x).and_modify(|count| *count -= 1).or_insert(-1);
        }

        let mut all_values_zero = true;
        for x in char_counts.values() {
            if *x != 0 { all_values_zero = false; }
        }

        all_values_zero
    }

    let original_chars: Vec<char> = original.chars().collect();
    let check_chars: Vec<char> = check.chars().collect();
    let mut indices_of_anagrams: Vec<i32> = Vec::new();

    for (i, w) in original_chars.windows(check_chars.len()).enumerate() {
        if is_anagram(w.to_vec().as_ref(), &check_chars) {
            indices_of_anagrams.push(i as i32);
        }
    }

    indices_of_anagrams
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
    let mut original = String::new();
    io::stdin().read_line(&mut original)?;
    original = original.trim_end().to_string();
    let mut check = String::new();
    io::stdin().read_line(&mut check)?;
    check = check.trim_end().to_string();
    let res = find_all_anagrams(original, check);
    print_words(&res);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::find_all_anagrams;

    #[test]
    fn example_basic_case() {
        let result = find_all_anagrams("cbaebabacd".to_string(), "abc".to_string());
        assert_eq!(result, vec![0, 6]);
    }

    #[test]
    fn overlapping_anagrams() {
        let result = find_all_anagrams("abab".to_string(), "ab".to_string());
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn no_matches() {
        let result = find_all_anagrams("abcdef".to_string(), "gh".to_string());
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn whole_string_is_anagram() {
        let result = find_all_anagrams("bca".to_string(), "abc".to_string());
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn repeated_letters_in_pattern() {
        let result = find_all_anagrams("aaabaa".to_string(), "aab".to_string());
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn repeated_letters_many_overlaps() {
        let result = find_all_anagrams("aaaaa".to_string(), "aa".to_string());
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn pattern_longer_than_original() {
        let result = find_all_anagrams("ab".to_string(), "abc".to_string());
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn single_character_pattern() {
        let result = find_all_anagrams("banana".to_string(), "a".to_string());
        assert_eq!(result, vec![1, 3, 5]);
    }

    #[test]
    fn match_at_end_only() {
        let result = find_all_anagrams("xxxxcab".to_string(), "abc".to_string());
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn all_windows_same_multiset() {
        let result = find_all_anagrams("abcabc".to_string(), "bca".to_string());
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn distinguish_counts_not_just_membership() {
        let result = find_all_anagrams("abbcabc".to_string(), "abb".to_string());
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn empty_result_when_letters_present_but_counts_wrong() {
        let result = find_all_anagrams("aaacb".to_string(), "aab".to_string());
        assert_eq!(result, Vec::<i32>::new());
    }
}
