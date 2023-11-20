fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


fn parse_dec(n: u64) -> String {
    let mut rem = n;
    let mut ans = Vec::new();
    let i_max = (n as f64).log(5.0) as u32 + 1;
    let mut has_digits = false;

    for i in (0..=i_max).rev() {
        let exp = 5u64.pow(i);
        let digit = rem / exp;

        if digit != 0 {
            has_digits = true;
        } else if !has_digits && i > 0 {
            continue;
        }

        rem -= digit * exp;

        let c = match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("Unknown digit: {}", digit)
        };

        if digit > 2 {
            for j in (0..ans.len()).rev() {
                let c = ans[j];
                if c == '=' {
                    ans[j] = '-';
                    break;
                } else if c == '-' {
                    ans[j] = '0';
                    break;
                } else if c == '0' {
                    ans[j] = '1';
                    break;
                } else if c == '1' {
                    ans[j] = '2';
                    break;
                } else if c == '2' {
                    ans[j] = '=';
                } else {
                    panic!("Unknown digit: {}", c);
                }
            }
        }

        ans.push(c);
    }

    if ans[0] == '=' || ans[0] == '-' {
        ans.insert(0, '1');
    }

    return ans.into_iter().collect::<String>();
}

fn parse_snafu_digit(digit: char) -> i64 {
    match digit {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '=' => -2,
        '-' => -1,
        _ => panic!("Unknown digit: {}", digit)
    }
}

fn parse_snafu(input: &str) -> u64 {
    let l = input.len() as u32;
    let exp: i64 = 5;
    input.chars()
        .enumerate()
        .map(|(i, c)| parse_snafu_digit(c) * exp.pow(l - i as u32 - 1))
        .sum::<i64>() as u64
}

fn part1(input: &str) -> String {
    parse_dec(
        input.lines()
            .map(parse_snafu)
            .sum())
}

fn part2(input: &str) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    macro_rules! parse_snafu_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(parse_snafu(input), expected);
        }
    )*
    }
}
    macro_rules! parse_dec_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (expected, input) = $value;
            assert_eq!(parse_dec(input), expected);
        }
    )*
    }
}

    parse_snafu_tests! {
        test_parse_snafu_0: ("0", 0),
        test_parse_snafu_1: ("1", 1),
        test_parse_snafu_2: ("2", 2),
        test_parse_snafu_3: ("1=", 3),
        test_parse_snafu_4: ("1-", 4),
        test_parse_snafu_5: ("10", 5),
        test_parse_snafu_6: ("11", 6),
        test_parse_snafu_7: ("12", 7),
        test_parse_snafu_8: ("2=", 8),
        test_parse_snafu_9: ("2-", 9),
        test_parse_snafu_10: ("20", 10),
        test_parse_snafu_15: ("1=0", 15),
        test_parse_snafu_20: ("1-0", 20),
        test_parse_snafu_2022: ("1=11-2", 2022),
        test_parse_snafu_12345: ("1-0---0", 12345),
        test_parse_snafu_314159265: ("1121-1110-1=0", 314159265),
    }

    parse_dec_tests! {
        test_parse_dec_0: ("0", 0),
        test_parse_dec_1: ("1", 1),
        test_parse_dec_2: ("2", 2),
        test_parse_dec_3: ("1=", 3),
        test_parse_dec_4: ("1-", 4),
        test_parse_dec_5: ("10", 5),
        test_parse_dec_6: ("11", 6),
        test_parse_dec_7: ("12", 7),
        test_parse_dec_8: ("2=", 8),
        test_parse_dec_9: ("2-", 9),
        test_parse_dec_10: ("20", 10),
        test_parse_dec_15: ("1=0", 15),
        test_parse_dec_20: ("1-0", 20),
        test_parse_dec_2022: ("1=11-2", 2022),
        test_parse_dec_12345: ("1-0---0", 12345),
        test_parse_dec_314159265: ("1121-1110-1=0", 314159265),
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), "2=-1=0");
        assert_eq!(part1(INPUT), "2---1010-0=1220-=010");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
