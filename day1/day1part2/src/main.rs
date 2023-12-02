use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let replace_list = [
        (Regex::new(r"one").unwrap(), "one1one"),
        (Regex::new(r"two").unwrap(), "two2two"),
        (Regex::new(r"three").unwrap(), "three3three"),
        (Regex::new(r"four").unwrap(), "four4four"),
        (Regex::new(r"five").unwrap(), "five5five"),
        (Regex::new(r"six").unwrap(), "six6six"),
        (Regex::new(r"seven").unwrap(), "seven7seven"),
        (Regex::new(r"eight").unwrap(), "eight8eight"),
        (Regex::new(r"nine").unwrap(), "nine9nine"),
    ];

    let number_filter: Regex = Regex::new(r"([0-9])").unwrap();

    // Import input file
    let mut input = read_to_string("../input.txt").unwrap();

    // Replace all occurances of word-digits in input
    for (regex, replacement) in replace_list {
        input = regex.replace_all(&input, replacement).to_string();
    }

    println!("{}", input);

    // Calculate number for each line
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

