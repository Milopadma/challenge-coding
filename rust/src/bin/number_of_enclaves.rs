fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    // check if the 1 is surrounded by 0s or other 1s, in which case,
    // it is an enclave
    if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1 {
        return 0;
    }

    if grid[i][j] == 1 {
        grid[i][j] = 2;
        let mut count = 1;
        count += dfs(grid, i + 1, j);
        count += dfs(grid, i - 1, j);
        count += dfs(grid, i, j + 1);
        count += dfs(grid, i, j - 1);
        count
    } else {
        0
    }
}

pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] == 0 {
                count = count + dfs(&mut grid, i, j);
            }
        }
    }
    count
}

pub fn main() {
    let grid = vec![
        vec![0, 0, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
    ];
    println!("{}", num_enclaves(grid));
}
