use std::{cell::RefCell, rc::Rc};

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // check if the root is none
    if root.is_none() {
        return 0;
    }
    // set max and stack to 0 and root in a vector
    let mut max = 0;
    let mut stack = vec![(root, 1)];
    // now iterate over the stack of root
    while let Some((node, depth)) = stack.pop() {
        if let Some(node) = node {
            // if the node is not none, then set max to the max of max and depth
            max = max.max(depth);
            // then push the left and right nodes to the stack
            stack.push((node.borrow().left.clone(), depth + 1));
            stack.push((node.borrow().right.clone(), depth + 1));
        }
    }
    max
}
pub fn main() {}
