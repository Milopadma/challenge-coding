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

mod merge_sorted_array_lc {
    use core::num;

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // trim out the 0s
        let mut trimmed_nums1: Vec<i32> = nums1
            .iter()
            .filter(|&x| *x != 0)
            .map(|&x| x)
            .collect::<Vec<i32>>();
        // iterate and evaluate, O of n^2
        for (i, num) in trimmed_nums1.clone().iter().enumerate() {
            for (i2, num2) in nums2.iter().enumerate() {
                if *num2 as i32 >= *num as i32 {
                    trimmed_nums1.insert(i, *num2);
                    nums2.remove(i2);
                    break;
                } else {
                    continue;
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
