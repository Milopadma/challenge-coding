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

// // explanation from gpt4
// 1. In the `dfs` function, the boundary check and the condition to return
//    `true` if the current cell is water (1) are combined into a single
//    condition at the beginning of the function. This simplifies the logic and
//    reduces the number of checks needed.

// 2. The boundary check is updated to use the correct comparison operators,
//    ensuring that it only returns `false` if the current cell is land (0) and
//    is on the boundary of the grid.

// 3. The `dfs` function now checks the four neighboring cells (top, bottom,
//    left, and right) separately and stores the individual results. The final
//    return value is the logical AND of all four results, which will only be
//    `true` if all neighboring cells are surrounded by water (1) or are within
//    the boundaries.

// 4. The traversal loop in the `closed_islands` function remains unchanged, as
//    it iterates through the grid and calls the `dfs` function for each cell
//    with a value of 0 (land). The count of closed islands is incremented only
//    if the `dfs` function returns `true` for the current cell.

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
