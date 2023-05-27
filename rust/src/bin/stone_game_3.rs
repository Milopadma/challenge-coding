pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    // dynamic programming
    let mut dp = vec![0; stone_value.len() + 1];
    // reverse the order of the array
    for i in (0..stone_value.len()).rev() {
        dp[i] = std::i32::MIN; // set to the smallest value
        let mut sum = 0;
        // iterate to 3 steps to find the max value
        for j in 0..3 {
            if i + j < stone_value.len() {
                sum += stone_value[i + j];
                dp[i] = dp[i].max(sum - dp[i + j + 1]); // compare the current value with the next value and get the max value
            }
        }
    }
    // then just compare the first value
    match dp[0].cmp(&0) {
        std::cmp::Ordering::Greater => "Alice".to_string(),
        std::cmp::Ordering::Less => "Bob".to_string(),
        std::cmp::Ordering::Equal => "Tie".to_string(),
    }
}

pub fn main() {
    let stone_value = vec![1, 2, 3, 7];
    println!("{}", stone_game_iii(stone_value));
}
