impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut diff = 0;
        let mut diff1 = 0;
        let mut diff2 = 0;
        let mut map = [0; 26];
        for i in 0..s.len() {
            let c1 = s.as_bytes()[i];
            let c2 = goal.as_bytes()[i];
            if c1 != c2 {
                diff += 1;
                if diff == 1 {
                    diff1 = i;
                } else if diff == 2 {
                    diff2 = i;
                } else {
                    return false;
                }
            }
            map[(c1 - b'a') as usize] += 1;
        }
        if diff == 0 {
            for i in 0..26 {
                if map[i] > 1 {
                    return true;
                }
            }
            return false;
        }
        if diff == 1 {
            return false;
        }
        if s.as_bytes()[diff1] == goal.as_bytes()[diff2]
            && s.as_bytes()[diff2] == goal.as_bytes()[diff1]
        {
            return true;
        }
        false
    }
}
