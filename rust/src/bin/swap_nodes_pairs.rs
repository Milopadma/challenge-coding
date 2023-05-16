#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

// function that takes a head of a linked list
// and swaps every two adjacent nodes
// and returns the head
// ex. 1 -> 2 -> 3 -> 4
// returns 2 -> 1 -> 4 -> 3
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // make it mut
    let mut head = head;
    // early catch and end case
    // if the head is None or the next node is None
    // return the head
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    // get the next node
    let mut next = head.as_mut().unwrap().next.take();

    // swap the nodes
    head.as_mut().unwrap().next = swap_pairs(next.as_mut().unwrap().next.take());

    // set the next node to the head
    next.as_mut().unwrap().next = head;

    // return the next node
    next
}

pub fn main() {
    let head = Some(Box::new(ListNode::new(
        1,
        Some(Box::new(ListNode::new(
            2,
            Some(Box::new(ListNode::new(
                3,
                Some(Box::new(ListNode::new(4, None))),
            ))),
        ))),
    )));

    let mut swapped = swap_pairs(head);

    while swapped.is_some() {
        println!("{}", swapped.as_ref().unwrap().val);
        swapped = swapped.unwrap().next;
    }
}
