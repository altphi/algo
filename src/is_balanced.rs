pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn is_balanced(tree: Tree<i32>) -> bool {
    fn rec_cf_lengths(branch: Tree<i32>) -> Option<i32> {
        if let Some(b) = branch {
            let left = rec_cf_lengths(b.left);
            let right = rec_cf_lengths(b.right);

            if let (Some(l), Some(r)) = (left.as_ref(), right.as_ref()) {
                let max = l.max(r);
                let min = l.min(r);
                if max - min > 1 { None } else { Some(1 + *max) }
            } else {
                None
            }
        } else {
            Some(1)
        }
    }

    rec_cf_lengths(tree).is_some()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Node, Tree, is_balanced};

    fn leaf(val: i32) -> Tree<i32> {
        Some(Box::new(Node {
            val,
            left: None,
            right: None,
        }))
    }

    fn node(val: i32, left: Tree<i32>, right: Tree<i32>) -> Tree<i32> {
        Some(Box::new(Node { val, left, right }))
    }

    #[test]
    fn empty_tree_is_balanced() {
        assert!(is_balanced(None));
    }

    #[test]
    fn single_node_tree_is_balanced() {
        assert!(is_balanced(leaf(1)));
    }

    #[test]
    fn one_missing_leaf_level_is_still_balanced() {
        let tree = node(1, leaf(2), None);

        assert!(is_balanced(tree));
    }

    #[test]
    fn example_one_is_balanced() {
        let tree = node(
            1,
            node(2, node(4, None, leaf(7)), leaf(5)),
            node(3, None, leaf(6)),
        );

        assert!(is_balanced(tree));
    }

    #[test]
    fn example_two_is_not_balanced() {
        let tree = node(
            1,
            node(2, node(4, None, leaf(7)), leaf(5)),
            node(3, None, node(6, leaf(8), None)),
        );

        assert!(!is_balanced(tree));
    }

    #[test]
    fn root_height_difference_greater_than_one_is_not_balanced() {
        let tree = node(1, node(2, node(3, leaf(4), None), None), None);

        assert!(!is_balanced(tree));
    }

    #[test]
    fn imbalance_below_root_is_detected() {
        let tree = node(
            10,
            node(5, node(3, leaf(1), None), None),
            node(15, leaf(12), None),
        );

        assert!(!is_balanced(tree));
    }
}
