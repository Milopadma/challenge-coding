pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    for (i, num) in nums.clone().iter().enumerate() {
        if *num == val {
            nums.swap(i, nums.len() - 1);
        }
    }
    nums.len() as i32
}

pub fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let len = remove_element(&mut nums, val);
    println!("len: {}, nums: {:?}", len, nums);
}
