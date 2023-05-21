// fn that takes matrix grid and finds the smallest number of 0's to change to
// 1's to connect the two islands together
pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    // Helper function to perform DFS and mark the first island with -1
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i < grid.len() && j < grid[0].len() && grid[i][j] == 1 {
            grid[i][j] = -1;
            dfs(grid, i + 1, j);
            dfs(grid, i.saturating_sub(1), j);
            dfs(grid, i, j + 1);
            dfs(grid, i, j.saturating_sub(1));
        }
    }

    // Find and mark the first island
    let mut found_island = false;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                dfs(&mut grid, i, j);
                found_island = true;
                break;
            }
        }
        if found_island {
            break;
        }
    }

    // Helper function to perform BFS and find the shortest bridge
    fn bfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> Option<usize> {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((i, j, 0));

        while let Some((i, j, steps)) = queue.pop_front() {
            if i < grid.len() && j < grid[0].len() {
                match grid[i][j] {
                    0 => {
                        grid[i][j] = -2; // Mark as visited
                        queue.push_back((i + 1, j, steps + 1));
                        queue.push_back((i.saturating_sub(1), j, steps + 1));
                        queue.push_back((i, j + 1, steps + 1));
                        queue.push_back((i, j.saturating_sub(1), steps + 1));
                    }
                    1 => return Some(steps),
                    -1 => (),
                    _ => (),
                }
            }
        }

        None
    }

    // Start BFS from each cell of the first island and find the shortest bridge
    let mut min_bridge = std::i32::MAX;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == -1 {
                if let Some(bridge) = bfs(&mut grid, i, j) {
                    min_bridge = min_bridge.min(bridge as i32);
                }
            }
        }
    }

    min_bridge
}

pub fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    println!("{}", shortest_bridge(grid));
}
