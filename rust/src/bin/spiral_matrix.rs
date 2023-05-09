// function that takes in a matrix
// and returns an array with
// the elements in spiral order
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // check of edge cases [[n]] || [[n,n]] || [[n], [n], [n]] cases
    if matrix.len() == 1 {
        return matrix[0].clone();
    } else if matrix.len() == 2 {
        let mut result: Vec<i32> = Vec::new();
        result.extend(matrix[0].clone());
        result.extend(matrix[1].clone().into_iter().rev());
        return result;
    } else if matrix[0].len() == 1 {
        let mut result: Vec<i32> = Vec::new();
        for i in 0..matrix.len() {
            result.push(matrix[i][0]);
        }
        return result;
    }

    let mut result: Vec<i32> = Vec::new();
    // by keeping track on the starts and ends of the rows and columns
    let mut row_start: usize = 0;
    let mut row_end: usize = matrix.len() - 1;
    let mut col_start: usize = 0;
    let mut col_end: usize = matrix[0].len() - 1;

    // if the row start index is greater than the row end index
    // and the column start index is greater than the column end index
    while row_start <= row_end && col_start <= col_end {
        // top row
        for i in col_start..=col_end {
            result.push(matrix[row_start][i]);
        }
        row_start += 1;

        // right column
        for i in row_start..=row_end {
            result.push(matrix[i][col_end]);
        }
        col_end -= 1;

        // bottom row
        if row_start <= row_end {
            for i in (col_start..=col_end).rev() {
                result.push(matrix[row_end][i]);
            }
        }
        row_end -= 1;

        // left column
        if col_start <= col_end {
            for i in (row_start..=row_end).rev() {
                result.push(matrix[i][col_start]);
            }
        }
        col_start += 1;
    }

    result
}

pub fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = spiral_order(matrix);
    println!("{:?}", result);
}
