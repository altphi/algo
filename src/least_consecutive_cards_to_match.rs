use std::collections::HashSet;
use std::error;
use std::io;
use std::str::FromStr;

// expand window until a double is 'in hand'
// shrink window until double no longer in hand
fn least_consecutive_cards_to_match(cards: Vec<i32>) -> i32 {
    if cards.len() <= 1 { return -1; }

    let mut shortest_to_match = usize::MAX;

    let mut p1 = 0;
    let mut p2 = 1;
    let mut cards_in_hand: HashSet<i32> = HashSet::new();
    cards_in_hand.insert(cards[p1]);

    while p2 < cards.len() {
        let mut is_double = !cards_in_hand.insert(cards[p2]);
        while is_double && p1 < p2 {
            if cards[p1] == cards[p2] {
                shortest_to_match = shortest_to_match.min(p2-p1 + 1);
                is_double = false;
            } else {
                cards_in_hand.remove(&cards[p1]);
            }
            p1 += 1;
        }

        p2 += 1;
    }

    if shortest_to_match == usize::MAX {
        -1
    } else {
        shortest_to_match as i32
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let cards = line
        .split_whitespace()
        .map(i32::from_str)
        .collect::<Result<_, _>>()?;
    let res = least_consecutive_cards_to_match(cards);
    println!("{}", res);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::least_consecutive_cards_to_match;

    #[test]
    fn example_case() {
        let cards = vec![3, 4, 2, 3, 4, 7];
        assert_eq!(least_consecutive_cards_to_match(cards), 4);
    }

    #[test]
    fn no_matching_pairs() {
        let cards = vec![1, 2, 3, 4, 5];
        assert_eq!(least_consecutive_cards_to_match(cards), -1);
    }

    #[test]
    fn adjacent_match() {
        let cards = vec![8, 8, 3, 4];
        assert_eq!(least_consecutive_cards_to_match(cards), 2);
    }

    #[test]
    fn match_at_start() {
        let cards = vec![5, 1, 5, 2, 3];
        assert_eq!(least_consecutive_cards_to_match(cards), 3);
    }

    #[test]
    fn match_at_end() {
        let cards = vec![1, 2, 3, 4, 3];
        assert_eq!(least_consecutive_cards_to_match(cards), 3);
    }

    #[test]
    fn multiple_matches_take_smallest_window() {
        let cards = vec![1, 2, 3, 1, 2, 3, 2];
        assert_eq!(least_consecutive_cards_to_match(cards), 3);
    }

    #[test]
    fn repeated_same_value_many_times() {
        let cards = vec![7, 1, 7, 2, 7];
        assert_eq!(least_consecutive_cards_to_match(cards), 3);
    }

    #[test]
    fn triple_adjacent_same_value() {
        let cards = vec![4, 4, 4];
        assert_eq!(least_consecutive_cards_to_match(cards), 2);
    }

    #[test]
    fn single_card_only() {
        let cards = vec![9];
        assert_eq!(least_consecutive_cards_to_match(cards), -1);
    }

    #[test]
    fn two_cards_same() {
        let cards = vec![6, 6];
        assert_eq!(least_consecutive_cards_to_match(cards), 2);
    }

    #[test]
    fn two_cards_different() {
        let cards = vec![6, 7];
        assert_eq!(least_consecutive_cards_to_match(cards), -1);
    }

    #[test]
    fn later_match_is_better_than_earlier_match() {
        let cards = vec![1, 2, 3, 1, 4, 4];
        assert_eq!(least_consecutive_cards_to_match(cards), 2);
    }

    #[test]
    fn large_gap_match() {
        let cards = vec![9, 1, 2, 3, 4, 5, 9];
        assert_eq!(least_consecutive_cards_to_match(cards), 7);
    }

    #[test]
    fn zero_values_allowed() {
        let cards = vec![0, 2, 0, 3];
        assert_eq!(least_consecutive_cards_to_match(cards), 3);
    }
}

