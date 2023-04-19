use std::{cell::RefCell, rc::Rc};

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // a recursive function
    fn is_mirror(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // match the left and right nodes
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                // if the values of left and right are equal
                left.borrow().val == right.borrow().val
                // and the left.left and right.right are mirror
                    && is_mirror(left.borrow().left.clone(), right.borrow().right.clone())
                    && is_mirror(left.borrow().right.clone(), right.borrow().left.clone())
            }
            _ => false,
        }
    }
    // if root is None, return true
    // else, return the result of is_mirror
    root.is_none()
        || is_mirror(
            root.as_ref().unwrap().borrow().left.clone(),
            root.as_ref().unwrap().borrow().right.clone(),
        )
}
pub fn main() {}
