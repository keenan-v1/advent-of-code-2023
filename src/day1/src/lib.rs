extern crate regex;

use regex::Regex;

pub fn day1(input: String) {
    let result = day1_impl(input);
    println!("The result is {}", result);
}

// We need to write a function for day 1, which takes a string as input and prints the result.
fn day1_impl(input: String) -> usize {
    let lines = input.split('\n');
    let mut result = 0;
    for line in lines {
        // use regex to find all numbers in the string
        let numbers = Regex::new(r"\d+").unwrap();
        // get the first and last number of the string and add them together as the result
        let mut numbers = numbers.find_iter(line)
            .map(|x| x.as_str().parse::<usize>().unwrap());
        result += numbers.next().unwrap_or(0) + numbers.last().unwrap_or(0);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = String::from("a1b2c3d\n4e5f6g7h8i9j\n");
        let result = day1(input);
        assert_eq!(result, 17);
    }
}
