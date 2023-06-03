struct Solution;

impl Solution {
    fn traverse_tree(
        subordinates: &Vec<Vec<i32>>,
        inform_time: &Vec<i32>,
        current_employee: i32,
        time_so_far: i32,
        max_time: &mut i32,
    ) {
        let mut new_time = time_so_far + inform_time[current_employee as usize];
        if new_time > *max_time {
            *max_time = new_time;
        }
        for subordinate in &subordinates[current_employee as usize] {
            Solution::traverse_tree(subordinates, inform_time, *subordinate, new_time, max_time);
        }
    }

    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut subordinates: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        for (i, &man) in manager.iter().enumerate() {
            if man != -1 {
                subordinates[man as usize].push(i as i32);
            }
        }
        let mut max_time = 0;
        Solution::traverse_tree(&subordinates, &inform_time, head_id, 0, &mut max_time);
        max_time
    }
}

pub fn main() {
    todo!()
}
