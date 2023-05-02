// function that takes a vec of sorted nums and
// returns the sign of the product of all the nums
// where 'product' means to multiply all the nums together
// and 'sign' means if the product is positive or negative
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut sign = 1;
    for num in nums {
        if num == 0 {
            return 0;
        } else if num < 0 {
            sign *= -1;
        }
    }
    sign
}

pub fn main() {
    // tests
    let nums = vec![-1, -2, -3, -4, 3, 2, 1];
    let sign = array_sign(nums);
    println!("sign: {}", sign);
}
