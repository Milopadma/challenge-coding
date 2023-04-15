use std::{cell::RefCell, rc::Rc};

// inorder traversal is left -> root -> right
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;
    // iterative solution instead of recursive
    // while current is not None or stack is not empty
    while current.is_some() || !stack.is_empty() {
        // push all left nodes to stack
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }
        // pop the last left node
        current = stack.pop();
        // push the value of the node to result
        if let Some(node) = current {
            result.push(node.borrow().val);
            // set current to the right node
            current = node.borrow().right.clone();
        }
    }
    result
}
pub fn main() {}
