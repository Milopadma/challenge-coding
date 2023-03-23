mod merge_sorted_array_lc {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // trim out the 0s
        nums1.truncate(m as usize); // uses m (which is the length of actual numbers before 0) to remove the 0s

        for (i, num) in nums2.iter().enumerate() {
            nums1.push(*num); // push the nums2 into nums1
        }

        nums1.sort(); // sort the nums1
    }
}

pub fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge_sorted_array_lc::merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:#?}", nums1);
}
