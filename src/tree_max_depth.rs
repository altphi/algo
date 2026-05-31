pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn tree_max_depth(root: Tree<i32>) -> i32 {
    fn rec_depth_check(branch: Tree<i32>, depth: i32) -> i32 {
        if let Some(n) = branch {
          let new_depth = depth + 1;
          rec_depth_check(n.left, new_depth).max(rec_depth_check(n.right, new_depth))
        } else {
          (depth-1).max(0)  // there are 2 zeros, oddly, in this challenge, so if we go negative, return 0
        }
    }

    rec_depth_check(root, 0)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{tree_max_depth, Node, Tree};

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
    fn empty_tree_has_depth_zero() {
        assert_eq!(tree_max_depth(None), 0);
    }

    #[test]
    fn single_node_tree_has_depth_zero() {
        assert_eq!(tree_max_depth(leaf(1)), 0);
    }

    #[test]
    fn balanced_tree_counts_edges_from_root() {
        let root = node(
            1,
            node(2, leaf(4), leaf(5)),
            node(3, leaf(6), leaf(7)),
        );

        assert_eq!(tree_max_depth(root), 2);
    }

    #[test]
    fn left_skewed_tree_uses_longest_path() {
        let root = node(1, node(2, node(3, node(4, leaf(5), None), None), None), None);

        assert_eq!(tree_max_depth(root), 4);
    }

    #[test]
    fn uneven_tree_uses_deeper_branch() {
        let root = node(
            1,
            node(2, leaf(4), None),
            node(3, None, node(5, node(6, leaf(7), None), None)),
        );

        assert_eq!(tree_max_depth(root), 4);
    }

    #[test]
    fn site_example_depth_is_two_edges() {
        // Preorder input: 5 4 3 x x 8 x x 6 x x
        let root = node(5, node(4, leaf(3), leaf(8)), leaf(6));

        assert_eq!(tree_max_depth(root), 2);
    }
}
