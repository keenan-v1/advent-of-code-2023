extern crate regex;

use regex::Regex;

pub fn day1() {
    let lines = include_str!("../inputs/day1.txt")
        .trim_matches('\n')
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let result = day1_part1(&lines);
    println!("Part 1: The result is {}", result);
    let result = day1_part2(&lines);
    println!("Part 2: The result is {}", result);
}

fn day1_part1(lines: &Vec<String>) -> usize {
    let mut result = 0;
    for line in lines {
        let numbers = Regex::new(r"\d").unwrap();
        let mut numbers = numbers
            .find_iter(line)
            .map(|x| x.as_str().parse::<usize>());
        let first = numbers.next().unwrap_or(Ok(0)).unwrap_or(0);
        let last = numbers.last().unwrap_or(Ok(first)).unwrap_or(0);
        assert!(0 < first && first < 10);
        assert!(0 < last && last < 10);
        result += first * 10 + last;
    }
    return result;
}

fn day1_part2(lines: &Vec<String>) -> usize {
    let mut result = 0;
    let word_to_num = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for line in lines {
        let mut new_line = String::new();
        let mut index = 0;
        while index < line.len() {
            if let Some(ch) = line.chars().nth(index) {
                if ch.is_digit(10) {
                    new_line.push(ch);
                    index += 1;
                } else {
                    let mut found = false;
                    for (word, num) in word_to_num.iter() {
                        if line[index..].starts_with(word) {
                            new_line.push_str(num);
                            index += 1;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        index += 1;
                    }
                }
            }
        }
        let numbers = Regex::new(r"\d").unwrap();
        let mut numbers = numbers
            .find_iter(new_line.as_str())
            .map(|x| x.as_str().parse::<usize>());
        let first = numbers.next().unwrap().unwrap();
        let last = numbers.last().unwrap_or(Ok(first)).unwrap();
        assert!(0 < first && first < 10);
        assert!(0 < last && last < 10);
        result += first * 10 + last;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let lines = include_str!("../inputs/examples/day1_part1.txt")
            .trim_matches('\n')
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = day1_part1(&lines);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let lines = include_str!("../inputs/examples/day1_part2.txt")
            .trim_matches('\n')
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let result = day1_part2(&lines);
        assert_eq!(result, 281 + 18);
    }
}
