advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut output: Vec<u32> = Vec::new();

    for line in input.lines() {
        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        if digits.len() >= 1 {
            let first = *digits.first().unwrap();
            let mut last = first;

            if let Some(&last_digit) = digits.last() {
                last = last_digit;
            }

            output.push(first * 10 + last);
        }
    }

    return Some(output.iter().sum());
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    // 54627 wrong
    let mut output: Vec<u32> = Vec::new();
    let mut input_parsed: String = String::new();
    let mut i = 0;

    while i < input.len() {
        let mut matched = false;
        for (j, &d) in DIGITS.iter().enumerate() {
            if input[i..].starts_with(d) {
                input_parsed.push_str(&(j + 1).to_string());
                i += d.len();
                matched = true;
                break;
            }
        }
        if !matched {
            input_parsed.push(input.chars().nth(i).unwrap());
            i += 1;
        }
    }

    for line in input_parsed.lines() {
        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        if digits.len() >= 1 {
            let first = *digits.first().unwrap();
            let mut last = first;

            if let Some(&last_digit) = digits.last() {
                last = last_digit;
            }

            output.push(first * 10 + last);
        }
    }

    return Some(output.iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
