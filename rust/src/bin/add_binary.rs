pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a = a.chars().rev();
    let mut b = b.chars().rev();
    loop {
        let a = a.next().unwrap_or('0');
        let b = b.next().unwrap_or('0');
        if a == '0' && b == '0' {
            if carry == 0 {
                break;
            } else {
                result.push('1');
                carry = 0;
            }
        } else if a == '1' && b == '1' {
            if carry == 0 {
                result.push('0');
                carry = 1;
            } else {
                result.push('1');
            }
        } else {
            if carry == 0 {
                result.push('1');
            } else {
                result.push('0');
            }
        }
    }
    result.chars().rev().collect()
}

pub fn main() {
    let a = String::from("11");
    let b = String::from("1");
    let result = add_binary(a, b);
    println!("result: {}", result);
}
