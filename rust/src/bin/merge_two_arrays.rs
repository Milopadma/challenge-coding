mod merge_sorted_array_lc {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // trim out the 0s
        nums1.truncate(m as usize); // uses m (which is the length of actual numbers before 0) to remove the 0s
                                    // iterate and evaluate, O of n^2
        for (i, num) in nums1.clone().iter().enumerate() {
            for (i2, num2) in nums2.iter().enumerate() {
                if *num2 as i32 >= *num as i32 {
                    nums1.insert(i, *num2);
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
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge_sorted_array_lc::merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:#?}", nums1);
}
