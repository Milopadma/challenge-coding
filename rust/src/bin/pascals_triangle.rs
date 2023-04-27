pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    // creating the pascal triangle vector and returning the rows
    let mut pascal_triangle: Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows {
        pascal_triangle.push(Vec::new());
        for j in 0..=i {
            if j == 0 || j == i {
                pascal_triangle[i as usize].push(1);
            } else {
                let temp = pascal_triangle[(i - 1) as usize][(j - 1) as usize]
                    + pascal_triangle[(i - 1) as usize][j as usize];
                pascal_triangle[i as usize].push(temp);
            }
        }
    }
    pascal_triangle
}

pub fn main() {
    // test cases
    println!("{:?}", generate(5));
}
