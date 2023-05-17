// function that given a list of questions and brainpower, returns the
// maximum number of points you can score in the exam
// - the 'brainpower' represents the amount of questions that needs to be
// skipped
// - the questions must be checked in order
pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    // using recursive dfs to find the maximum number of points
    fn dfs(questions: &Vec<Vec<i32>>, index: usize, memo: &mut Vec<i64>) -> i64 {
        if index >= questions.len() {
            return 0;
        }
        if memo[index] != -1 {
            return memo[index];
        }
        let skip = dfs(questions, index + 1, memo);
        let solve = questions[index][0] as i64
            + dfs(questions, index + (questions[index][1] as usize) + 1, memo);
        memo[index] = skip.max(solve);
        memo[index]
    }
    let n = questions.len();
    let mut memo = vec![-1; n];
    dfs(&questions, 0, &mut memo) as i64
}

pub fn main() {
    let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
    // this needs to return 5
    println!("{}", most_points(questions));
}
// let mut brainpower = 0;
// let mut points = 0;
// for question in questions {
//     if brainpower >= question[1] {
//         brainpower -= question[1];
//     } else {
//         points += question[0] - brainpower;
//         brainpower = 0;
//     }
// }
// points as i64

// 27ms runtime solution
use std::convert::TryInto;
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        // dp[i] is number of points we can get from suffix with len = i.
        let mut dp = vec![0i64; questions.len() + 1];
        for (q, suff_len) in questions.iter().rev().zip(1..) {
            assert!(q.len() >= 2);
            let points = q[0];
            let brain_power: usize = q[1].try_into().unwrap();
            let suff_len_if_use = questions
                .get(questions.len() - suff_len..)
                .and_then(|s| s.get(1..))
                .and_then(|s| s.get(brain_power..))
                .map(|x| x.len())
                .unwrap_or(0);
            dp[suff_len] = std::cmp::max(dp[suff_len - 1], dp[suff_len_if_use] + i64::from(points));
        }
        dp[questions.len()]
    }
}
