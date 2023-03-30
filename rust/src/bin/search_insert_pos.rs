pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;

    while left <= right {
        let mid: usize = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    // At this point, the target is not in the array, so we return the
    // index where it should be inserted.
    left as i32
}
pub fn main() {
    // generate test cases for search_insert
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let result = search_insert(nums, target);
    println!("result: {}", result);
}
