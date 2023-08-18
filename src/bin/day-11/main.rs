#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u32>,
    operation: fn(u32) -> u32,
    divider: u32,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u32,
}

impl Monkey {
    fn new(id: usize, items: Vec<u32>, operation: fn(u32) -> u32, divider: u32, true_monkey: usize, false_monkey: usize) -> Monkey {
        Monkey {
            id,
            items,
            operation,
            divider,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        }
    }

    fn inspect(&mut self) -> (usize, u32) {
        self.inspect_count += 1;
        let item = self.items.remove(0);
        //println!("  Monkey inspects an item with a worry level of {}.", item);
        let mut worry_level = (self.operation)(item);
        //println!("    Worry level is {}.", worry_level);
        worry_level = worry_level / 3;
        //println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
        let receiver = if worry_level % self.divider == 0 {
            //println!("    Current worry level is divisible by {}.", self.divider);
            self.true_monkey
        } else {
            //println!("    Current worry level is divisible by {}.", self.divider);
            self.false_monkey
        };
        //println!("    Item with worry level {} is thrown to monkey {}.", worry_level, receiver);

        (receiver, worry_level)
    }
}

fn main() {
    /*let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));*/

    let mut monkeys = [
        Monkey::new(0, vec![79, 98], |x| x * 19, 23, 2, 3),
        Monkey::new(1, vec![54, 65, 75, 74], |x| x + 6, 19, 2, 0),
        Monkey::new(2, vec![79, 60, 97], |x| x * x, 13, 1, 3),
        Monkey::new(3, vec![74], |x| x + 3, 17, 0, 1)
    ];

    println!("Part 1: {}", part1(&mut monkeys));
}

fn part1(monkeys: &mut [Monkey]) -> u32 {
    let rounds = 20;

    for round in 0..rounds {
        println!("#### Round {}:", round + 1);

        for i in 0..monkeys.len() {
            // println!("Monkey {}:", i);
            let monkey = &mut monkeys[i];
            let mut moved_items = Vec::new();
            while monkey.items.len() > 0 {
                moved_items.push(monkey.inspect());
            }

            for (receiver, worry_level) in moved_items {
                monkeys[receiver].items.push(worry_level);
            }
        }

        for monkey in monkeys.iter() {
            println!("Monkey {}: {}", monkey.id, monkey.items.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
        }
    }

    let mut inspect_counts = monkeys.iter().map(|x| x.inspect_count).collect::<Vec<_>>();
    inspect_counts.sort();
    inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2]
}