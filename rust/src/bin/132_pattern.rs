struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![]; // keep track of possible 2nd numbers in a stack
        let mut s3 = std::i32::MIN; // keep track of the 3rd number
        for i in (0..nums.len()).rev() {
            if nums[i] < s3 {
                return true; // if we find a 1st number that is smaller than the 3rd number, we found a 132 pattern
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
