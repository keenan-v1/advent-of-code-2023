extern crate util;

use std::collections::HashMap;
use regex::Regex;

pub fn day2() {
    let lines = util::load_input("inputs/day2.txt");
    let result = day2_part1(&lines);
    println!("Part 1: The result is {}", result);
    let result = day2_part2(&lines);
    println!("Part 2: The result is {}", result);
}

fn day2_part1(lines: &Vec<String>) -> usize {
    let blocks = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut result = 0;
    for line in lines {
        let game_regex = Regex::new(r"\d+").unwrap();
        let game = game_regex.find_iter(line).map(|x| x.as_str().parse::<usize>().unwrap()).next().unwrap();
        let parts: Vec<&str> = line.split(" ").skip(2).collect();
        let draws = parts.join(" ");
        let mut possible = true;
        for draw in draws.split(";") {
            for group in draw.split(",") {
                let parts = group.trim().split(" ").collect::<Vec<&str>>();
                let number = parts.get(0).unwrap().parse::<i32>().unwrap();
                let color = parts.get(1).unwrap();
                if number.gt(blocks.get(color).unwrap()) {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            result += game;
        }
    }
    return result;
}

fn day2_part2(lines: &Vec<String>) -> usize {
    let mut result: usize = 0;
    for line in lines {
        let mut blocks = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        let mut index = 0;
        let draws = line.split(" ").skip(2).collect::<Vec<&str>>();
        while index < draws.len() {
            let number = draws.get(index).unwrap().parse::<usize>().unwrap();
            index += 1;
            let mut color = *draws.get(index).unwrap();
            if color.ends_with(";") || color.ends_with(",") {
                color = color.trim_matches(color.chars().last().unwrap());
            }
            let count = blocks.get(color).unwrap();
            if number.gt(count) {
                blocks.insert(color, number);
            }
            index += 1;
        }
        let mut total = 1;
        blocks.iter().for_each(|(_, &v)| total *= v);
        result += total;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let lines = util::load_input("../../inputs/examples/day2_part1.txt");
        let result = day2_part1(&lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2() {
        let lines = util::load_input("../../inputs/examples/day2_part2.txt");
        let result = day2_part2(&lines);
        assert_eq!(result, 2286);
    }
}
