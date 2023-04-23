use std::{cell::RefCell, rc::Rc};

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // to check if the tree is balanced, we need to check the height of the left and right subtree
    // if the height of the left and right subtree is not more than 1, then the tree is balanced

    // if the root is None, then the tree is balanced
    if root.is_none() {
        return true;
    }

    fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // if the root is None, then the height is 0
        if root.is_none() {
            return 0;
        }

        // get the height of the left and right subtree
        let left_height = get_height(root.as_ref().unwrap().borrow().left.clone());
        let right_height = get_height(root.as_ref().unwrap().borrow().right.clone());

        // return the height of the tree
        return 1 + left_height.max(right_height);
    }
    // get the height of the left and right subtree
    let left_height = get_height(root.as_ref().unwrap().borrow().left.clone());
    let right_height = get_height(root.as_ref().unwrap().borrow().right.clone());

    // if the height of the left and right subtree is not more than 1, then the tree is balanced
    if (left_height - right_height).abs() <= 1 {
        // check if the left and right subtree are balanced
        return is_balanced(root.as_ref().unwrap().borrow().left.clone())
            && is_balanced(root.as_ref().unwrap().borrow().right.clone());
    } else {
        // if the height of the left and right subtree is more than 1, then the tree is not balanced
        return false;
    }
}
