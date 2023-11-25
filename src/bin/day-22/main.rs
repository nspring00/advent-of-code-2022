use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2::<50>(input));
}


const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

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

fn part2<const SIZE: usize>(input: &str) -> u32 {
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
                // p_field[r][c] = match d {
                //     2 => '<',
                //     0 => '>',
                //     3 => '^',
                //     1 => 'v',
                //     _ => panic!("Invalid direction"),
                // };
                0
            }
            Action::TurnRight => {
                d = (d + 1) % 4;
                // p_field[r][c] = match d {
                //     2 => '<',
                //     0 => '>',
                //     3 => '^',
                //     1 => 'v',
                //     _ => panic!("Invalid direction"),
                // };
                0
            }
            Action::Move(steps) => steps
        };

        for _ in 0..steps {
            let (dx, dy) = DIRS[d];
            let mut r_new = r as isize + dy;
            let mut c_new = c as isize + dx;
            let mut d_new = d;

            if r_new < 0 || r_new >= field.len() as isize || c_new < 0 || c_new >= field[r_new as usize].len() as isize || field[r_new as usize][c_new as usize] == ' ' {
                // Wrap around
                (r_new, c_new, d_new) = part2_try_move::<SIZE>(r, c, d);
            }

            if field[r_new as usize][c_new as usize] == '#' {
                break;
            }
            r = r_new as usize;
            c = c_new as usize;
            d = d_new;

            positions.push((r, c));

            // p_field[r][c] = match d {
            //     2 => '<',
            //     0 => '>',
            //     3 => '^',
            //     1 => 'v',
            //     _ => panic!("Invalid direction"),
            // };
        }
    }

    // Debug only
    // print_field(&p_field);

    1004 + 1000 * r as u32 + 4 * c as u32 + d as u32
}

fn part2_try_move<const SIZE: usize>(r: usize, c: usize, d: usize) -> (isize, isize, usize) {
    let f = get_face::<SIZE>(r, c);
    let r = r as isize;
    let c = c as isize;
    match SIZE {
        4 => {
            match (f, d) {
                (3, UP) => (c - 4, 8, RIGHT),
                (4, RIGHT) => (8, 7 - r + 12, DOWN),
                (5, DOWN) => (11 - r + 7, 11 - c, UP),
                _ => unimplemented!("{}, {}", f, d)
            }
        }
        50 => {
            let r = r % SIZE as isize;
            let c = c % SIZE as isize;
            match (f, d) {
                (1, UP) => (150 + c, 0, RIGHT),
                (1, LEFT) => (149 - r, 0, RIGHT),
                (2, RIGHT) => (149 - r, 99, LEFT),
                (2, DOWN) => (50 + c, 99, LEFT),
                (2, UP) => (199, c, UP),
                (3, RIGHT) => (49, 100 + r, UP),
                (3, LEFT) => (100, r, DOWN),
                (4, LEFT) => (49 - r, 50, RIGHT),
                (4, UP) => (50 + c, 50, RIGHT),
                (5, DOWN) => (150 + c, 49, LEFT),
                (5, RIGHT) => (49 - r, 149, LEFT),
                (6, RIGHT) => (149, 50 + r, UP),
                (6, DOWN) => (0, 100 + c, DOWN),
                (6, LEFT) => (0, 50 + r, DOWN),
                _ => unimplemented!("{}, {}", f, d)
            }
        }
        _ => panic!("Invalid size"),
    }
}

fn get_face<const SIZE: usize>(r: usize, c: usize) -> usize {
    let pos = (r / SIZE, c / SIZE);
    match SIZE {
        4 => match pos {
            (0, 2) => 1,
            (1, 2) => 4,
            (2, 2) => 5,
            (1, 1) => 3,
            (1, 0) => 2,
            (2, 3) => 6,
            _ => panic!("Invalid face"),
        },
        50 => {
            let field: [[usize; 3]; 4] = [[0, 1, 2], [0, 3, 0], [4, 5, 0], [6, 0, 0]];
            field[pos.0][pos.1]
        }
        _ => panic!("Invalid size"),
    }
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
        assert_eq!(part2::<4>(TEST_INPUT_1), 5031);
        assert_eq!(part2::<50>(INPUT), 106392);
    }

    #[test]
    fn test_part2_try_move() {
        assert_eq!(part2_try_move::<50>(0, 60, UP), (160, 0, RIGHT));  // 1 UP
        assert_eq!(part2_try_move::<50>(10, 50, LEFT), (139, 0, RIGHT));  // 1 LEFT
        assert_eq!(part2_try_move::<50>(10, 149, RIGHT), (139, 99, LEFT));  // 2 RIGHT
        assert_eq!(part2_try_move::<50>(49, 110, DOWN), (60, 99, LEFT));  // 2 DOWN
        assert_eq!(part2_try_move::<50>(0, 110, UP), (199, 10, UP));  // 2 UP
        assert_eq!(part2_try_move::<50>(60, 99, RIGHT), (49, 110, UP));  // 3 RIGHT
        assert_eq!(part2_try_move::<50>(60, 50, LEFT), (100, 10, DOWN));  // 3 LEFT
        assert_eq!(part2_try_move::<50>(110, 0, LEFT), (39, 50, RIGHT));  // 4 LEFT
        assert_eq!(part2_try_move::<50>(100, 10, UP), (60, 50, RIGHT));  // 4 UP
        assert_eq!(part2_try_move::<50>(149, 60, DOWN), (160, 49, LEFT));  // 5 DOWN
        assert_eq!(part2_try_move::<50>(110, 99, RIGHT), (39, 149, LEFT));  // 5 RIGHT
        assert_eq!(part2_try_move::<50>(160, 49, RIGHT), (149, 60, UP));  // 6 RIGHT
        assert_eq!(part2_try_move::<50>(199, 10, DOWN), (0, 110, DOWN));  // 6 DOWN
        assert_eq!(part2_try_move::<50>(160, 0, LEFT), (0, 60, DOWN));  // 6 LEFT
    }
}
