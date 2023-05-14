// function that takes in an array of numbers of size 2 * n,
// performs n operations on the array and
// the operation is
// - choose two elements x and y,
// - receive a score of i * gcd(x, y)
// - remove x and y from the array
// returns the maximum score that can be received after n operations
// phind iml
use std::collections::HashMap;
pub fn max_score(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();

    fn helper(
        op: usize,
        visited: usize,
        nums: &Vec<i32>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let n = nums.len();
        if op == n / 2 + 1 {
            return 0;
        }
        if let Some(&score) = memo.get(&(op, visited)) {
            return score;
        }

        let mut max_score = 0;
        for i in 0..n {
            if visited & (1 << i) == 0 {
                for j in (i + 1)..n {
                    if visited & (1 << j) == 0 {
                        let new_visited = visited | (1 << i) | (1 << j);
                        let score = op as i32 * gcd(nums[i], nums[j])
                            + helper(op + 1, new_visited, nums, memo);
                        max_score = max_score.max(score);
                    }
                }
            }
        }

        memo.insert((op, visited), max_score);
        max_score
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    helper(1, 0, &nums, &mut memo)
}
pub fn main() {
    let nums = vec![1, 2];
    println!("{}", max_score(nums));
    let nums = vec![3, 4, 6, 8];
    println!("{}", max_score(nums));
    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("{}", max_score(nums));
}

// Original
// pub fn max_score(nums: Vec<i32>) -> i32 {
//     let mut nums = nums;
//     let mut score = 0;
//     let mut n = nums.len() / 2;
//     while n > 0 {
//         let mut max = 0;
//         let mut max_i = 0;
//         let mut max_j = 0;
//         for i in 0..nums.len() {
//             for j in i + 1..nums.len() {
//                 let gcd = gcd(nums[i], nums[j]);
//                 if gcd > max {
//                     max = gcd;
//                     max_i = i;
//                     max_j = j;
//                 }
//             }
//         }
//         score += max * n as i32;
//         nums.remove(max_i);
//         nums.remove(max_j - 1);
//         n -= 1;
//     }
//     score
// }

// // gcd is greatest common divisor
// pub fn gcd(a: i32, b: i32) -> i32 {
//     if a == 0 {
//         return b;
//     }
//     gcd(b % a, a)
// }

// GPT-4
// use std::cmp::{max, min};
// pub fn max_score(nums: Vec<i32>) -> i32 {
//     let n = nums.len();
//     let mut dp = vec![vec![0; n + 1]; n + 1];
//     let mut nums = nums.clone();
//     nums.sort();

//     for i in 0..n {
//         for j in (0..i).rev() {
//             let x = nums[i];
//             let y = nums[j];
//             let gcd_value = gcd(x, y);
//             let new_i = i + 1;
//             let cur = dp[new_i][min(new_i, j + 1)];

//             dp[new_i][j] = max(dp[new_i][j], dp[i][j] + gcd_value);
//             dp[new_i][j + 1] = max(dp[new_i][j + 1], cur + gcd_value * ((n - j) as i32 / 2));
//         }
//     }

//     dp[n][0]
// }

// // gcd is greatest common divisor
// pub fn gcd(a: i32, b: i32) -> i32 {
//     if a == 0 {
//         return b;
//     }
//     gcd(b % a, a)
// }
