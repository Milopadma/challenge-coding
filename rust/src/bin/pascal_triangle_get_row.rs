// pascals triangle get individual  row
pub fn get_row(row_index: i32) -> Vec<i32> {
    // row to be returned
    let mut row: Vec<i32> = Vec::new();

    // first row
    row.push(1);

    // loop through each row
    for i in 1..=row_index {
        // push 1 to the row
        row.push(1);

        // loop through each element in the row
        for j in (1..i).rev() {
            //.rev() because we are adding the previous element to the current element
            // add the previous element to the current element
            row[j as usize] += row[j as usize - 1];
        }
    }

    // return
    row
}

pub fn main() {
    // test case
    println!("{:?}", get_row(3));
}
