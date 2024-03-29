pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // use a vecdeque to store the nodes
    use std::collections::VecDeque;

    // loop through the tree and find the min depth
    // if the node has no children, return 1
    // if the node has one child, return 1 + min_depth(child)

    if root.is_none() {
        return 0;
    }

    let mut min_depth = 0; // keep track of the min depth
    let mut queue = VecDeque::new(); // use a queue to store the nodes
    queue.push_back(root); // push the root node into the queue

    // loop through the queue untill it is empty
    while !queue.is_empty() {
        min_depth += 1;
        let mut size = queue.len();

        // while the queue is not empty, pop the nodes and check if they have children
        while size > 0 {
            let node = queue.pop_front().unwrap();
            let node = node.unwrap();
            let node = node.borrow();

            // if the node has no children, return the min depth
            if node.left.is_none() && node.right.is_none() {
                return min_depth;
            }

            // if the node has children, push them into the queue
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
