struct Solution;
impl Solution {
    // since k is radius, the first k-number of elements would be -1 since there wouldnt be enough numbers to average
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut avgs = vec![-1; n];
        for i in 0..n {
            if i >= k as usize && i < n - k as usize {
                let start = i - k as usize;
                let end = i + k as usize + 1; // plus 1 because the range end is exclusive
                let sum: i32 = nums[start..end].iter().sum();
                avgs[i] = sum / (2 * k + 1);
            }
        }
        avgs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::get_averages(vec![1, 3, 2, 6, -1, 4, 1, 8, 2], 5),
            vec![2, 2, 2, 3, 2]
        );
    }
}
