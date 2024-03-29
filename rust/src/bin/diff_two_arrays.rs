// function that takes in two integer arrays and returns
// the difference between the two arrays
// by giving the elements that are not common in both arrays
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer: Vec<Vec<i32>> = vec![vec![], vec![]];

    // let it be mut
    let mut nums1 = nums1;
    let mut nums2 = nums2;

    // and then remove dupes
    nums1.dedup();
    nums1.sort();
    nums2.dedup();
    nums2.sort();

    for i in 0..nums1.len() {
        if !nums2.contains(&nums1[i]) {
            answer[0].append(&mut vec![nums1[i]])
        }
    }

    for i in 0..nums2.len() {
        if !nums1.contains(&nums2[i]) {
            answer[1].append(&mut vec![nums2[i]])
        }
    }

    answer[0].dedup();
    answer[1].dedup();

    answer
}

pub fn better_find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![], vec![]];
    let mut set1 = std::collections::HashSet::new();
    let mut set2 = std::collections::HashSet::new();
    for n in nums1 {
        set1.insert(n);
    }
    for n in nums2 {
        set2.insert(n);
    }
    for n in set1.difference(&set2) {
        result[0].push(*n);
    }
    for n in set2.difference(&set1) {
        result[1].push(*n);
    }
    result
}
pub fn main() {
    let nums1 = vec![1, 2, 3, 4, 5, 6];
    let nums2 = vec![1, 3, 4, 6, 7];
    let answer = find_difference(nums1, nums2);
    println!("{:?}", answer);
}
