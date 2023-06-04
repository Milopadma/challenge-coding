struct Solution;
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];
        let mut count = 0;
        for i in 0..is_connected.len() {
            if !visited[i] {
                Self::dfs(&is_connected, &mut visited, i);
                count += 1;
            }
        }
        count
    }

    fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
        visited[i] = true;
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && !visited[j] {
                Self::dfs(is_connected, visited, j);
            }
        }
    }
}

pub fn main() {
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    println!("{}", Solution::find_circle_num(is_connected));
}
