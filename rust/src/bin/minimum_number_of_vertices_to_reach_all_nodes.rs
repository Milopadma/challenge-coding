pub fn find_smallest_set_of_verteces(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    // the indegree is the number of edges pointing to a node
    let mut indegree = vec![0; n as usize];
    // iterate through every node edges and count the indegrees for that node
    for edge in edges {
        indegree[edge[1] as usize] += 1;
    }
    // if the indegree is 0, then it means that there is no edge pointing to that node
    // change from for loop to a filter and map for a 16% speed increase
    indegree
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == 0)
        .map(|(i, _)| i as i32)
        .collect()
}
pub fn main() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
    let res = find_smallest_set_of_verteces(n, edges);
    println!("{:?}", res);
}
