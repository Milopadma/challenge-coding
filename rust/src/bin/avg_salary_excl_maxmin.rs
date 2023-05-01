// function that takes a vec of salaries where the index is the employee's
// salary  and returns the average salary excluding the min and max salaries
pub fn average(salary: Vec<i32>) -> i64 {
    let mut min = salary[0];
    let mut max = salary[0];
    let mut sum = 0;
    // all in one loop iteration
    for i in salary.iter() {
        // find the min
        if *i < min {
            min = *i;
        }
        // find the max
        if *i > max {
            max = *i;
        }
        // get the sum
        sum += *i;
    }
    // now get the average (without the min and max)
    (sum - min - max) as i64 / (salary.len() as i32 - 2) as i64
}
pub fn main() {
    // test cases
    let salary = vec![4000, 3000, 1000, 2000];
    println!("Average salary excluding min and max: {}", average(salary));
}
