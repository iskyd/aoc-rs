advent_of_code::solution!(7);

enum Operators {
    Add,
    Multiply,
}

fn generate_combinations(operators: Vec<&Operators>, n: usize) -> Vec<Vec<&Operators>> {
    // Start with a vector containing an empty combination
    let mut combinations = vec![vec![]];

    // Repeat n times to build combinations iteratively
    for _ in 0..n {
        combinations = combinations
            .into_iter()
            .flat_map(|prefix| {
                operators.iter().map(move |op| {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(op.clone());
                    new_prefix
                })
            })
            .collect();
    }

    combinations
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut res: u64 = 0;

    for line in input.lines() {
        let splitted: Vec<&str> = line.split(":").collect();
        assert!(splitted.len() == 2);
        let equation_result = splitted[0].parse::<u64>().unwrap();
        let values: Vec<u64> = splitted[1]
            .trim()
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        let combinations = generate_combinations(
            vec![&Operators::Add, &Operators::Multiply],
            values.len() - 1,
        );

        for c in combinations {
            let mut local_res = values[0];
            assert!(c.len() == values.len() - 1);
            for i in 0..c.len() {
                match c[i] {
                    Operators::Add => {
                        local_res += values[i + 1];
                    }
                    Operators::Multiply => {
                        local_res *= values[i + 1];
                    }
                }
            }

            if local_res == equation_result {
                res += equation_result;
                break;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
