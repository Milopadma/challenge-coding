struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut s3 = std::i32::MIN;
        for i in (0..nums.len()).rev() {
            if nums[i] < s3 {
                return true;
            }
            while !stack.is_empty() && nums[i] > stack[stack.len() - 1] {
                s3 = stack.pop().unwrap();
            }
            stack.push(nums[i]);
        }
        false
    }
}

fn main() {
    assert_eq!(true, Solution::find132pattern(vec![3, 1, 4, 2]));
    assert_eq!(false, Solution::find132pattern(vec![1, 2, 3, 4]));
    println!("Pass test cases!");
}
