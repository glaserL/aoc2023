#[derive(PartialEq)]
enum Strints {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Strints {
    fn as_u32(&self) -> u32 {
        match self {
            Strints::One => 1,
            Strints::Two => 2,
            Strints::Three => 3,
            Strints::Four => 4,
            Strints::Five => 5,
            Strints::Six => 6,
            Strints::Seven => 7,
            Strints::Eight => 8,
            Strints::Nine => 9,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Strints::One => "one",
            Strints::Two => "two",
            Strints::Three => "three",
            Strints::Four => "four",
            Strints::Five => "five",
            Strints::Six => "six",
            Strints::Seven => "seven",
            Strints::Eight => "eight",
            Strints::Nine => "nine",
        }
    }
}
const ALL: [Strints; 9] = [
    Strints::One,
    Strints::Two,
    Strints::Three,
    Strints::Four,
    Strints::Five,
    Strints::Six,
    Strints::Seven,
    Strints::Eight,
    Strints::Nine,
];
fn first_and_last(digits: Vec<u32>) -> u32 {
    let first = digits.first();
    let last = digits.last();

    let solution = match (first, last) {
        (Some(f_digit), Some(l_digit)) => format!("{f_digit}{l_digit}").parse::<u32>().unwrap(),
        _ => 0,
    };
    return solution;
}
fn get_digits_and_numbers(line: String) -> u32 {
    let mut vec: Vec<u32> = Vec::new();
    let mut buffer: String = String::new();
    for char in line.chars() {
        if char.is_numeric() {
            vec.push(char.to_digit(10).unwrap());
            buffer.clear();
            continue;
        }
        buffer.push(char);

        let matched = ALL.into_iter().find(|s| buffer.contains(s.as_str()));
        match matched {
            Some(val) => {
                vec.push(val.as_u32());
                buffer.clear();
                buffer.push(char);
            }
            None => {
                continue;
            }
        }
    }

    return first_and_last(vec);
}

fn get_digits_only(input: &str) -> u32 {
    let digits: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    let first = digits.first();
    let last = digits.last();

    let solution = match (first, last) {
        (Some(f_digit), Some(l_digit)) => format!("{f_digit}{l_digit}").parse::<u32>().unwrap(),
        _ => 0,
    };

    return solution;
}

pub fn solve1(data: String) -> u32 {
    let solution: u32 = data.lines().map(|line| get_digits_only(line)).sum();
    return solution;
}

pub fn solve2(data: String) -> u32 {
    let solution: u32 = data
        .lines()
        .map(|line| get_digits_and_numbers(line.to_string()))
        .sum();
    return solution;
}
#[cfg(test)]
mod tests {
    use super::{solve1, solve2};

    #[test]
    fn puzzle_1() {
        let data = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(solve1(data), 142);
    }
    #[test]
    fn puzzle_2() {
        let data = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(solve2(data), 281);
    }
}
