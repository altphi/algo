pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

fn visible_tree_node(root: Tree<i32>) -> i32 {
    fn rec_visible_tree(branch: Tree<i32>, max_so_far: i32) -> i32 {
        if let Some(b) = branch {
            let new_max_so_far: i32;
            let is_visible: i32;
            if b.val >= max_so_far {
                new_max_so_far = b.val;
                is_visible = 1;
            } else {
                new_max_so_far = max_so_far;
                is_visible = 0;
            }
            is_visible
                + rec_visible_tree(b.left, new_max_so_far)
                + rec_visible_tree(b.right, new_max_so_far)
        } else {
            0
        }
    }

    if let Some(r) = &root {
        let init_max = r.val;
        rec_visible_tree(root, init_max)
    } else {
        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Node, Tree, visible_tree_node};

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
    fn empty_tree_has_no_visible_nodes() {
        assert_eq!(visible_tree_node(None), 0);
    }

    #[test]
    fn single_node_tree_counts_the_root() {
        assert_eq!(visible_tree_node(leaf(10)), 1);
    }

    #[test]
    fn example_tree_counts_root_eight_and_six() {
        let root = node(5, node(4, leaf(3), leaf(8)), leaf(6));

        assert_eq!(visible_tree_node(root), 3);
    }

    #[test]
    fn strictly_increasing_path_counts_every_node() {
        let root = node(1, None, node(2, None, node(3, None, leaf(4))));

        assert_eq!(visible_tree_node(root), 4);
    }

    #[test]
    fn strictly_decreasing_path_counts_only_the_root() {
        let root = node(10, node(9, node(8, leaf(7), None), None), None);

        assert_eq!(visible_tree_node(root), 1);
    }

    #[test]
    fn equal_values_on_path_are_visible() {
        let root = node(5, node(5, leaf(5), leaf(4)), leaf(5));

        assert_eq!(visible_tree_node(root), 4);
    }

    #[test]
    fn negative_values_are_compared_against_ancestors_not_zero() {
        let root = node(-10, leaf(-20), node(-5, leaf(-5), leaf(-6)));

        assert_eq!(visible_tree_node(root), 3);
    }
}
