use regex::Regex;

advent_of_code::solution!(2);

struct Cube {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cube {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

struct Cubes {
    current: Cube,
    limit: Cube,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self {
            current: Cube::new(0, 0, 0),
            limit: Cube { red, green, blue },
        }
    }

    fn reset(&mut self) {
        self.current = Cube::new(0, 0, 0);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let mut cubes = Cubes::new(12, 13, 14);
    let mut possible_games: Vec<u32> = Vec::new();

    for cap in re.captures_iter(input) {
        let game_number: u32 = cap[1].parse().unwrap();
        possible_games.push(game_number);

        let re = Regex::new(r"(\d+) ([red|green|blue]+)").unwrap();

        for bag in cap[2].split(";") {
            cubes.reset();
            for c in re.captures_iter(bag) {
                let count: u32 = c[1].parse().unwrap();
                let color: &str = &c[2];

                match color {
                    "red" => cubes.current.red += count,
                    "green" => cubes.current.green += count,
                    "blue" => cubes.current.blue += count,
                    _ => panic!("Unknown color"),
                }
            }

            if cubes.current.red > cubes.limit.red
                || cubes.current.green > cubes.limit.green
                || cubes.current.blue > cubes.limit.blue
            {
                possible_games.retain(|&x| x != game_number);
                break;
            }
        }
    }

    return Some(possible_games.iter().sum());
}

pub fn part_two(_input: &str) -> Option<u32> {
    None // idk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
