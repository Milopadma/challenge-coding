pub fn add_binary(a: String, b: String) -> String {
    // using match
    let mut result = String::new();
    let mut carry = 0;
    let mut a = a.chars().rev();
    let mut b = b.chars().rev();
    loop {
        match (a.next(), b.next()) {
            (Some('1'), Some('1')) => {
                result.push(if carry == 1 { '1' } else { '0' });
                carry = 1;
            }
            (Some('1'), Some('0')) | (Some('0'), Some('1')) => {
                result.push(if carry == 1 { '0' } else { '1' });
            }
            (Some('0'), Some('0')) => {
                result.push(if carry == 1 { '1' } else { '0' });
                carry = 0;
            }
            (Some('1'), None) | (None, Some('1')) => {
                result.push(if carry == 1 { '0' } else { '1' });
            }
            (Some('0'), None) | (None, Some('0')) => {
                result.push(if carry == 1 { '1' } else { '0' });
                carry = 0;
            }
            (None, None) => {
                if carry == 1 {
                    result.push('1');
                }
                break;
            }
            _ => unreachable!(),
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
