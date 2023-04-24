pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // use a vecdeque to store the nodes
    use std::collections::VecDeque;

    // loop through the tree and find the min depth
    // if the node has no children, return 1
    // if the node has one child, return 1 + min_depth(child)

    if root.is_none() {
        return 0;
    }

    let mut min_depth = 0;
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        min_depth += 1;
        let mut size = queue.len();

        while size > 0 {
            let node = queue.pop_front().unwrap();
            let node = node.unwrap();
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() {
                return min_depth;
            }

            if node.left.is_some() {
                queue.push_back(node.left.clone());
            }

            if node.right.is_some() {
                queue.push_back(node.right.clone());
            }

            size -= 1;
        }
    }

    min_depth
}
