use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let regex_game = Regex::new(r"Game (\d+):").unwrap();
    let regex_red = Regex::new(r"(\d+) red").unwrap();
    let regex_green = Regex::new(r"(\d+) green").unwrap();
    let regex_blue = Regex::new(r"(\d+) blue").unwrap();

    // Import input file
    let input = read_to_string("../input.txt").unwrap();
    
    let mut game_sum: i32 = 0;
    input.split_terminator('\n').for_each(|line| {
        let red: Vec<_> = regex_red.captures_iter(line)
            .map(|c| c.extract::<1>())
            .map(|c| c.1[0].parse::<i32>().unwrap()).collect();

        let green: Vec<_> = regex_green.captures_iter(line)
            .map(|c| c.extract::<1>())
            .map(|c| c.1[0].parse::<i32>().unwrap()).collect();

        let blue: Vec<_> = regex_blue.captures_iter(line)
            .map(|c| c.extract::<1>())
            .map(|c| c.1[0].parse::<i32>().unwrap()).collect();

        let power: i32 = {
            let mut min_red = 0;
            red.iter().for_each(|&r| {
                if min_red < r {
                    min_red = r;
                }
            });

            let mut min_green = 0;
            green.iter().for_each(|&g| {
                if min_green < g {
                    min_green = g;
                }
            });

            let mut min_blue = 0;
            blue.iter().for_each(|&b| {
                if min_blue < b {
                    min_blue = b;
                }
            });

            min_red * min_green * min_blue
        };

        game_sum += power;
    });

    println!("{:?}", game_sum);
}