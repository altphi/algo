use std::error;
use std::io;
use std::str::FromStr;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn valid_bst(root: Tree<i32>) -> bool {
    fn check(t: Tree<i32>, max: Option<i32>, min: Option<i32>) -> bool {
        if let Some(n) = t {
            if max.is_some() && n.val >= max.unwrap() || min.is_some() && n.val <= min.unwrap() {
                false
            } else {
                check(n.left, Some(n.val), min) && check(n.right, max, Some(n.val))
            }
        } else {
            true
        }
    }

    check(root, None, None)
}

fn build_tree<'a, T, I>(nodes: &mut I) -> Result<Tree<T>, Box<dyn error::Error>>
where
    T: FromStr,
    I: Iterator<Item = &'a str>,
    <T as FromStr>::Err: 'static + error::Error,
{
    let val = match nodes.next().ok_or("eol")? {
        "x" => return Ok(None),
        v => v.parse()?,
    };
    let left = build_tree(nodes)?;
    let right = build_tree(nodes)?;
    Ok(Some(Box::new(Node { val, left, right })))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let root = build_tree(&mut line.split_whitespace())?;
    let res = valid_bst(root);
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to easily parse serializations into a tree for testing
    fn parse_tree(s: &str) -> Tree<i32> {
        let mut tokens = s.split_whitespace();
        build_tree(&mut tokens).expect("Failed to build tree from test input")
    }

    #[test]
    fn test_empty_tree() {
        // An empty tree is technically a valid BST
        let tree = parse_tree("x");
        assert!(valid_bst(tree));
    }

    #[test]
    fn test_single_node() {
        // A single node is always a valid BST
        let tree = parse_tree("10 x x");
        assert!(valid_bst(tree));
    }

    #[test]
    fn test_simple_valid_bst() {
        //      10
        //     /  \
        //    5    15
        let tree = parse_tree("10 5 x x 15 x x");
        assert!(valid_bst(tree));
    }

    #[test]
    fn test_invalid_left_child() {
        // Left child must be strictly less than root
        //      10
        //     /  \
        //   12    15
        let tree = parse_tree("10 12 x x 15 x x");
        assert!(!valid_bst(tree));
    }

    #[test]
    fn test_invalid_right_child() {
        // Right child must be strictly greater than root
        //      10
        //     /  \
        //    5    8
        let tree = parse_tree("10 5 x x 8 x x");
        assert!(!valid_bst(tree));
    }

    #[test]
    fn test_equal_values_fail() {
        // BSTs typically do not allow duplicate keys (or left/right rules must be strict).
        // Standard definition requires left < root < right.
        //      10
        //     /  \
        //   10    15
        let tree = parse_tree("10 10 x x 15 x x");
        assert!(!valid_bst(tree));
    }

    #[test]
    fn test_deep_sub_tree_invalid_ancestor_constraint() {
        // This tree is truly INVALID because 11 is in the
        // left subtree of 10, meaning it violates the root's constraint (11 > 10).
        // Local check (11 > 5) passes, but global check fails.
        //        10
        //       /  \
        //      5    15
        //     / \
        //    x   11
        let tree = parse_tree("10 5 x 11 x x 15 x x");
        assert!(!valid_bst(tree));
    }

    #[test]
    fn test_deep_sub_tree_valid() {
        // A deeper, perfectly valid BST.
        //        10
        //       /  \
        //      5    15
        //     / \   / \
        //    3   8 12  20
        let tree = parse_tree("10 5 3 x x 8 x x 15 12 x x 20 x x");
        assert!(valid_bst(tree));
    }
}
