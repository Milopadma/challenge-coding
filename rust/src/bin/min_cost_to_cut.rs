pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
    // add 0 and n to cuts for starting and ending
    cuts.push(0);
    cuts.push(n);
    cuts.sort();
    let n = cuts.len();
    let mut cost: Vec<Vec<i32>> = vec![vec![0; n]; n]; // new length

    // sliding window
    for l in 2..n {
        for i in 0..n - l {
            let j = i + l;
            cost[i][j] = i32::MAX; // this sets the max value for the cost
                                   // see cuts
            for k in i + 1..j {
                // calc cost
                // by. adding the cost of the left and right
                cost[i][j] = cost[i][j].min(cost[i][k] + cost[k][j] + cuts[j] - cuts[i]);
            }
        }
    }

    cost[0][n - 1]
}

pub fn main() {
    let n = 7;
    let cuts = vec![1, 3, 4, 5];
    println!("{}", min_cost(n, cuts));
}
