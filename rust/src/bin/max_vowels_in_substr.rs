// function that takes in string and a substr length of k
// and return the longest number of vowels in the substr of length k
// high efficiency implementation
pub fn max_vowels(s: String, k: i32) -> i32 {
    // the function to check if a char is a vowel
    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
    let mut max_vowels = 0;
    let mut vowels = 0;
    // we have two indexes but in one loop
    let mut i = 0;
    let mut j = 0;
    let chars: Vec<char> = s.chars().collect();
    while j < chars.len() {
        if is_vowel(chars[j]) {
            vowels += 1;
        }
        // if the substr length is k, we need to move the window
        if j - i + 1 == k as usize {
            // update the max_vowels
            max_vowels = std::cmp::max(max_vowels, vowels);
            // if the char at i is a vowel, we need to decrease the vowels
            if is_vowel(chars[i]) {
                vowels -= 1;
            }
            // move the window
            i += 1;
        }
        j += 1;
    }
    max_vowels
}

pub fn main() {
    let s = String::from("abciiidef");
    let k = 3;
    let max_vowels = max_vowels(s, k);
    println!("max_vowels: {}", max_vowels);
}
