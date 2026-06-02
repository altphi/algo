pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn subtree_of_another_tree(root: Tree<i32>, sub_root: Tree<i32>) -> bool {
    fn is_same_tree(a_tree: &Tree<i32>, b_tree: &Tree<i32>) -> bool {
        match (a_tree, b_tree) {
            (None, None) => true,
            (Some(a_node), Some(b_node)) => {
            a_node.val == b_node.val
                && is_same_tree(&a_node.left, &b_node.left)
                && is_same_tree(&a_node.right, &b_node.right)
            }
            _ => false
        }
    }

    fn travroot(r: &Tree<i32>, s: &Tree<i32>) -> bool {
        if is_same_tree(r, s) {
            true
        } else {
            if let Some(r1) = r {
                travroot(&r1.left, s) || travroot(&r1.right, s)
            } else {
                false
            }
        }
    }

    travroot(&root, &sub_root)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Node, Tree, subtree_of_another_tree};

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
    fn empty_subtree_is_subtree_of_empty_tree() {
        assert!(subtree_of_another_tree(None, None));
    }

    #[test]
    fn empty_subtree_is_subtree_of_non_empty_tree() {
        let root = node(1, leaf(2), leaf(3));

        assert!(subtree_of_another_tree(root, None));
    }

    #[test]
    fn non_empty_subtree_is_not_subtree_of_empty_tree() {
        assert!(!subtree_of_another_tree(None, leaf(1)));
    }

    #[test]
    fn identical_single_node_trees_match() {
        assert!(subtree_of_another_tree(leaf(7), leaf(7)));
    }

    #[test]
    fn example_subtree_matches() {
        let root = node(3, node(4, leaf(1), leaf(2)), leaf(5));
        let sub_root = node(4, leaf(1), leaf(2));

        assert!(subtree_of_another_tree(root, sub_root));
    }

    #[test]
    fn identical_full_tree_matches() {
        let root = node(3, node(4, leaf(1), leaf(2)), leaf(5));
        let sub_root = node(3, node(4, leaf(1), leaf(2)), leaf(5));

        assert!(subtree_of_another_tree(root, sub_root));
    }

    #[test]
    fn matching_values_with_extra_descendant_do_not_match() {
        let root = node(3, node(4, leaf(1), node(2, leaf(0), None)), leaf(5));
        let sub_root = node(4, leaf(1), leaf(2));

        assert!(!subtree_of_another_tree(root, sub_root));
    }

    #[test]
    fn matching_values_with_different_shape_do_not_match() {
        let root = node(1, node(2, leaf(3), None), None);
        let sub_root = node(2, None, leaf(3));

        assert!(!subtree_of_another_tree(root, sub_root));
    }

    #[test]
    fn duplicate_values_can_match_later_subtree() {
        let root = node(
            1,
            node(2, leaf(4), None),
            node(2, node(4, leaf(8), None), leaf(5)),
        );
        let sub_root = node(2, node(4, leaf(8), None), leaf(5));

        assert!(subtree_of_another_tree(root, sub_root));
    }
}
