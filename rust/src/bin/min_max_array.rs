pub fn minimize_array_value(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return nums[0];
    }
    // Initialize the minimum possible value of the maximum integer
    let mut min_max_val = nums[0];
    // Iterate through the array
    for i in 1..n {
        // If nums[i] > 0 and nums[i - 1] < nums[i], perform the operation
        if nums[i] > 0 && nums[i - 1] < nums[i] {
            nums[i - 1] += 1;
            nums[i] -= 1;
        }
        // Update the minimum possible value of the maximum integer
        min_max_val = min_max_val.max(nums[i]);
    }
    min_max_val
}

pub fn main() {
    let mut nums = vec![3, 7, 1, 6];
    let min = minimize_array_value(&mut nums);
    println!("Min value is {}", min);
}
