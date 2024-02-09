mod tree_node;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut node_1 = tree_node::TreeNode {
        val: 220,
        left: None,
        right: None,
    };
    let node_2 = tree_node::TreeNode {
        val: 13,
        left: None,
        right: None,
    };
    let node_3 = tree_node::TreeNode {
        val: 123,
        left: None,
        right: None,
    };
    node_1.left = Some(Rc::new(RefCell::new(node_2)));
    node_1.right = Some(Rc::new(RefCell::new(node_3)));

    println!(
        "sum of tree is {}",
        tree_sum(Some(&Rc::new(RefCell::new(node_1))))
    );
}

fn tree_sum(root: Option<&tree_node::RefTreeNode>) -> i32 {
    if let Some(root) = root {
        root.borrow().val
            + tree_sum(root.borrow().left.as_ref())
            + tree_sum(root.borrow().right.as_ref())
    } else {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use tree_node::TreeNode;

    #[test]

    fn test_tree_sum() {
        let mut node_1 = TreeNode {
            val: 20,
            left: None,
            right: None,
        };
        let mut node_2 = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let mut node_3 = TreeNode {
            val: 23,
            left: None,
            right: None,
        };
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        assert_eq!(tree_sum(Some(&Rc::new(RefCell::new(node_1)))), 46);
    }

    #[test]
    fn test_tree_sum_none() {
        assert_eq!(tree_sum(None), 0);
    }
}
