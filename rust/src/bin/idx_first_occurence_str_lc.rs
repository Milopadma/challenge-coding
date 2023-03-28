pub fn str_str(haystack: String, needle: String) -> i32 {
    for (i, c) in haystack.char_indices() {
        if Some((0, c)) == needle.char_indices().next() {
            if haystack[i..].starts_with(&needle) {
                // only then check if the char continues with the proper needle
                return i as i32;
            }
        }
    }
    -1
}
pub fn main() {
    println!("{}", str_str("hello".to_string(), "ll".to_string()));
}
