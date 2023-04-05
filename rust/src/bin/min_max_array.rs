pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return nums[0];
    }
    let mut min_max_val = nums[0];
    let mut sum = nums[0] as usize;

    for i in 1..n {
        // Update the sum of elements in the array
        sum += nums[i] as usize;
        // Calculate the minimum possible value for the current maximum element
        let min_possible_val = (sum + i) / (i + 1);
        min_max_val = min_max_val.max(min_possible_val as i32);
    }

    min_max_val
}

pub fn main() {
    let mut nums = vec![3, 7, 1, 6];
    let min = minimize_array_value(nums);
    println!("Min value is {}", min);

    let mut nums_2 = vec![10, 1];
    let min_2 = minimize_array_value(nums_2);
    println!("Min value is {}", min_2);
}
