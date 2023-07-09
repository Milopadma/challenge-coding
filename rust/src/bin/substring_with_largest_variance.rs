impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut max_variance = 0.0;
        let mut max_variance_start = 0;
        let mut max_variance_end = 0;
        let mut max_variance_len = 0;
        let mut max_variance_sum = 0.0;
        let mut max_variance_sum_of_squares = 0.0;
        let mut sum = 0.0;
        let mut sum_of_squares = 0.0;
        let mut len = 0;
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.chars().enumerate() {
            if c == ' ' {
                if len > 0 {
                    let mean = sum / len as f64;
                    let variance = sum_of_squares / len as f64 - mean * mean;
                    if variance > max_variance {
                        max_variance = variance;
                        max_variance_start = start;
                        max_variance_end = end;
                        max_variance_len = len;
                        max_variance_sum = sum;
                        max_variance_sum_of_squares = sum_of_squares;
                    }
                    sum = 0.0;
                    sum_of_squares = 0.0;
                    len = 0;
                }
                start = i + 1;
            } else {
                end = i + 1;
                let x = c as i32 - '0' as i32;
                sum += x as f64;
                sum_of_squares += x as f64 * x as f64;
                len += 1;
            }
        }
        if len > 0 {
            let mean = sum / len as f64;
            let variance = sum_of_squares / len as f64 - mean * mean;
            if variance > max_variance {
                max_variance = variance;
                max_variance_start = start;
                max_variance_end = end;
                max_variance_len = len;
                max_variance_sum = sum;
                max_variance_sum_of_squares = sum_of_squares;
            }
        }
        if max_variance_len == 0 {
            return 0;
        }
        let mean = max_variance_sum / max_variance_len as f64;
        let variance = max_variance_sum_of_squares / max_variance_len as f64 - mean * mean;
        let mut result = 0;
        for i in max_variance_start..max_variance_end {
            let c = s.chars().
        
    }
}