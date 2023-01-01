impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[2] = 2;
        dp[3] = 3;
        for i in 4..=n {
            dp[i as usize] = std::cmp::max(dp[i as usize], dp[i as usize - 2] * 2);
            dp[i as usize] = std::cmp::max(dp[i as usize], dp[i as usize - 3] * 3);
        }
        dp[n as usize]
    }
}