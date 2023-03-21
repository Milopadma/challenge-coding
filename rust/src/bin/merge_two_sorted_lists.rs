use crate::merge_two_sorted_lists_lc::ListNode;

mod merge_two_sorted_lists_lc {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        {
            match (list1, list2) {
                (None, None) => None,
                (None, Some(node)) | (Some(node), None) => Some(node),
                (Some(node1), Some(node2)) => {
                    if node1.val < node2.val {
                        Some(Box::new(ListNode {
                            val: node1.val,
                            next: merge_two_lists(node1.next, Some(node2)),
                        }))
                    } else {
                        Some(Box::new(ListNode {
                            val: node2.val,
                            next: merge_two_lists(Some(node1), node2.next),
                        }))
                    }
                }
            }
        }
    }
}

pub fn main() {
    // mod 16 test case
    // first linked list
    let mut list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    // second linked list
    let mut list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    println!(
        "{:#?}",
        merge_two_sorted_lists_lc::merge_two_lists(list1, list2)
    );
}
