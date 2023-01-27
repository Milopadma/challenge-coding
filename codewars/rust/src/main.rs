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

fn main() {
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
}