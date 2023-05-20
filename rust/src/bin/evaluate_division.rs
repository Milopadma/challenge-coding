// what the fuck is this
pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut graph = std::collections::HashMap::new();
    // iterate over the equations
    // and store the knowns in a graph
    for (i, equation) in equations.iter().enumerate() {
        let (a, b) = (&equation[0], &equation[1]);
        graph
            .entry(a)
            .or_insert_with(std::collections::HashMap::new)
            .insert(b, values[i]);
        // the second eq letter will be 1 divide by value
        graph
            .entry(b)
            .or_insert_with(std::collections::HashMap::new)
            .insert(a, 1.0 / values[i]);
    }
    let mut result = Vec::new();
    for query in queries {
        let (a, b) = (&query[0], &query[1]);
        if !graph.contains_key(a) || !graph.contains_key(b) {
            result.push(-1.0);
            continue;
        }
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((a, 1.0));
        visited.insert(a);
        let mut found = false;
        while !queue.is_empty() {
            let (node, value) = queue.pop_front().unwrap();
            if node == b {
                result.push(value);
                found = true;
                break;
            }
            for (next, next_value) in graph.get(node).unwrap() {
                if !visited.contains(next) {
                    queue.push_back((next, value * next_value));
                    visited.insert(next);
                }
            }
        }
        if !found {
            result.push(-1.0);
        }
    }
    result
}

pub fn main() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    let result = calc_equation(equations, values, queries);
    println!("{:?}", result);
}
