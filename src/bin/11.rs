use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones: Vec<u64> = input
        .split(" ")
        .map(|d| d.trim().parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut i = 0;
        loop {
            if i >= stones.len() {
                break;
            }

            let element = stones[i];
            let element_str = element.to_string();
            if element == 0 {
                stones[i] = 1;
                i += 1;
            } else if element_str.len() % 2 == 0 {
                let s1 = &element_str[0..element_str.len() / 2]
                    .parse::<u64>()
                    .unwrap();
                let s2 = &element_str[element_str.len() / 2..].parse::<u64>().unwrap();
                stones[i] = *s1;
                stones.insert(i + 1, *s2);
                i += 2;
            } else {
                stones[i] = stones[i] * 2024;
                i += 1;
            }
        }
    }

    Some(stones.len())
}

fn calculate_stones(memo: &mut HashMap<(usize, u64), u64>, blinks: usize, v: u64) -> u64 {
    if memo.contains_key(&(blinks, v)) {
        return *memo.get(&(blinks, v)).unwrap();
    }

    if blinks == 0 {
        return 1;
    }

    if v == 0 {
        let res = calculate_stones(memo, blinks - 1, 1);
        memo.insert((blinks, v), res);
        return res;
    }

    let v_str = v.to_string();
    if v_str.len() % 2 == 0 {
        let s1 = &v_str[0..v_str.len() / 2].parse::<u64>().unwrap();
        let s2 = &v_str[v_str.len() / 2..].parse::<u64>().unwrap();
        let res = calculate_stones(memo, blinks - 1, *s1) + calculate_stones(memo, blinks - 1, *s2);
        memo.insert((blinks, v), res);
        return res;
    }

    let res = calculate_stones(memo, blinks - 1, v * 2024);
    memo.insert((blinks, v), res);
    return res;
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split(" ")
        .map(|d| d.trim().parse::<u64>().unwrap())
        .collect();

    let mut memo: HashMap<(usize, u64), u64> = HashMap::new();
    let mut res: u64 = 0;
    for x in stones {
        res += calculate_stones(&mut memo, 75, x);
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
