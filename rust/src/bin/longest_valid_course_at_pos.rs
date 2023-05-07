pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; obstacles.len()];
    let mut stack = Vec::new(); // keep track of longest increasing subsequence
    for i in 0..obstacles.len() {
        let mut l = 0;
        let mut r = stack.len();
        // binary search for the first element in stack that is greater than
        // current obstacle
        while l < r {
            let mid = (l + r) / 2;
            if stack[mid] <= obstacles[i] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if l == stack.len() {
            // if we didn't find any element in stack that is greater than
            // current obstacle, we push current obstacle to stack
            stack.push(obstacles[i]);
        } else {
            // if we found an element in stack that is greater than current
            // obstacle, we replace it with current obstacle
            stack[l] = obstacles[i];
        }
        // the length of longest increasing subsequence is the length of stac
        ans[i] = (l + 1) as i32;
    }
    ans
}

pub fn main() {
    let obstacles = vec![1, 2, 3, 2];
    let result = longest_obstacle_course_at_each_position(obstacles);
    println!("{:?}", result);
}
