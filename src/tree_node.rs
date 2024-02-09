use std::{cell::RefCell, rc::Rc};

pub type RefTreeNode = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<RefTreeNode>,
    pub right: Option<RefTreeNode>,
}
