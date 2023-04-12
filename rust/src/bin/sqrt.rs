pub fn my_sqrt(x: i32) -> i32 {
    let mut i: usize = 0;
    while i * i <= x as usize {
        i += 1;
    }
    (i - 1) as i32
}
pub fn main() {
    println!("{}", my_sqrt(2));
}
