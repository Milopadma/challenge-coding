pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut current = &mut head;
    // if current node exists
    while let Some(node) = current {
        // if next node exists
        while let Some(next) = &node.next {
            // if next node value is equal to current node value
            if next.val == node.val {
                // set current node next to next node next
                node.next = next.next.clone();
            } else {
                break;
            }
        }
        // set current to current node next
        current = &mut node.next;
    }
    // return head
    head
}
pub fn main() {}
