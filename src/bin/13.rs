advent_of_code::solution!(13);

struct Point {
    x: u64,
    y: u64,
}

struct ButtonMovement {
    x: u64,
    y: u64,
}

struct ClawMachine {
    prize: Point,
    button_a: ButtonMovement,
    button_b: ButtonMovement,
}

// Diophantine equation -> X - x1 * z1 ≡ 0 (mod z2)
fn min_tokens_to_win(claw_machine: ClawMachine) -> Option<u64> {
    let mut solutions: Vec<(u64, u64)> = vec![];

    let X = claw_machine.prize.x;
    let z1 = claw_machine.button_a.x;
    let z2 = claw_machine.button_b.x;

    let mut x1 = 0;
    while x1 * z1 <= X {
        let remainder = X - x1 * z1;
        if remainder % z2 == 0 {
            let x2 = remainder / z2;
            // Check if this solution is also valid for y.
            if (claw_machine.button_a.y * x1) + (claw_machine.button_b.y * x2)
                == claw_machine.prize.y
            {
                solutions.push((x1, x2));
            }
        }
        x1 += 1;
    }

    if solutions.len() == 0 {
        None
    } else {
        let mut min_tokens = solutions[0].0 * 3 + solutions[0].1 * 1;
        for i in 1..solutions.len() {
            let required_tokens = solutions[i].0 * 3 + solutions[i].1 * 1;
            if required_tokens < min_tokens {
                min_tokens = required_tokens;
            }
        }

        Some(min_tokens)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut machines: Vec<ClawMachine> = vec![];
    let mut i = 0;
    while i < lines.len() {
        let a_movements: Vec<u64> = lines[i][10..]
            .split(',')
            .map(|v| v.trim()[2..].parse::<u64>().unwrap())
            .collect();
        let button_a = ButtonMovement {
            x: a_movements[0],
            y: a_movements[1],
        };

        let b_movements: Vec<u64> = lines[i + 1][10..]
            .split(',')
            .map(|v| v.trim()[2..].parse::<u64>().unwrap())
            .collect();
        let button_b = ButtonMovement {
            x: b_movements[0],
            y: b_movements[1],
        };

        let points: Vec<u64> = lines[i + 2][6..]
            .split(',')
            .map(|v| v.trim()[2..].parse::<u64>().unwrap())
            .collect();
        let prize = Point {
            x: points[0],
            y: points[1],
        };

        let claw_machine = ClawMachine {
            prize,
            button_a,
            button_b,
        };
        machines.push(claw_machine);
        i += 4;
    }

    let mut res = 0;
    for claw_machine in machines {
        match min_tokens_to_win(claw_machine) {
            Some(t) => res += t,
            None => {}
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
