// fn that takes matrix grid and finds the smallest number of 0's to change to
// 1's to connect the two islands together
pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut queue = Vec::new();
    let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Find the first island and mark it with 2s
    'outer: for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                dfs(&mut grid, i, j, &mut queue);
                break 'outer;
            }
        }
    }

    // BFS to expand the island until it reaches the second island
    let mut distance = 0;
    while !queue.is_empty() {
        distance += 1;
        let len = queue.len();
        for _ in 0..len {
            let (x, y) = queue.remove(0);
            for (dx, dy) in &offsets {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                    if grid[nx as usize][ny as usize] == 1 {
                        return distance - 1;
                    } else if grid[nx as usize][ny as usize] == 0 {
                        grid[nx as usize][ny as usize] = 2;
                        queue.push((nx as usize, ny as usize));
                    }
                }
            }
        }
    }

    -1
}

fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, queue: &mut Vec<(usize, usize)>) {
    if x >= grid.len() || y >= grid.len() || grid[x][y] != 1 {
        return;
    }
    grid[x][y] = 2;
    queue.push((x, y));
    dfs(grid, x + 1, y, queue);
    dfs(grid, x, y + 1, queue);
    if x > 0 {
        dfs(grid, x - 1, y, queue);
    }
    if y > 0 {
        dfs(grid, x, y - 1, queue);
    }
}

pub fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    println!("{}", shortest_bridge(grid));
}
