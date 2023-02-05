mod team_points {
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
    pub fn convert_from_string(str: &str) -> i32 {
        //parse to i32
        let num = str.parse::<i32>().unwrap();
        return num;
    }
}

mod count_by {
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

    // mod 5 test case
    // println!("{}", threes_fives::solution(10)); // 23

    // mod 6 test case
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes::move_zeroes(&mut nums);
    println!("{:?}", move_zeroes::move_zeroes(&mut nums));

    // mod 7 test case
    pyramid_slide::run();

    // mod 8 test case
    who_likes_it::run();
}
