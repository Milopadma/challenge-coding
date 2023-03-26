pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    for (i, nums) in nums.iter().enumerate() {
        if *nums == val {
            nums.remove(i);
        }
    }
}

pub fn main() {}
