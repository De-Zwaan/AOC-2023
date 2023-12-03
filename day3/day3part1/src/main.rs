use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let symbol = Regex::new(r"[^\d^\.]").unwrap();
    let number = Regex::new(r"(\d+)").unwrap();
    
    let around: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let input = read_to_string("../input.txt").unwrap();
// "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..".to_string();

    let height = input.split_terminator('\n').count();
    let width = input.split_terminator('\n').map(|l| l.chars().count()).collect::<Vec<usize>>()[0];

    let mut part_map: Vec<Vec<bool>> = vec![vec![false; width]; height];

    input.split_terminator('\n').enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, char)| {
            let is_symbol = symbol.is_match(char.to_string().as_str());

            if is_symbol {
                for (dy, dx) in around {
                    if (i as isize + dy) < 0 || (i as isize + dy) >= height as isize {continue;}
                    if (j as isize + dx) < 0 || (j as isize + dx) >= width as isize {continue;}
    
                    println!("{}, ({}, {}) + ({}, {})", char, i, j, dy, dx);
    
                    part_map[(i as isize + dy) as usize][(j as isize + dx) as usize] = is_symbol;
                }
            }
        });
    });

    let mut result = 0;

    input.split_terminator('\n').enumerate().for_each(|(i, line)| {
        let finds: Vec<(&str, std::ops::Range<usize>)> = number.find_iter(line).map(|m| (m.as_str(), m.range())).collect();
        
        'find: for (n, range) in finds {
            for j in range {
                if part_map[i][j] {
                    println!("{}", n);
                    result += n.parse::<i32>().unwrap();
                    continue 'find;
                }
            }
        }
    });

    println!("{:?}", result)
}
