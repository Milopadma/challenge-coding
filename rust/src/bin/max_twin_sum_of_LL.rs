pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
// function that takes in a head of an always even linked
// list, goes through it and finds the maximum
// sum between two twin nodes
// twin nodes in this case refers to nodes
// where ith node is a twin of (n-1-i)th node
pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut head = head;
    let mut max_sum = 0;
    let mut current = &mut head;
    while let Some(node) = current {
        let mut current2 = node.next.as_mut();
        while let Some(node2) = current2 {
            let sum = node.val + node2.val;
            if sum > max_sum {
                max_sum = sum;
            }
            current2 = node2.next.as_mut();
        }
        current = node.next.as_mut();
    }
    max_sum
}

pub fn main() {}
