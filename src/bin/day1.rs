fn main() {
    let input = include_str!("../../inputs/day1.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect::<Vec<i32>>();

        let num =  digits.first().unwrap() * 10 + digits.last().unwrap();

        sum += num;
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in input.lines() {
        let mut digits: Vec<i32> = Vec::new();

        for (index, char) in line.char_indices() {
            if char.is_numeric() {
                digits.push(char.to_digit(10).unwrap() as i32);

                continue;
            }

            let rest = &line[index..];

            for (index, num) in nums.iter().enumerate() {
                if rest.starts_with(num) {
                    digits.push((index + 1) as i32)
                }
            }
        }

        let num =  digits.first().unwrap() * 10 + digits.last().unwrap();

        sum += num;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_example() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281");
    }
}