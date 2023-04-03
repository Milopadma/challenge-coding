pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

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
    -1
}
pub fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let result = search(nums, target);
    println!("result: {}", result);

    let nums = vec![5];
    let target = -5;
    let result = search(nums, target);
    println!("result: {}", result);
}
