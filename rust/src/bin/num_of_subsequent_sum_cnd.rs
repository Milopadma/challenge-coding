// Two pointers method
// Initialize answer = 0 and the length of nums as n. Set two pointers left = 0
// and right = n - 1. Iterate over left while left <= right, for each index
// left:
// 1. If nums[left] + nums[right] > target, it means nums[right] is too large
//    for the right boundary, we shall move it to the left by setting right =
//    right - 1.
// 2. Otherwise, right is the rightmost index which nums[right] <= target -
// nums[left], we can count the total number of valid subsequences as 2right -
// left2 ^ {\text{right - left}}2 right - left
// and increment answer by this number.
// 3. Repeat step 2. Return answer once the iteration ends.
pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut ans = 0;
    let n = nums.len();
    let mut nums = nums;
    nums.sort();
    let mut left = 0;
    let mut right = n - 1;
    if right == 0 {
        return 0;
    }
    let modulo = 1_000_000_007;
    let mut pow = vec![1; n];
    for i in 1..n {
        pow[i] = pow[i - 1] * 2 % modulo;
    }
    while left <= right {
        if nums[left] + nums[right] > target {
            right -= 1;
        } else {
            ans = (ans + pow[right - left]) % modulo;
            left += 1;
        }
    }
    ans as i32
}
pub fn main() {
    let nums = vec![3, 5, 6, 7];
    let target = 9;
    let result = num_subseq(nums, target);
    println!("result: {}", result);
}
