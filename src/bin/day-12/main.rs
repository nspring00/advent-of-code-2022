use std::collections::VecDeque;

struct Board {
    board: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part_2(input));
}

fn part1(input: &str) -> u32 {
    let mut board = parse_board(input);
    board.board[board.end.1][board.end.0] = b'z';

    let mut queue: VecDeque<(usize, usize, u32)> = VecDeque::new();
    queue.push_back((board.start.0, board.start.1, 0));

    while let Some((x, y, l)) = queue.pop_front() {
        let value = board.board[y][x];

        if value == b'#' {
            continue;
        }
        if x == board.end.0 && y == board.end.1 {
            return l;
        }

        let next_max = match value {
            b'S' => b'z',
            x => x + 1
        };
        board.board[y][x] = b'#';
        if x > 0 && board.board[y][x - 1] <= next_max && board.board[y][x - 1] != b'#' {
            queue.push_back((x - 1, y, l + 1));
        }
        if x < board.width - 1 && board.board[y][x + 1] <= next_max && board.board[y][x + 1] != b'#' {
            queue.push_back((x + 1, y, l + 1));
        }
        if y > 0 && board.board[y - 1][x] <= next_max && board.board[y - 1][x] != b'#' {
            queue.push_back((x, y - 1, l + 1));
        }
        if y < board.height - 1 && board.board[y + 1][x] <= next_max && board.board[y + 1][x] != b'#' {
            queue.push_back((x, y + 1, l + 1));
        }
    }

    panic!("No path found");
}

fn part_2(input: &str) -> u32 {
    let mut board = parse_board(input);
    board.board[board.start.1][board.start.0] = b'a';
    board.board[board.end.1][board.end.0] = b'z';

    let mut queue: VecDeque<(usize, usize, u32)> = VecDeque::new();
    queue.push_back((board.end.0, board.end.1, 0));

    while let Some((x, y, l)) = queue.pop_front() {
        let value = board.board[y][x];

        if value == b'#' {
            continue;
        }
        if value == b'a' {
            return l;
        }

        let next_min = value - 1;
        board.board[y][x] = b'#';
        if x > 0 && board.board[y][x - 1] >= next_min && board.board[y][x - 1] != b'#' {
            queue.push_back((x - 1, y, l + 1));
        }
        if x < board.width - 1 && board.board[y][x + 1] >= next_min && board.board[y][x + 1] != b'#' {
            queue.push_back((x + 1, y, l + 1));
        }
        if y > 0 && board.board[y - 1][x] >= next_min && board.board[y - 1][x] != b'#' {
            queue.push_back((x, y - 1, l + 1));
        }
        if y < board.height - 1 && board.board[y + 1][x] >= next_min && board.board[y + 1][x] != b'#' {
            queue.push_back((x, y + 1, l + 1));
        }
    }

    panic!("No path found");
}

fn parse_board(input: &str) -> Board {
    let board = input.lines().map(|x| x.bytes().collect::<Vec<_>>()).collect::<Vec<_>>();
    let width = board[0].len();
    let height = board.len();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if board[y][x] == b'S' {
                start = (x, y);
            } else if board[y][x] == b'E' {
                end = (x, y);
            }
        }
    }

    Board {
        board,
        width,
        height,
        start,
        end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 31);
        assert_eq!(part1(INPUT), 472);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(TEST_INPUT_1), 29);
        assert_eq!(part_2(INPUT), 465);
    }
}