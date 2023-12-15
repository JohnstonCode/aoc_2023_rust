const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

fn main() {
    let input = include_str!("../../inputs/day2.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    'outer: for line in input.lines() {
        let (game, turns) = line.split_once(": ").unwrap();
        let (_, game_num) = game.split_once(" ").unwrap();
        let game_num: i32 = game_num.parse().unwrap();

        for turn in turns.split("; ") {
            for cubes in turn.split(", ") {
                let (count, color) = cubes.split_once(" ").unwrap();
                let count: i32 = count.parse().unwrap();

                if color == "red" && count > RED_CUBES {
                    continue 'outer;
                }

                if color == "green" && count > GREEN_CUBES {
                    continue 'outer;
                }

                if color == "blue" && count > BLUE_CUBES {
                    continue 'outer;
                }
            }
        }

        sum += game_num;
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let (_, turns) = line.split_once(": ").unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for turn in turns.split("; ") {
            for cubes in turn.split(", ") {
                let (count, color) = cubes.split_once(" ").unwrap();
                let count: i32 = count.parse().unwrap();

                match color {
                    "red" => max_red = max_red.max(count),
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    _ => panic!("No color match")
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_example() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286");
    }
}