// depth first search algorithm to find out
// if the island is closed or not by checking
// if its at the edge, and
// if any of its 4 sides are 1s (up, down, left, right)
fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    if i < 0 || i >= grid.len() || j < 0 || j >= grid[0].len() || grid[i][j] == 1 {
        return true;
    }
    if (i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1) && grid[i][j] == 0 {
        return false;
    }

    grid[i][j] = 1;

    let top = dfs(grid, i - 1, j);
    let bottom = dfs(grid, i + 1, j);
    let left = dfs(grid, i, j - 1);
    let right = dfs(grid, i, j + 1);

    top && bottom && left && right
}

pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
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
    println!("{:?}", closed_island(grid));
}
