use std::{cell::RefCell, rc::Rc};

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // check if the root is empty
    if root.is_none() {
        return 0;
    }
    // then traverse tree until reach end, and add to max var
    let mut max = 0;
    traverse_tree(root, &mut max)
}

fn traverse_tree(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    while let Some(node) = root {
        let node = node.borrow();
        let left = node.left.clone();
        let right = node.right.clone();
        max += 1;
        traverse_tree(left, max);
        traverse_tree(right, max);
    }
    max
}
pub fn main() {}
