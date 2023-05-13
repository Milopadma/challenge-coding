// function that given a list of questions and brainpower, returns the
// maximum number of points you can score in the exam
// - the 'brainpower' represents the amount of questions that needs to be
// skipped
// - the questions must be checked in order
pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
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
