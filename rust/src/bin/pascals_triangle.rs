// Title: Pascal's Triangle
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    // creating the pascal triangle vector and returning the rows
    let mut pascal_triangle: Vec<Vec<i32>> = Vec::new();
    // iterating through the rows
    for i in 0..num_rows {
        pascal_triangle.push(Vec::new());
        // iterating through the columns
        for j in 0..=i {
            if j == 0 || j == i {
                // if the column is the first or last, then push 1
                pascal_triangle[i as usize].push(1);
            } else {
                // temp variable to store the sum of the two numbers above
                let temp = pascal_triangle[(i - 1) as usize][(j - 1) as usize]
                    + pascal_triangle[(i - 1) as usize][j as usize];
                // then push the temp variable to the vector
                pascal_triangle[i as usize].push(temp);
            }
        }
    }
    // returning the pascal triangle
    pascal_triangle
}

pub fn main() {
    // test cases
    println!("{:?}", generate(5));
}
