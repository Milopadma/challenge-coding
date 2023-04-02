pub fn multiply(num1: String, num2: String) -> String {
    let mut result = vec![0; num1.len() + num2.len()];
    let mut carry = 0;
    for i in (0..num1.len()).rev() {
        for j in (0..num2.len()).rev() {
            let sum = (num1.as_bytes()[i] - b'0') * (num2.as_bytes()[j] - b'0')
                + carry
                + result[i + j + 1];
            carry = sum / 10;
            result[i + j + 1] = sum % 10;
        }
        result[i] += carry;
        carry = 0;
    }
    let mut res = String::new();
    let mut i = 0;
    while i < result.len() && result[i] == 0 {
        i += 1;
    }
    while i < result.len() {
        res.push((result[i] + b'0') as char);
        i += 1;
    }
    if res.is_empty() {
        res.push('0');
    }
    res
}

pub fn main() {
    let num1 = String::from("123");
    let num2 = String::from("456");
    println!("{}", multiply(num1, num2));
}
