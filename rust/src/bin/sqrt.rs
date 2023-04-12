pub fn my_sqrt(x: i32) -> i32 {
    let mut i = 0;
    while i * i <= x {
        i += 1;
    }
    i - 1
}
pub fn main() {
    println!("{}", my_sqrt(2));
}
