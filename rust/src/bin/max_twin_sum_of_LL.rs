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
    // using stack
    let mut stack = Vec::new();
    let mut head = head;
    let mut max_sum = 0;
    while let Some(node) = head {
        stack.push(node.val);
        head = node.next;
    }
    // iterate through the stack and find the max sum
    // between the twin nodes
    let mut i = 0;
    let mut j = stack.len() - 1;
    while i < j {
        let sum = stack[i] + stack[j];
        if sum > max_sum {
            max_sum = sum;
        }
        i += 1;
        j -= 1;
    }
    max_sum
}

pub fn main() {}
