use std::{rc::Rc, cell::RefCell};

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>> q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = Vec::new();
    stack.push((p, q));
    while !stack.is_empty() {
        // first pop from stack and unwrap the tuple
        let (p, q) = stack.pop().unwrap();
        if p.is_none() && q.is_none() {
            continue;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        // then finally unwrap it to get the values
        let (p, q) = (p.unwrap(), q.unwrap());
        if p.borrow().val != q.borrow().val {
            return false;
        }
        stack.push((p.borrow().left.clone(), q.borrow().left.clone()));
        stack.push((p.borrow().right.clone(), q.borrow().right.clone()));
    }
    true
}
pub fn main(){ 

}