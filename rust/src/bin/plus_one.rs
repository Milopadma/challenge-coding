pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    // get the last element in array
    let last = digits.len() - 1;
    match digits[last] {
        9 => {
            digits[last] = 0;
            let mut i = last;
            while i > 0 {
                if digits[i - 1] == 9 {
                    digits[i - 1] = 0;
                    i -= 1;
                } else {
                    digits[i - 1] += 1;
                    break;
                }
            }
            if digits[0] == 0 {
                digits.insert(0, 1);
            }
        }
        _ => {
            digits[last] += 1;
        }
    }
    digits
}
pub fn main() {
    let digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let result = plus_one(digits);
    println!("{:?}", result);

    let digits2 = vec![
        7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4, 7,
        0, 1, 1, 1, 7, 4, 0, 0, 6,
    ];
    let result2 = plus_one(digits2);
    println!("{:?}", result2);
}
