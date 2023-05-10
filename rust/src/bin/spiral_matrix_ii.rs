// function that given a non negative  number;
// generates a proper matrix
// then iterates to return the spiral of said matrix
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0; n as usize]; n as usize];
    let mut row_start = 0;
    let mut row_end = n - 1;
    let mut col_start = 0;
    let mut col_end = n - 1;
    let mut num = 1;

    // similar iteration technique to previoius spiral matrix
    // but this one sets the num and increments it for every
    // iteration, the other one just pushes
    while row_start <= row_end && col_start <= col_end {
        for i in col_start..=col_end {
            result[row_start as usize][i as usize] = num;
            num += 1;
        }
        row_start += 1;

        for i in row_start..=row_end {
            result[i as usize][col_end as usize] = num;
            num += 1;
        }
        col_end -= 1;

        if row_start <= row_end {
            for i in (col_start..=col_end).rev() {
                result[row_end as usize][i as usize] = num;
                num += 1;
            }
        }
        row_end -= 1;

        if col_start <= col_end {
            for i in (row_start..=row_end).rev() {
                result[i as usize][col_start as usize] = num;
                num += 1;
            }
        }
        col_start += 1;
    }

    result
}

pub fn main() {
    let n = 3;
    let matrix = generate_matrix(n);
    println!("{:?}", matrix);
}
