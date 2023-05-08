// function that takes a square matrix, iterate add the
// diagonal elements return the sum
// (both diagonals)
pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in 0..mat.len() {
        // if mat[i] == mat[mat.len() - i - 1] is true, then thats the middle element
        // so we subtract it from the sum
        if i == mat.len() - i - 1 {
            sum -= mat[i][i];
        }
        sum += mat[i][i];
        sum += mat[i][mat.len() - i - 1];
    }
    sum
}

pub fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("Sum of diagonal elements: {}", diagonal_sum(mat));
}
