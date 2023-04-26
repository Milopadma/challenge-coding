use std::{cell::RefCell, rc::Rc};

// this approach subtracts the value of the node from the target sum instead of
// adding the value of each iterative node and compared to the target sum
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    // put tree to stack
    let mut stack = vec![];
    if let Some(node) = root {
        stack.push((node, target_sum));
    }
    // iterate over the stack and find the path and sum
    while let Some((node, target_sum)) = stack.pop() {
        let node = node.borrow();
        let val = node.val;
        if node.left.is_none() && node.right.is_none() && val == target_sum {
            return true;
        }
        if let Some(left) = &node.left {
            // we - val because we are going down the tree if we reach the leaf
            // node and the sum is equal to target_sum, we return true
            stack.push((left.clone(), target_sum - val));
        }
        if let Some(right) = &node.right {
            stack.push((right.clone(), target_sum - val));
        }
    }
    // if it reaches here, it means there is no path
    false
}
