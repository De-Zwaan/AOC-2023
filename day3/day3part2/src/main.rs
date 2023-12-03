use std::fs::read_to_string;
use regex::Regex;

fn main() {
    // Regexes
    let symbol = Regex::new(r"\*").unwrap();
    let number = Regex::new(r"(\d+)").unwrap();
    let digit = Regex::new(r"\d").unwrap();
    
    // List of relative (y, x) indices for the eight neighbouring cells
    let around: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    // The puzzle input
    let input = read_to_string("../input.txt").unwrap();

    // Rows of the puzzle input
    let rows: Vec<&str> = input.split_terminator('\n').collect();

    // Width and height of puzzle input
    let height = rows.len();
    let width = rows.iter().map(|&l| l.chars().count()).collect::<Vec<usize>>()[0];

    let mut result = 0;

    // Iterate over all cells 
    rows.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, char)| {
            let is_symbol = symbol.is_match(char.to_string().as_str());
            let mut surrounding_numbers: Vec<i32> = Vec::new();

            // If the current cell is a *
            if is_symbol {
                // Iterate over the 8 neighboring cells
                for (dy, dx) in around {
                    // Stop if out of bounds
                    if (i as isize + dy) < 0 || (i as isize + dy) >= height as isize {continue;}
                    if (j as isize + dx) < 0 || (j as isize + dx) >= width as isize {continue;}
    
                    println!("{}, ({}, {}) + ({}, {})", char, i, j, dy, dx);
    
                    // Get the character at the neighbouring cell
                    let c = rows[(i as isize + dy) as usize].chars().collect::<Vec<char>>()[(j as isize + dx) as usize];
                    
                    // If this character is a digit, look in the row of the cell for all the numbers
                    if digit.is_match(c.to_string().as_str()) {

                        // Get the numbers of the row of the digit to find the whole number
                        let numbers: Vec<(&str, std::ops::Range<_>)> = number.find_iter(rows[(i as isize + dy) as usize]).map(|m| (m.as_str(), m.range())).collect();

                        // Check for all numbers if it is the number that we are looking for
                        for (number, location) in numbers {
                            println!("{:?} {:?}", number, location);
                            if location.contains(&((j as isize + dx) as usize)) {
                                // Add it to the list of surrounding numbers
                                surrounding_numbers.push(number.parse::<i32>().unwrap());
                            }
                        }
                    }
                }
            }

            // Remove duplicates (hope that there are no two numbers the same around a * in the puzzle input)
            surrounding_numbers.dedup(); 

            // Multiply the two numbers and add them to the total
            if surrounding_numbers.len() == 2 {
                result += surrounding_numbers[0] * surrounding_numbers[1];
            }
        });
    });

    println!("{:?}", result)
}
