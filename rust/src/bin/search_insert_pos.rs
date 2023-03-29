pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}
pub fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let result = search_insert(nums, target);
    println!("result: {}", result);
}
