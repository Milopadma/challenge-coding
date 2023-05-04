// function that finds out how many senators are in each party
// and return the winning party either
// "Radiant" or "Dire"
pub fn predict_party_victory(senate: String) -> String {
    // use a vecdeque for efficient removal of elements from the front and back
    use std::collections::VecDeque;
    let mut radiant = VecDeque::new();
    let mut dire = VecDeque::new();
    let senate = senate.chars().collect::<Vec<char>>();
    let mut i = 0;
    // push the indices of the senators into the vecdeques
    while i < senate.len() {
        if senate[i] == 'R' {
            radiant.push_back(i); //e.g: [0, 2, 4, 6, 8]
        } else {
            dire.push_back(i); //e.g. [1, 3, 5, 7, 9]
        }
        i += 1;
    }
    // loop through the vecdeques and remove the first element from each
    while !radiant.is_empty() && !dire.is_empty() {
        let r = radiant.pop_front().unwrap();
        let d = dire.pop_front().unwrap();
        // if the index of the first element in the radiant vecdeque is less than
        // the index of the first element in the dire vecdeque, then push the index
        // along with the length of the senate into the radiant vecdeque
        if r < d {
            radiant.push_back(r + senate.len());
        } else {
            dire.push_back(d + senate.len());
        }
        // this loop slowly removes the elements until one of em is empty
    }
    if radiant.is_empty() {
        "Dire".to_string()
    } else {
        "Radiant".to_string()
    }
}

// tests for the function above
pub fn main() {
    println!("{}", predict_party_victory("RDRRDDDRR".to_string()));
}
