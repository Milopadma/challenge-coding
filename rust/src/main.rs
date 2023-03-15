mod team_points {
    #[allow(dead_code)]
    pub fn points(games: &[String]) -> u32 {
        let mut total: u32 = 0;
        games.iter().for_each(|game| {
            let mut score = game.split(":");
            let home = score.next().unwrap().parse::<u32>().unwrap();
            let away = score.next().unwrap().parse::<u32>().unwrap();
            match home.cmp(&away) {
                std::cmp::Ordering::Greater => {
                    total += 3;
                }
                std::cmp::Ordering::Less => {
                    total += 0;
                }
                std::cmp::Ordering::Equal => {
                    total += 1;
                }
            }
        });
        return total;
    }
}

mod convert_from_string {
    #[allow(dead_code)]
    pub fn convert_from_string(str: &str) -> i32 {
        //parse to i32
        let num = str.parse::<i32>().unwrap();
        return num;
    }
}

mod count_by {
    #[allow(dead_code)]
    pub fn count_by(x: u32, n: u32) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        for i in 1..=n {
            result.push(x * i);
        }
        return result;
    }
}

mod rep_max_calc {
    pub fn calculate_1_rm(w: i32, r: i32) -> i32 {
        match r {
            0 => {
                return 0;
            }
            1 => {
                return w;
            }
            _ => test_each_formulas(w, r),
        }
    }
    fn test_each_formulas(w: i32, r: i32) -> i32 {
        // array of f64
        let mut array: [f64; 3] = [0_f64; 3];
        array[0] = ((w as f64) * (1_f64 + (r as f64) / 30_f64)).round(); // epley
                                                                         // array[0] = (w as f64) * f64::from(r).powf(0.1_f64);
        array[1] = ((f64::from(100) * f64::from(w)) / (101.3 - 2.67123 * f64::from(r))).ceil(); // mcglothin
        array[2] = (f64::from(w) * ((r as f64).powf(0.1) as f64)).round(); // lombardi
        println!("WEIGHT: {:}, REPS: {:?}: {:?}", w, r, array);

        return array.iter().map(|&x| x as i32).max().unwrap();
    }
}

mod threes_fives {
    #[allow(dead_code)]
    pub fn solution(num: i32) -> i32 {
        let mut sum = 0;
        for i in 1..num {
            match i % 3 {
                0 => {
                    sum += i;
                }
                _ => match i % 5 {
                    0 => {
                        sum += i;
                    }
                    _ => {}
                },
            }
        }
        return sum;
    }

    // a better solution i found on codewars
    #[allow(dead_code)]
    pub fn solution_better(num: i32) -> i32 {
        (1..num)
            .into_iter()
            .filter(|&n| (n % 3 == 0 || n % 5 == 0))
            .sum()
    }
}

mod move_zeroes {
    pub fn move_zeroes(nums: &[u8]) -> Vec<u8> {
        let mut i = 0;
        let mut j = 0;
        let mut new_nums = nums.clone().to_vec();
        while i < nums.len() {
            if nums[i] != 0 as u8 {
                new_nums.swap(i, j); // swap the values if the current value is not 0
                j += 1;
                // eventually all the non-zero values will be at the beginning of the array
                dbg!(&new_nums);
            }
            i += 1;
        }

        return new_nums;
    }

    // functionally the same as above but is a bit more concise for functional programming enthusiasts
    use std::iter;
    #[allow(dead_code)]
    fn move_zeros(arr: &[u8]) -> Vec<u8> {
        arr.iter()
            .cloned()
            .filter(|&x| x != 0)
            .chain(iter::repeat(0))
            .take(arr.len())
            .collect()
    }
}

mod pyramid_slide {
    // 4 kyu
    fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
        let mut pyramid = pyramid.to_vec(); // turns to vector
        for i in (0..pyramid.len() - 1).rev() {
            // why -1? because without it we would be out of bounds
            // since we are doing i + 1 comparisons
            // take the length of the pyramid and subtract 1 to get the last index
            // and then iterate backwards
            for j in 0..pyramid[i].len() {
                // at the base of the pyramid array,
                // find the max of the two values and add it to the current value
                pyramid[i][j] += pyramid[i + 1][j].max(pyramid[i + 1][j + 1]);
                dbg!(&pyramid);
            }
        }
        return pyramid[0][0];
    }

    // this way, the iteration walks through the pyramid from the bottom up and
    // progressively adds the max of the two values to the current value it is at
    // which basically propagates the max value up the pyramid
    // whilst also keeping the rule of only being able to check the near values

    pub fn run() {
        let pyramid = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        println!("{}", longest_slide_down(&pyramid));
    }

    // solution taken from pmengelbert on codewars
    // appeases to the functional programming style
    use std::cmp::max;
    #[allow(dead_code)]
    fn longest_slide_down_func(pyramid: &[Vec<u16>]) -> u16 {
        pyramid
            .iter()
            .rev()
            .skip(1)
            .fold(pyramid[pyramid.len() - 1].to_vec(), |acc, v| {
                v.iter()
                    .enumerate()
                    .map(|(i, n)| n + max(acc[i], acc[i + 1]))
                    .collect::<Vec<u16>>()
            })[0]
    }
}

mod who_likes_it {
    // 6 kyu
    pub fn likes(names: &[&str]) -> String {
        match names.len() {
            0 => "no one likes this".to_string(),
            1 => format!("{} likes this", names[0]),
            2 => format!("{} and {} like this", names[0], names[1]),
            3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
            _ => format!(
                "{}, {} and {} others like this",
                names[0],
                names[1],
                names.len() - 2
            ),
        }
    }

    pub fn run() {
        let names = vec!["Peter", "Jacob", "Alex", "Max", "John", "Mark"];
        println!("{}", likes(&names));
    }
}

mod ten_minute_walk {
    fn is_valid_walk(walk: &[char]) -> bool {
        // array to store the coords
        let mut coords = vec![0, 0];

        // early return
        if walk.len() != 10 {
            return false;
        }

        for char in walk {
            match char {
                'n' => coords[1] += 1,
                's' => coords[1] -= 1,
                'e' => coords[0] += 1,
                'w' => coords[0] -= 1,
                _ => (),
            };

            println!("{:?}", coords);
        }

        if coords[0] == 0 && coords[1] == 0 {
            true
        } else {
            false
        }
    }

    pub fn test() {
        let walk = ['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'];
        println!("{}", is_valid_walk(&walk));
    }
}

mod rgb_to_hex {
    pub fn rgb(r: i32, g: i32, b: i32) -> String {
        format!(
            "{:02X}{:02X}{:02X}",
            r.max(0).min(255),
            g.max(0).min(255),
            b.max(0).min(255)
        )
    }
}

mod factorials {
    fn factorial(n: u64) -> u64 {
        if n == 0 {
            return 1;
        }
        // recursion call to the function itself until n == 0
        // then it will return 1
        return n * factorial(n - 1);
    }

    // same as above but using a for loop
    fn looping_factorial(n: u64) -> u64 {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        return result;
    }

    // to find the trailing zeros of a factorial
    fn trailing_zeros(n: u64) -> u64 {
        let mut result = 0;
        let mut n = n;
        // while n is greater than 0
        while n > 0 {
            n /= 5; // divide n by 5, why do we divide by 5?
                    // because 5 is the smallest prime factor of 10
                    // so if we divide by 5, we will get the number of 5s in the number
                    // and since 10 = 2 * 5, we can get the number of 10s in the number
                    // by dividing by 5
                    // https://www.purplemath.com/modules/factzero.htm
            result += n; // add n to result
                         // repeats until n is below 0
            println!("n: {}, result: {}", n, result);
        }
        return result;
    }

    pub fn run() {
        println!("{}", factorial(6));
        println!("{}", looping_factorial(6));
        println!("{}", trailing_zeros(6));
        println!("{}", trailing_zeros(1000));
    }
}

mod molecule_to_atoms {
    use thiserror::Error;

    pub type Atom = (String, usize);
    pub type Molecule = Vec<Atom>;

    #[derive(Error, Debug)]
    pub enum ParseError {
        #[error("invalid molecule")]
        InvalidMolecule,
        #[error("Mismatched parentheses")]
        MismatchedParentheses,
    }

    pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
        // https://www.codewars.com/kata/52f831fa9d332c6591000511
        let mut molecule = vec![];
        // just straight up invalidate lowercase first chars
        if s.chars().nth(0).unwrap().is_lowercase() {
            return Err(ParseError::InvalidMolecule);
        } else {
            // turn s into a tuple of chars and the number next to it
            // if there is no number next to it, then it is 1
            let mut chars = s.chars().peekable();
            // peekable so we can peek at the next char without consuming it
            let mut num = 1;
            let mut atom = String::new();
            while let Some(c) = chars.next() {
                if c.is_uppercase() {
                    if !atom.is_empty() {
                        molecule.push((atom, num));
                        atom = String::new();
                        num = 1;
                    }
                    atom.push(c);
                } else if c.is_lowercase() {
                    atom.push(c);
                } else if c.is_numeric() {
                    num = c.to_digit(10).unwrap() as usize;
                    if let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            num = num * 10 + c.to_digit(10).unwrap() as usize;
                            chars.next();
                        }
                    }
                } else if c == '(' {
                    let mut count = 1;
                    let mut sub = String::new();
                    while let Some(c) = chars.next() {
                        if c == '(' {
                            count += 1;
                        } else if c == ')' {
                            count -= 1;
                        }
                        if count == 0 {
                            break;
                        }
                        sub.push(c);
                    }
                    let sub = parse_molecule(&sub)?;
                    for (atom, n) in sub {
                        molecule.push((atom, n * num));
                    }
                    num = 1;
                } else {
                    return Err(ParseError::InvalidMolecule);
                }
            }
            if !atom.is_empty() {
                molecule.push((atom, num));
            }
        }
        Ok(molecule)
    }

    // best practice solution from codewars
    // https://www.codewars.com/kata/reviews/58acf5ad530a19d7580006ff/groups/63f3feb1d8d7350001b984e2
}

mod insertion_sort_list_leetcode {
    use std::mem;
    //  Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // Create a new node with the given value
        let mut new_node = Box::new(ListNode::new(val));

        // If the list is empty, return the new node
        if head.is_none() {
            return Some(new_node);
        }

        let mut current = &mut head;

        // Traverse the list until we find the correct position to insert the new node
        while let Some(node) = current.as_deref_mut() {
            if node.val > val {
                new_node.clone().next = mem::replace(current.as_mut(), Some(new_node.clone()));
                break;
            }
            current = &mut node.next;
        }

        // If we reached the end of the list, just append the new node
        if current.clone().is_none() {
            mem::replace(current, Some(new_node));
        }

        head
    }

    fn insertion_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted = None;

        // Traverse the original list and insert each node into the sorted list
        let mut current = head;
        while let Some(mut node) = current {
            current = node.next.take();
            sorted = insertion_sort_list(sorted, node.val);
        }

        sorted
    }

    mod two_sum_leetcode {
        pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut result = vec![];
            for i in 0..nums.len() - 1 {
                if nums[i] + nums[i + 1] == target {
                    nums.remove(i);
                    result.push(i as i32);
                } else {
                    continue;
                }
            }
            result
        }
    }
}

fn main() {
    // mod 1 test case
    // let games = vec![
    //     "3:1".to_string(),
    //     "2:2".to_string(),
    //     "0:1".to_string(),
    //     "1:0".to_string(),
    //     "4:0".to_string(),
    //     "2:1".to_string(),
    //     "1:3".to_string(),
    //     "0:0".to_string(),
    //     "0:2".to_string(),
    //     "1:2".to_string()
    // ];
    // println!("{}", team_points::points(&games));

    // mod 2 test case
    // let str = "123";
    // println!("{}", convert_from_string::convert_from_string(str));

    // mod 3 test case
    // let x = 2;
    // let n = 5;
    // println!("{:?}", count_by::count_by(x, n));

    // mod 4 test case
    println!("{}", rep_max_calc::calculate_1_rm(135, 20)); // 282
    println!("{}", rep_max_calc::calculate_1_rm(200, 8)); // 253
    println!("{}", rep_max_calc::calculate_1_rm(270, 2)); // 289
                                                          // println!("{}", rep_max_calc::calculate_1_rm(360, 1)); // 360
                                                          // println!("{}", rep_max_calc::calculate_1_rm(400, 0)); // 0

    //
    // println!("{}", threes_fives::solution(10)); // 23

    // mod 6 test case
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes::move_zeroes(&mut nums);
    println!("{:?}", move_zeroes::move_zeroes(&mut nums));

    // mod 7 test case
    pyramid_slide::run();

    // mod 8 test case
    who_likes_it::run();

    // mod 9 test case
    ten_minute_walk::test();

    // mod 10 test case
    println!("{}", rgb_to_hex::rgb(0, 255, 0)); // 000000
    println!("{}", rgb_to_hex::rgb(0, 0, -20)); // 000000

    // mod 11 test case
    factorials::run();

    // mod 12 test case
    println!("{:?}", molecule_to_atoms::parse_molecule("H2O"));
}
