pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().map_or(0, |s| s.len()) as i32
}
pub fn main() {
    let s = "Hello World".to_string();
    println!("{}", length_of_last_word(s));
}
