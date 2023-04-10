mod solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;

        // Mark all land cells connected to the boundary as 0 (sea)
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1)
                    && grid[i][j] == 1
                {
                    dfs(&mut grid, i, j);
                }
            }
        }

        // Count the remaining land cells
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }

        // By separating the marking of land cells connected to the boundary and
        // counting the remaining land cells, the modified solution correctly
        // handles the cases where land cells connected to the boundary are also
        // connected to other land cells not directly connected to the boundary.

        count
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
            return;
        }

        grid[i][j] = 0;

        if i > 0 {
            dfs(grid, i - 1, j);
        }
        if i < grid.len() - 1 {
            dfs(grid, i + 1, j);
        }
        if j > 0 {
            dfs(grid, i, j - 1);
        }
        if j < grid[0].len() - 1 {
            dfs(grid, i, j + 1);
        }
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

// what went wrong?
// prev code just skipped checks on all edge cells instead of
// including them in the check and this is why
// cases where edge land cells connected to inside land cells
// were not being properly passed.
