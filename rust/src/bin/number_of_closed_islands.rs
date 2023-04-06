pub fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 {
        return false;
    }
    if grid[i][j] == 1 {
        return true;
    }
    grid[i][j] = 1;
    let mut res = true;
    res &= dfs(grid, i + 1, j);
    res &= dfs(grid, i - 1, j);
    res &= dfs(grid, i, j + 1);
    res &= dfs(grid, i, j - 1);
    res
}

pub fn closed_islands(grid: Vec<Vec<i32>>) -> i32 {
    // 0: land
    // 1: water
    // find a closed island, where all 4 sides of 0 are 1s
    // return the number of closed islands
    let mut grid = grid;
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                if dfs(&mut grid, i, j) {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn main() {
    // test cases
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    println!("{:?}", closed_islands(grid));
}
