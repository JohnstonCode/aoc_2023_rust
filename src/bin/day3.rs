use regex::Regex;

struct Position(usize, usize);

fn main() {
    let input = include_str!("../../inputs/day3.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> String {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let rx = Regex::new(r"\d+").unwrap();

    for (row, line) in input.lines().enumerate() {
        for num in rx.find_iter(line) {
            numbers.push((num.as_str(), Position(row, num.start())))
        }

        for (col, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.push((ch, Position(row, col)))
            }
        }
    }

    let mut part_numbers = Vec::new();
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    for (num, start_pos) in numbers.iter() {
        let end_pos = Position(start_pos.0, start_pos.1 + num.len() as usize - 1);
        for (_, symbol_pos) in symbols.iter() {
            let neighbors = [
                Position(symbol_pos.0 - 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 - 1, symbol_pos.1),
                Position(symbol_pos.0 - 1, symbol_pos.1 + 1),
                Position(symbol_pos.0, symbol_pos.1 - 1),
                Position(symbol_pos.0, symbol_pos.1 + 1),
                Position(symbol_pos.0 + 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 + 1, symbol_pos.1),
                Position(symbol_pos.0 + 1, symbol_pos.1 + 1),
            ]
            .into_iter()
            .filter(|p| p.0 < num_rows && p.1 < num_cols)
            .collect::<Vec<Position>>();

            for neighbor in neighbors.iter() {
                if neighbor.0 == start_pos.0 && neighbor.1 >= start_pos.1 && neighbor.1 <= end_pos.1 {
                    part_numbers.push(num.parse::<usize>().unwrap());
                    break;
                }
            }
        }
    }

    part_numbers.iter().sum::<usize>().to_string()
}

fn part2(input: &str) -> String {
    let mut numbers = Vec::new();
    let mut gears = Vec::new();
    let rx = Regex::new(r"\d+").unwrap();

    for (row, line) in input.lines().enumerate() {
        for num in rx.find_iter(line) {
            numbers.push((num.as_str(), Position(row, num.start())))
        }

        for (col, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch == '*' {
                gears.push((ch, Position(row, col)))
            }
        }
    }

    let mut gear_ratios = Vec::new();
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    for (_, gear_pos) in gears.iter() {
        let neighbors = [
            Position(gear_pos.0 - 1, gear_pos.1 - 1),
            Position(gear_pos.0 - 1, gear_pos.1),
            Position(gear_pos.0 - 1, gear_pos.1 + 1),
            Position(gear_pos.0, gear_pos.1 - 1),
            Position(gear_pos.0, gear_pos.1 + 1),
            Position(gear_pos.0 + 1, gear_pos.1 - 1),
            Position(gear_pos.0 + 1, gear_pos.1),
            Position(gear_pos.0 + 1, gear_pos.1 + 1),
        ]
            .into_iter()
            .filter(|p| p.0 < num_rows && p.1 < num_cols)
            .collect::<Vec<Position>>();

        let mut adjacent_nums = Vec::new();
        for (num, num_start_pos) in numbers.iter() {
            let num_end_pos = Position(num_start_pos.0, num_start_pos.1 + num.len() - 1);
            for neighbor in neighbors.iter() {
                if neighbor.0 == num_start_pos.0 && neighbor.1 >= num_start_pos.1 && neighbor.1 <= num_end_pos.1 {
                    adjacent_nums.push(num.parse().unwrap());

                    break;
                }
            }
        }

        if adjacent_nums.len() == 2 {
            gear_ratios.push(adjacent_nums.iter().product())
        }
    }

    gear_ratios.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2_example() {
        let result = part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, "467835");
    }
}