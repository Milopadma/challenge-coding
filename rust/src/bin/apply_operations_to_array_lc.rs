pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        } else {
            continue;
        }
    }
    // to shift all 0s to the end of the array
    nums.iter()
        .filter(|&x| *x != 0) // takes the 0s out
        .chain(nums.iter().filter(|&x| *x == 0)) // puts them at the back
        .cloned()
        .collect()
}
pub fn main() {
    let nums = vec![1, 0, 0, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", apply_operations(nums));
}
