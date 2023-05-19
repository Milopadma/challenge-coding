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
