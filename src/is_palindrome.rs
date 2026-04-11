use std::error;
use std::io;

fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    if chars.len() <= 1 {
        return true;
    }

    let mut p1 = 0;
    let mut p2 = chars.len()-1;

    while p1 <= p2 {
        if chars[p2] != chars[p1] {
            return false;
        }

        p1 += 1;
        p2 -= 1;
    }

    true
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    s = s.trim_end().to_string();
    let res = is_palindrome(s);
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_palindrome() {
        assert_eq!(is_palindrome("racecar".to_string()), true);
    }

    #[test]
    fn simple_non_palindrome() {
        assert_eq!(is_palindrome("hello".to_string()), false);
    }

    #[test]
    fn single_character() {
        assert!(is_palindrome("a".to_string()));
    }

    #[test]
    fn empty_string() {
        assert_eq!(is_palindrome("".to_string()), true);
    }

    #[test]
    fn even_length_palindrome() {
        assert_eq!(is_palindrome("abba".to_string()), true);
    }

    #[test]
    fn even_length_non_palindrome() {
        assert_eq!(is_palindrome("abca".to_string()), false);
    }

    #[test]
    fn with_spaces() {
        assert_eq!(is_palindrome("race car".to_string()), false);
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(is_palindrome("Racecar".to_string()), true);
    }

    #[test]
    fn numeric_palindrome() {
        assert_eq!(is_palindrome("12321".to_string()), true);
    }

    #[test]
    fn numeric_non_palindrome() {
        assert_eq!(is_palindrome("12345".to_string()), false);
    }

    #[test]
    fn special_characters() {
        assert_eq!(is_palindrome("!@##@!".to_string()), true);
    }

    #[test]
    fn long_palindrome() {
        let s = "a".repeat(1000);
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn long_non_palindrome() {
        let mut s = "a".repeat(999);
        s.push('b');
        assert_eq!(is_palindrome(s), false);
    }
}
