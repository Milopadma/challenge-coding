use std::{cell::RefCell, rc::Rc};

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }
        current = stack.pop();
        if let Some(node) = current {
            result.push(node.borrow().val);
            current = node.borrow().right.clone();
        }
    }
    result
}
pub fn main() {}
