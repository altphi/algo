use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn insert_bst(mut bst: Tree<i32>, val: i32) -> Tree<i32> {
    let mut bst_ptr = &mut bst;

    while let Some(n) = bst_ptr {
        if val == n.val {
            return bst;
        }
        if val < n.val {
            bst_ptr = &mut n.left;
        } else {
            bst_ptr = &mut n.right;
        }
    }

    *bst_ptr = Some(Box::new(Node {
        val,
        left: None,
        right: None,
    }));

    bst
}

// this function builds a tree from input
// learn more about how trees are encoded in https://algo.monster/problems/serializing_tree
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

fn format_tree<T: Display>(node: &Tree<T>, out: &mut Vec<String>) {
    match node {
        None => out.push("x".to_string()),
        Some(node) => {
            out.push(format!("{}", node.val));
            format_tree(&node.left, out);
            format_tree(&node.right, out);
        }
    }
}

fn print_words<T: Display>(v: &[T]) {
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
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let bst = build_tree(&mut line.split_whitespace())?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let val = line.trim_end().parse()?;
    let res = insert_bst(bst, val);
    let mut out = Vec::new();
    format_tree(&res, &mut out);
    print_words(&out);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_from_str(s: &str) -> Tree<i32> {
        build_tree(&mut s.split_whitespace()).unwrap()
    }

    fn tree_to_str(t: &Tree<i32>) -> String {
        let mut out = Vec::new();
        format_tree(t, &mut out);
        out.join(" ")
    }

    #[test]
    fn inserts_into_empty_tree() {
        let bst = tree_from_str("x");
        let result = insert_bst(bst, 5);
        assert_eq!(tree_to_str(&result), "5 x x");
    }

    #[test]
    fn inserts_as_left_leaf() {
        //     5
        //    / \
        //   3   8
        let bst = tree_from_str("5 3 x x 8 x x");
        let result = insert_bst(bst, 2);

        assert_eq!(tree_to_str(&result), "5 3 2 x x x 8 x x");
    }

    #[test]
    fn inserts_as_right_leaf() {
        let bst = tree_from_str("5 3 x x 8 x x");
        let result = insert_bst(bst, 10);

        assert_eq!(tree_to_str(&result), "5 3 x x 8 x 10 x x");
    }

    #[test]
    fn inserts_deep_left_right() {
        // Insert 4 under 3's right
        let bst = tree_from_str("5 3 2 x x x 8 x x");
        let result = insert_bst(bst, 4);

        assert_eq!(tree_to_str(&result), "5 3 2 x x 4 x x 8 x x");
    }

    #[test]
    fn inserts_deep_right_left() {
        // Insert 7 under 8's left
        let bst = tree_from_str("5 3 x x 8 x 10 x x");
        let result = insert_bst(bst, 7);

        assert_eq!(tree_to_str(&result), "5 3 x x 8 7 x x 10 x x");
    }

    #[test]
    fn duplicate_root_does_not_insert() {
        let bst = tree_from_str("5 3 x x 8 x x");
        let result = insert_bst(bst, 5);

        assert_eq!(tree_to_str(&result), "5 3 x x 8 x x");
    }

    #[test]
    fn duplicate_leaf_does_not_insert() {
        let bst = tree_from_str("5 3 x x 8 x x");
        let result = insert_bst(bst, 8);

        assert_eq!(tree_to_str(&result), "5 3 x x 8 x x");
    }

    #[test]
    fn handles_negative_values() {
        let bst = tree_from_str("0 -10 x x 10 x x");
        let result = insert_bst(bst, -5);

        assert_eq!(tree_to_str(&result), "0 -10 x -5 x x 10 x x");
    }
}
