// given the integers zero, one, low, and high,
// return the number of different good strings
// that can be constructed in the range [low, high] using
// only the characters '0' 'zero' param times and '1' 'one' param times.

//
// 1. Create an array dp of size 1 + high. Initialize dp[0] = 1 and the value of all the rest cells as -1.
// 2. Define a recursive function dfs(end), if dp[end] != -1, return dp[end], otherwise:
// - Set answer = 0.
// - If end >= zero, increment answer by dfs(end - zero).
// - If end >= one, increment answer by dfs(end - one).
// - Update dp[end] as answer.
// 3. Once the iteration ends, add up the numbers in dp[low ~ high].

const MOD: i64 = 1_000_000_007;
pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    // dynamic programming; reduce the subproblem recursively until it reaches the base case
    fn dfs(end: usize, zero: usize, one: usize, dp: &mut Vec<i64>) -> i64 {
        if dp[end] != -1 {
            return dp[end];
        }
        let mut ans = 0;
        if end >= zero {
            ans += dfs(end - zero, zero, one, dp);
            ans %= MOD;
        }
        if end >= one {
            ans += dfs(end - one, zero, one, dp);
            ans %= MOD;
        }
        dp[end] = ans;
        ansk
    }
    //
    let mut dp = vec![-1; 1 + high as usize];
    dp[0] = 1;
    let mut ans = 0;
    for i in low..=high {
        ans += dfs(i as usize, zero as usize, one as usize, &mut dp);
        ans %= MOD;
    }
    ans as i32
}

pub fn main() {
    let low = 3;
    let high = 3;
    let zero = 1;
    let one = 1;
    let ans = count_good_strings(low, high, zero, one);
    println!("{}", ans);
}
