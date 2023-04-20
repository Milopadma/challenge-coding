pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    // check if the array is empty
    if nums.is_empty() {
        return None;
    }
    // get the middle element
    let mid = nums.len() / 2;
    // create a new node with the middle element
    let mut root = TreeNode::new(nums[mid]);
    // this will eventually have the left and right subtrees be height balanced
    root.left = sorted_array_to_bst(nums[..mid].to_vec());
    root.right = sorted_array_to_bst(nums[mid + 1..].to_vec());
    // return the root
    Some(Rc::new(RefCell::new(root)))
}
pub fn main() {}
