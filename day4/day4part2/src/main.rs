use std::{fs::read_to_string, collections::HashMap};
use regex::Regex;

fn main() {

    let input = read_to_string("../input.txt").unwrap();
// "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();

    let original_cards: Vec<&str> = input.split_terminator('\n').collect();
    let mut card_wins: Vec<i32> = Vec::new();

    // let mut result = 0;

    for card in original_cards {
        let (card_number, winning, numbers) = get_card_data(card);
        
        println!("{:?}: {:?} | {:?}", card_number, winning, numbers);

        let wins = get_num_wins(winning, numbers);
        
        card_wins.push(wins);
    }

    println!("{:?}", card_wins);


    /* 
    -> [4, 2, 2, 1, 0, 0]:
        [1, 1, 1, 1, 1, 1] + 1 * [0, 1, 1, 1, 1, 0]
        [1, 2, 2, 2, 2, 1] + 2 * [0, 0, 1, 1, 0, 0]
        [1, 2, 4, 4, 2, 1] + 4 * [0, 0, 0, 1, 1, 0]
        [1, 2, 4, 8, 6, 1] + 8 * [0, 0, 0, 0, 1, 0]
        [1, 2, 4, 8,14, 1] + 1 * [0, 0, 0, 0, 0, 0]
    */

    let mut card_counts = vec![1; card_wins.len()];

    for (i, &card_win) in card_wins.iter().enumerate() {
        for n in 0..card_win {
            card_counts[i + n as usize + 1] += card_counts[i];
        }

        // println!("{:?}", card_counts);
    }

    let mut num_cards = 0;

    for card_count in card_counts {
        num_cards += card_count;
    }

    println!("{:?}", num_cards);
}

fn get_card_data(card: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let card_regex = Regex::new(r"Card +(\d+): ([ \d]+) \| ([ \d]+)").unwrap();

    let (card_number_string, winning_string, numbers_string) = card_regex.captures(card).map(|c| (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str())).unwrap();

    let card_number = card_number_string.parse::<i32>().unwrap(); 
    let winning_vec: Vec<i32> = winning_string.split_ascii_whitespace().map(|r| r.parse::<i32>().unwrap()).collect();
    let numbers_vec: Vec<i32> = numbers_string.split_ascii_whitespace().map(|r| r.parse::<i32>().unwrap()).collect();

    (card_number, winning_vec, numbers_vec)
}

fn get_num_wins(winning: Vec<i32>, numbers: Vec<i32>) -> i32 {
    let mut wins: i32 = 0;
    for number in numbers {
        if winning.contains(&number) {
            wins += 1;
        }
    }
    wins
}
