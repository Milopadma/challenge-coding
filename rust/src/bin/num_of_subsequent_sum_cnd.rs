pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums: Vec<i32> = nums;
    let mut count = 0;
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mod_num = 10_i32.pow(9) + 7;
    let mut pow = vec![1; nums.len()];
    // calculate 2^i   
    for i in 1..nums.len() {
        pow[i] = (pow[i - 1] * 2) % mod_num;
    }
    nums.sort();
    // iterate from both ends
    while i <= j {
        // if the sum of the two numbers is greater than target, then we need to
        // decrease the sum by decreasing the larger number
        if nums[i] + nums[j] > target {
            j -= 1;
        } else {
            // if the sum of the two numbers is less than target, then we can
            // choose any number between i and j to pair with j, so we add 2^(j-i)
            count = (count + pow[j - i]) % mod_num;
            i += 1;
        }
    }
    count
}

pub fn main() {
    let nums = vec![3, 5, 6, 7];
    let target = 9;
    let result = num_subseq(nums, target);
    println!("result: {}", result);
}
