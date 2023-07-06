impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min = std::usize::MAX;
        let mut sum = 0;
        let mut left = 0;
        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            while sum >= target {
                min = min.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if min == std::usize::MAX {
            0
        } else {
            min as i32
        }
    }
}
