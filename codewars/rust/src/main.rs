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

mod convert_from_String {
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
        let mut array: [f64; 3] = [0.0, 0.0, 0.0];
        let val_formula_1: f64 = (w * (1 + r / 30)).into(); // epley
        let val_formula_2: f64 = (f64::from(100) * f64::from(w)) / (101.3 - 2.67123 * f64::from(r)); // mcglothin
        let val_formula_3: f64 = (w * r.pow(0.1 as u32)).into(); // lombardi
        array[0] = val_formula_1;
        array[1] = val_formula_2;
        array[2] = val_formula_3;
        println!("{:?}", array);
        return array
            .iter()
            .map(|&x| x as i32)
            .max()
            .unwrap();
    }
}

fn main() {
    // mod 1 test case
    let games = vec![
        "3:1".to_string(),
        "2:2".to_string(),
        "0:1".to_string(),
        "1:0".to_string(),
        "4:0".to_string(),
        "2:1".to_string(),
        "1:3".to_string(),
        "0:0".to_string(),
        "0:2".to_string(),
        "1:2".to_string()
    ];
    println!("{}", team_points::points(&games));

    // mod 2 test case
    let str = "123";
    println!("{}", convert_from_String::convert_from_string(str));

    // mod 3 test case
    let x = 2;
    let n = 5;
    println!("{:?}", count_by::count_by(x, n));

    // mod 4 test case
    println!("{}", rep_max_calc::calculate_1_rm(135, 20)); //282
    println!("{}", rep_max_calc::calculate_1_rm(200, 8)); //253
    println!("{}", rep_max_calc::calculate_1_rm(270, 2)); //289
    println!("{}", rep_max_calc::calculate_1_rm(360, 1)); //360
    println!("{}", rep_max_calc::calculate_1_rm(400, 0)); //0
}