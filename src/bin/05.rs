use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut input_rule_part = true;
    let mut res = 0;
    for line in input.lines() {
        if input_rule_part {
            let splitted: Vec<&str> = line.split('|').collect();

            if splitted.len() < 2 {
                input_rule_part = false;
                continue;
            }
            let n1 = splitted[0].parse::<u32>().unwrap();
            let n2 = splitted[1].parse::<u32>().unwrap();
            match rules.get_mut(&n1) {
                Some(r) => r.push(n2),
                None => {
                    rules.insert(n1, vec![n2]);
                }
            }
        } else {
            let splitted: Vec<&str> = line.split(',').collect();
            assert!(splitted.len() % 2 == 1);
            let mut previous: HashMap<u32, bool> = HashMap::new();
            let mut is_valid = true;
            for s in &splitted {
                let n = s.parse::<u32>().unwrap();
                match rules.get(&n) {
                    Some(r) => {
                        for p in r {
                            match previous.get(&p) {
                                Some(_) => {
                                    // Against the rule, row is not valid. Break.
                                    is_valid = false;
                                    break;
                                }
                                None => {}
                            }
                        }
                        previous.insert(n, true);
                    }
                    None => {
                        previous.insert(n, true);
                    }
                }
            }

            if is_valid {
                res += splitted[splitted.len() / 2].parse::<u32>().unwrap();
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
