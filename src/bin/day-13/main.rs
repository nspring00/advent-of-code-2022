use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Symbol {
    Open,
    Nr(u8),
    Close,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Nr(u8),
    Seq(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nr(n) => write!(f, "{}", n),
            Value::Seq(seq) => {
                write!(f, "[")?;
                for (i, v) in seq.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Value::Nr(ref l), &Value::Nr(ref r)) => l.partial_cmp(r),
            (&Value::Nr(_), &Value::Seq(_)) => Value::Seq(vec![self.clone()]).partial_cmp(other),
            (&Value::Seq(_), &Value::Nr(_)) => self.partial_cmp(&Value::Seq(vec![other.clone()])),
            (&Value::Seq(ref l), &Value::Seq(ref r)) => l.partial_cmp(r),
        }
    }
}

impl Ord for Value {
fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let input_lexed = input.lines()
        .map(|x| x.as_bytes())
        .map(lex)
        .collect::<Vec<_>>();
    let pairs = input_lexed
        .chunks(3)
        .map(|x| (parse(&x[0], &mut 1), parse(&x[1], &mut 1))).collect::<Vec<_>>();

    let mut sum = 0;
    for (i, (l, r)) in pairs.iter().enumerate() {
        let result = l < r;
        if result {
            sum += i as u32 + 1;
        }
    }

    sum
}

fn lex(line: &[u8]) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    let mut i = 0;
    while i < line.len()
    {
        let c = &line[i];
        match c {
            b'[' => symbols.push(Symbol::Open),
            b']' => symbols.push(Symbol::Close),
            b'0'..=b'9' => symbols.push(if line[i + 1] == b'0' { i += 1; Symbol::Nr(10) } else { Symbol::Nr(*c - b'0') }),
            b',' => (),
            _ => panic!("Invalid character '{}'", *c as char),
        }
        i += 1;
    }

    symbols
}

fn parse(symbols: &[Symbol], idx: &mut usize) -> Value {
    let mut values = Vec::new();

    while *idx < symbols.len() - 1 {
        let symbol = symbols[*idx];
        *idx += 1;
        match symbol {
            Symbol::Open => values.push(parse(symbols, idx)),
            Symbol::Nr(n) => values.push(Value::Nr(n)),
            Symbol::Close => return Value::Seq(values),
        }
    }

    Value::Seq(values)
}

fn part2(input: &str) -> u32 {
    let mut values = input.lines()
        .filter(|x| x.len() > 0)
        .map(|x| x.as_bytes())
        .map(lex)
        .map(|x| parse(&x, &mut 1))
        .collect::<Vec<_>>();

    values.push(Value::Seq(vec![Value::Seq(vec![Value::Nr(2)])]));
    values.push(Value::Seq(vec![Value::Seq(vec![Value::Nr(6)])]));

    values.sort();

    // println!("{:}", values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));

    let p1 = values.iter().position(|x| *x == Value::Seq(vec![Value::Seq(vec![Value::Nr(2)])])).unwrap();
    let p2 = values.iter().position(|x| *x == Value::Seq(vec![Value::Seq(vec![Value::Nr(6)])])).unwrap();

    (p1 as u32 + 1) * (p2 as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 13);
        assert_eq!(part1(INPUT), 6076);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 140);
        assert_eq!(part2(INPUT), 24805);
    }
}
