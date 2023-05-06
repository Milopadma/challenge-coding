// Initialize answer = 0 and the length of nums as n. Iterate over the left
// index left from 0 to n - 1, for each index left: Use binary search to locate
// the rightmost index right which nums[right] <= target - nums[left]. If left
// <= right, count the total number of valid subsequences as 2right - left2 ^
// {\text{right - left}}2 right - left Increment answer by the number of valid
// subsequences. Return answer once the iteration ends.
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
