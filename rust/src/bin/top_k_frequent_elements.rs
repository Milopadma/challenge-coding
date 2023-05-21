// fn that takes in an integer array and an integer k and returns the k most
// frequent elements in the array
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // using a hashmap to keep track of the unique recurrences
    let mut map = std::collections::HashMap::new();
    // iterate through the array and add the elements to the hashmap
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    // create a binary heap and push the values from the hashmap into it
    // using a heap becasue we want to pop the k most frequent elements
    let mut heap = std::collections::BinaryHeap::new();
    for (key, value) in map {
        heap.push((value, key));
    }
    // pop the k most frequent elements from the heap and return them
    let mut result = Vec::new();
    for _ in 0..k {
        result.push(heap.pop().unwrap().1);
    }
    result
}

pub fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = top_k_frequent(nums, k);
    println!("{:?}", result);
}
