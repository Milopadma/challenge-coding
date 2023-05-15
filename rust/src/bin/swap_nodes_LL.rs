#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// function that takes in a linked list head and swaps
// the kth node from the beginning with the kth node from the end
// and returns the new head
pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    // using stacks
    let mut stack = Vec::new();
    let mut node = &head;

    // push all nodes into a stack
    while let Some(n) = node {
        stack.push(n.val);
        node = &n.next;
    }

    // swap the kth node from the beginning with the kth node from the end
    let mut node = &mut head;
    let mut i = 1;
    while let Some(n) = node {
        if i == k {
            n.val = stack[stack.len() - k as usize];
        } else if i as usize == stack.len() - k as usize + 1 {
            println!("n.val: {}", n.val);
            n.val = stack[k as usize - 1];
            println!("stack.len: {}", stack.len() - k as usize);
        }
        i += 1;
        node = &mut n.next;
    }

    head
}

pub fn main() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node = &mut head;
    for i in 2..=5 {
        node.as_mut().unwrap().next = Some(Box::new(ListNode::new(i)));
        node = &mut node.as_mut().unwrap().next;
    }
    let head = swap_nodes(head, 2);
    let mut node = &head;
    while let Some(n) = node {
        println!("{}", n.val);
        node = &n.next;
    }
}
