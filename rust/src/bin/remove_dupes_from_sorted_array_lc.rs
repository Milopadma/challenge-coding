// Title: Remove Duplicates from Sorted Array

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    for (i, num) in nums.clone().iter().enumerate() {
        if i > 0 && num == &nums[i - 1] {
            nums.remove(i);
        }
    }
    nums.len() as i32
}
// turn the vec into enumeratable iterator and then
// remove the duplicate numbers from the vec, in place

pub fn main() {
    // generate test cases for pub fn remove duplicates
    let mut nums = vec![1, 1, 2];
    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let mut nums3 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    println!("{:?}", remove_duplicates(&mut nums));
    println!("{:?}", remove_duplicates(&mut nums2));
    println!("{:?}", remove_duplicates(&mut nums3));
}
