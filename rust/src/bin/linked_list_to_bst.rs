use std::{cell::RefCell, rc::Rc};

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nums = Vec::new();
    let mut head = head;
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
    let mid = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid]);
    root.left = build_tree(&nums[..mid].to_vec());
    root.right = build_tree(&nums[mid + 1..].to_vec());
    Some(Rc::new(RefCell::new(root)))
}
