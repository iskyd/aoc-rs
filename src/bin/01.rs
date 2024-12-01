use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut l1: Vec<u32> = vec![];
    let mut l2: Vec<u32> = vec![];
    for line in input.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        assert!(v.len() == 2);
        l1.push(v[0].parse::<u32>().unwrap());
        l2.push(v[1].parse::<u32>().unwrap());
    }
    l1.sort();
    l2.sort();

    assert!(l1.len() == l2.len());

    let res = l1.into_iter().zip(l2).map(|(a, b)| a.abs_diff(b)).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut l1: Vec<u32> = vec![];
    let mut m1 = HashMap::new();
    for line in input.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        assert!(v.len() == 2);
        l1.push(v[0].parse::<u32>().unwrap());
        *m1.entry(v[1].parse::<u32>().unwrap()).or_insert(0) += 1;
    }

    let res = l1.into_iter().map(|a| a * *m1.entry(a).or_insert(0)).sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2756096));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23117829));
    }
}
