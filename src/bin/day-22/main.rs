use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn part1(input: &str) -> u32 {
    let (field, actions) = parse_input(input);

    let r1_start = field[0].iter().position(|&c| c == '.').unwrap();

    // Debug only
    // let mut p_field = field.clone();
    // p_field[0][r1_start] = '>';

    let mut r = 0;
    let mut c = r1_start;
    let mut d = 0;

    let mut positions = vec![(r, c)];

    for action in actions {
        let steps = match action {
            Action::TurnLeft => {
                d = (d + 3) % 4;
                0
            }
            Action::TurnRight => {
                d = (d + 1) % 4;
                0
            }
            Action::Move(steps) => steps
        };

        let (dx, dy) = DIRS[d];

        for _ in 0..steps {
            let mut r_new = r as isize + dy;
            let mut c_new = c as isize + dx;

            if r_new < 0 || r_new >= field.len() as isize || c_new < 0 || c_new >= field[r_new as usize].len() as isize || field[r_new as usize][c_new as usize] == ' ' {
                // Wrap around
                r_new = r as isize;
                c_new = c as isize;
                while r_new >= 0 && c_new >= 0 && r_new < field.len() as isize && c_new < field[r_new as usize].len() as isize && field[r_new as usize][c_new as usize] != ' ' {
                    r_new -= dy;
                    c_new -= dx;
                }
                r_new += dy;
                c_new += dx;
            }

            if field[r_new as usize][c_new as usize] == '#' {
                break;
            }
            r = r_new as usize;
            c = c_new as usize;

            positions.push((r, c));
        }
    }

    // Debug only
    // print_field(&p_field);

    1004 + 1000 * r as u32 + 4 * c as u32 + d as u32
}

fn part2(input: &str) -> u32 {
    0
}

fn print_field(field: &Vec<Vec<char>>) {
    for row in field {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

enum Action {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Action>) {
    let field = input.lines()
        .take_while(|line| !line.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let actions_raw = input.lines().rev().next().unwrap();

    let re = Regex::new(r"L|R|\d+").unwrap();

    let actions = re.find_iter(actions_raw).map(|m| {
        match m.as_str() {
            "L" => Action::TurnLeft,
            "R" => Action::TurnRight,
            x => Action::Move(x.parse().unwrap())
        }
    }).collect();

    (field, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 6032);
        assert_eq!(part1(INPUT), 73346);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
