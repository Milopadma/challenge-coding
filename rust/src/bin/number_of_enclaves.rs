mod solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                count = count + bfs(&mut grid, i, j);
            }
        }
        count
    }

    // basifc bfs with check to see if the node is at the edge,
    // if it is, then we don't count it nad dont count any of its neighbors
    fn bfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if grid[i][j] == 0 {
            return 0;
        }
        let mut count = 0;
        let mut queue = vec![(i, j)];
        while !queue.is_empty() {
            let (i, j) = queue.pop().unwrap();
            if grid[i][j] == 0 {
                continue;
            }
            if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1 {
                return 0;
            }
            grid[i][j] = 0;
            count += 1;
            if i > 0 {
                queue.push((i - 1, j));
            }
            if i < grid.len() - 1 {
                queue.push((i + 1, j));
            }
            if j > 0 {
                queue.push((i, j - 1));
            }
            if j < grid[0].len() - 1 {
                queue.push((i, j + 1));
            }
        }
        count
    }
}
pub fn main() {
    let grid = vec![
        vec![0, 0, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
    ];
    println!("{}", solution::num_enclaves(grid));
}
