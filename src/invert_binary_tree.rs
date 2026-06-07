pub type Tree<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn invert_binary_tree(tree: Tree<i32>) -> Tree<i32> {
    fn swap_node(tree_node: Tree<i32>) -> Tree<i32> {
        if let Some(t) = tree_node {
            return Some(Box::new(Node {
                val: t.val,
                left: swap_node(t.right),
                right: swap_node(t.left),
            }));
        }
        None
    }

    swap_node(tree)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Node, Tree, invert_binary_tree};

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

    fn assert_same_tree(actual: &Tree<i32>, expected: &Tree<i32>) {
        match (actual, expected) {
            (None, None) => {}
            (Some(actual), Some(expected)) => {
                assert_eq!(actual.val, expected.val);
                assert_same_tree(&actual.left, &expected.left);
                assert_same_tree(&actual.right, &expected.right);
            }
            (None, Some(expected)) => {
                panic!("expected node {}, but actual tree was empty", expected.val);
            }
            (Some(actual), None) => {
                panic!("expected empty tree, but actual node was {}", actual.val);
            }
        }
    }

    #[test]
    fn empty_tree_inverts_to_empty_tree() {
        assert_same_tree(&invert_binary_tree(None), &None);
    }

    #[test]
    fn single_node_tree_is_unchanged() {
        let actual = invert_binary_tree(leaf(1));
        let expected = leaf(1);

        assert_same_tree(&actual, &expected);
    }

    #[test]
    fn root_children_are_swapped() {
        let tree = node(1, leaf(2), leaf(3));
        let expected = node(1, leaf(3), leaf(2));

        assert_same_tree(&invert_binary_tree(tree), &expected);
    }

    #[test]
    fn complete_tree_is_mirrored_at_every_level() {
        let tree = node(4, node(2, leaf(1), leaf(3)), node(7, leaf(6), leaf(9)));
        let expected = node(4, node(7, leaf(9), leaf(6)), node(2, leaf(3), leaf(1)));

        assert_same_tree(&invert_binary_tree(tree), &expected);
    }

    #[test]
    fn uneven_tree_preserves_values_while_mirroring_shape() {
        let tree = node(1, node(2, leaf(3), leaf(4)), leaf(5));
        let expected = node(1, leaf(5), node(2, leaf(4), leaf(3)));

        assert_same_tree(&invert_binary_tree(tree), &expected);
    }

    #[test]
    fn left_skewed_tree_becomes_right_skewed() {
        let tree = node(1, node(2, node(3, leaf(4), None), None), None);
        let expected = node(1, None, node(2, None, node(3, None, leaf(4))));

        assert_same_tree(&invert_binary_tree(tree), &expected);
    }
}
