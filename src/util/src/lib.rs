pub fn load_input(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input
        .trim_matches('\n')
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_input() {
        let input = load_input("../../inputs/day1.txt");
        assert_eq!(input.len(), 1000)
    }
}
