pub fn is_palindrome(s: String) -> bool {
    // a phrase is a palindrome if it is the same forwards and backwards
    // normalize input first and remove any non-alphanumeric characters
    let mut normalized = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    // reverse the normalized string
    let mut reversed = normalized.clone();

    // compare the normalized and reversed strings
    match reversed.chars().rev().collect::<String>().as_str() {
        s if s == normalized.as_str() => true,
        _ => false,
    }
}
pub fn main() {
    // test
    println!("{:?}", is_palindrome("race a car".to_string()));
    println!("{:?}", is_palindrome("racecar".to_string()));
}
