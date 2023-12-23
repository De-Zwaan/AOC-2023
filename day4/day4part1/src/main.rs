use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let re = Regex::new(r"Card +(\d+): ([ \d]+) \| ([ \d]+)").unwrap();

    let input = read_to_string("../input.txt").unwrap();
// "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();

    let rows: Vec<&str> = input.split_terminator('\n').collect();

    let mut result = 0;

    for row in rows {
        let (winning, numbers) = re.captures(row).map(|c| (c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str())).unwrap();

        let winning_set: Vec<i32> = winning.split_ascii_whitespace().map(|r| r.parse::<i32>().unwrap()).collect();
        let numbers_set: Vec<i32> = numbers.split_ascii_whitespace().map(|r| r.parse::<i32>().unwrap()).collect();
        
        println!("{:?} | {:?}", winning_set, numbers_set);

        let mut wins: i32 = 0;
        for number in numbers_set {
            if winning_set.contains(&number) {
                wins += 1;
            }
        }

        if wins > 0 {
            result += (2_i32).pow((wins - 1) as u32);
        }
    }

    println!("{}", result);
}
