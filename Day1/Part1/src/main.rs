#![allow(non_snake_case)]

use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let number_filter: Regex = Regex::new(r"([0-9])").unwrap();

    let input = read_to_string("../input.txt").unwrap();
    
    let results: Vec<Option<i32>> = input.split_terminator("\n").map(|line| {
        let mut numbers = number_filter
            .captures_iter(line)
            .map(|c| { c.extract() });

            let first = numbers.next().unwrap_or(("", [""])).0;
            let last = numbers.last().unwrap_or((first, [""])).0;

        // let first = &numbers.map(||).unwrap().extract().0;?
        let result = format!("{first}{last}").parse().unwrap();
        if result < 10 { None } else { Some(result) }
    }).collect();

    let mut total = 0;
    for r in results {
        total += r.unwrap_or(0);
    }

    println!("{:?}", total);
}

