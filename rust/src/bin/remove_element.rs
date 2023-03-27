pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
            println!("i: {}, j: {}", i, j)
        }
    }
    i as i32
}

pub fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let len = remove_element(&mut nums, val);
    println!("len: {}, nums: {:?}", len, nums);
}
