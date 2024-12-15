use std::fmt;

advent_of_code::solution!(14);

struct Point {
    x: u64,
    y: u64,
}

struct Velocity {
    x: i64,
    y: i64,
}

struct Robot {
    position: Point,
    velocity: Velocity,
}

impl fmt::Display for Robot {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "p={},{} v={},{}",
            self.position.x, self.position.y, self.velocity.x, self.velocity.y
        )
    }
}

fn parse(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let splitted: Vec<&str> = line.trim().split(' ').collect();
        assert!(splitted.len() == 2);
        let positions: Vec<u64> = splitted[0][2..]
            .split(',')
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        let velocities: Vec<i64> = splitted[1][2..]
            .split(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        let robot = Robot {
            position: Point {
                x: positions[0],
                y: positions[1],
            },
            velocity: Velocity {
                x: velocities[0],
                y: velocities[1],
            },
        };
        robots.push(robot);
    }

    robots
}

pub fn part_one(input: &str) -> Option<u32> {
    //let wide = 101;
    //let tall = 103;
    let wide = 11;
    let tall = 7;
    let mut robots: Vec<Robot> = parse(input);

    for robot in robots.iter_mut() {
        let x_movement = ((robot.velocity.x * 100) % wide + wide) % wide;
        let y_movement = ((robot.velocity.y * 100) % tall + tall) % tall;
        if robot.position.x as i64 + x_movement < wide && robot.position.x as i64 + x_movement >= 0
        {
            robot.position.x = (robot.position.x as i64 + x_movement) as u64;
        } else {
            robot.position.x = (robot.position.x as i64 + (x_movement - wide)) as u64;
        }

        if robot.position.y as i64 + y_movement < tall && robot.position.y as i64 + y_movement >= 0
        {
            robot.position.y = (robot.position.y as i64 + y_movement) as u64;
        } else {
            robot.position.y = (robot.position.y as i64 + (y_movement - tall)) as u64;
        }
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let half_x = ((wide - 1) / 2) as u64;
    let half_y = ((tall - 1) / 2) as u64;
    for robot in robots {
        if robot.position.x < half_x && robot.position.y < half_y {
            q1 += 1;
        } else if robot.position.x > half_x && robot.position.y < half_y {
            q2 += 1;
        } else if robot.position.x < half_x && robot.position.y > half_y {
            q3 += 1;
        } else if robot.position.x > half_x && robot.position.y > half_y {
            q4 += 1;
        }
    }

    Some(q1 * q2 * q3 * q4)
}

fn print_map(robots: Vec<Robot>, rows: usize, cols: usize) {
    let mut map = vec![vec![' '; cols]; rows];
    for robot in robots {
        map[robot.position.y as usize][robot.position.x as usize] = '.';
    }

    for i in 0..map.len() - 1 {
        for j in 0..map[0].len() - 1 {
            print!("{}", map[i][j]);
        }
        println!("");
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let wide = 101;
    let tall = 103;
    let mut robots: Vec<Robot> = parse(input);

    print_map(robots, tall, wide);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
