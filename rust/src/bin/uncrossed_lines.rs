// function thta takes in two arrays
// and figures out the max number of imaginary lines
// that can be made between two equal numbers in both
// arrays, the lines cannot cross each other.
pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    // dp[i][j] = max number of lines that can be drawn
    let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];

    // iterate through both arrays
    for i in 0..nums1.len() {
        for j in 0..nums2.len() {
            if nums1[i] == nums2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    println!("State of dp : {:?}", dp);
    dp[nums1.len()][nums2.len()]
}

// tests in main
pub fn main() {
    let nums1 = vec![1, 4, 2];
    let nums2 = vec![1, 2, 4];
    let res = max_uncrossed_lines(nums1, nums2);
    println!("res: {}", res);
}
