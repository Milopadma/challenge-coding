pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut color = vec![0; graph.len()];
    for i in 0..graph.len() {
        if color[i] == 0 && !dfs(&graph, &mut color, i, 1) {
            return false;
        }
    }
    true
}

fn dfs(graph: &Vec<Vec<i32>>, color: &mut Vec<i32>, node: usize, c: i32) -> bool {
    if color[node] != 0 {
        return color[node] == c;
    }
    color[node] = c;
    for &neighbor in &graph[node] {
        if !dfs(graph, color, neighbor as usize, -c) {
            return false;
        }
    }
    true
}

pub fn main() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    println!("is_bipartite: {}", is_bipartite(graph));
}
