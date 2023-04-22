use std::{cell::RefCell, rc::Rc};

// similar to implementatino of converting array to bst
// but this one is differenti since it takes
// the linked list to an array first
pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nums = Vec::new();
    let mut head = head;
    // just a simple linked list to array conversion loop
    while let Some(node) = head {
        nums.push(node.val);
        head = node.next;
    }
    build_tree(&nums)
}

fn build_tree(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    // take the middle
    let mid = nums.len() / 2;
    // set the root
    let mut root = TreeNode::new(nums[mid]);
    // recursively build the left and right subtrees
    root.left = build_tree(&nums[..mid].to_vec());
    root.right = build_tree(&nums[mid + 1..].to_vec());
    // return the root
    Some(Rc::new(RefCell::new(root)))
}
