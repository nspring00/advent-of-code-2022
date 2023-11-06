use std::collections::HashMap;
use itertools::iproduct;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1_rec(valve: &str, budget: u32, state: u32, flow: u32, answer: &mut HashMap<u32, u32>, system: &ValveSystem2) {
    let new_answer = answer.get(&state).map_or(0, |x| *x).max(flow);
    answer.insert(state, new_answer);

    for next_valve in system.flow_rates.keys() {
        if (system.mask_values[next_valve] & state) > 0 || system.distances[valve][next_valve] + 1 >= budget {
            continue;
        }

        let new_budget = budget - system.distances[valve][next_valve] - 1;
        let next_state = state | system.mask_values[next_valve];
        part1_rec(next_valve, new_budget, next_state, flow + new_budget * system.flow_rates[next_valve], answer, system);
    }
}

fn part1(input: &str) -> u32 {
    // Inspired by https://github.com/juanplopes/advent-of-code-2022/blob/main/day16.py

    // The idea is to store the state of the system as a bitmask, where each bit represents a valve.
    // Overall, we only consider valves with flow rate > 0.
    // For each state, we store the maximum flow rate that can be achieved by opening all active
    // valves in that state.
    // The algorithm searches recursively, decreasing the remaining time upon travelling and opening
    // valves. The stored value on the other hand already takes into account the remaining time,
    // i.e. integrates the flow rate with the remaining time budget.

    let system = parse_input2(input);
    let mut answer = HashMap::new();
    part1_rec("AA", 30, 0, 0, &mut answer, &system);
    *answer.values().max().unwrap()
}

fn part2(input: &str) -> u32 {

    // For part 2, the valves that we and the elephant opened must be distinct
    // From these combination of states, find the one with the maximum flow sum

    let system = parse_input2(input);
    let mut answer = HashMap::new();
    part1_rec("AA", 26, 0, 0, &mut answer, &system);

    iproduct!(&answer, &answer)
        .filter(|((state1, _), (state2, _))| *state1 & *state2 == 0)
        .map(|((_, flow1), (_, flow2))| flow1 + flow2)
        .max()
        .unwrap()
}

struct ValveSystem2<'a> {
    flow_rates: HashMap<&'a str, u32>,
    mask_values: HashMap<&'a str, u32>,
    distances: HashMap<&'a str, HashMap<&'a str, u32>>,
}

fn parse_input2(input: &str) -> ValveSystem2 {
    let re = Regex::new(r"^Valve (.+) has flow rate=(\d+);.*to valves? (.*)$").unwrap();
    let valves_raw = input.lines()
        .map(|x| re.captures(x).map(|caps| {
            let (_, [label, flow_rate, next_valves_str]) = caps.extract();
            let next_valves = next_valves_str.split(", ").collect::<Vec<_>>();
            (label, flow_rate.parse::<u32>().unwrap(), next_valves)
        }).unwrap())
        .collect::<Vec<_>>();

    let tunnels = valves_raw.iter().map(|(label, _, next_valves)| (*label, next_valves.clone())).collect::<HashMap<_, _>>();

    let flow_rates = valves_raw.iter().filter(|(_, flow_rate, _)| *flow_rate != 0).map(|(label, flow_rate, _)| (*label, *flow_rate)).collect::<HashMap<_, _>>();

    let mask_values = flow_rates.iter().enumerate().map(|(i, x)| (*(x.0), 1 << i)).collect::<HashMap<_, _>>();

    // Compute a distance matrix between all valves using Floyd-Warshall
    let mut distances = tunnels.keys().map(|x| (*x, tunnels.keys().map(|y| (*y, 1000u32)).collect::<HashMap<_, _>>())).collect::<HashMap<_, _>>();
    for (x, ys) in tunnels.iter() {
        distances.get_mut(x).unwrap().insert(*x, 0);
        for y in ys {
            distances.get_mut(x).unwrap().insert(*y, 1);
        }
    }
    for k in tunnels.keys() {
        for i in tunnels.keys() {
            for j in tunnels.keys() {
                let new_dist = distances[i][j].min(distances[k][j] + distances[i][k]);
                distances.get_mut(i).unwrap().insert(*j, new_dist);
            }
        }
    }

    ValveSystem2 {
        flow_rates,
        mask_values,
        distances,
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    const TEST_INPUT_1: &str = include_str!("test_input.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_parse_input2() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB\n\
                            Valve BB has flow rate=13; tunnels lead to valves AA, CC\n\
                            Valve CC has flow rate=2; tunnels lead to valves BB";
        let valve_system = parse_input2(input);
        assert_eq!(valve_system.flow_rates, vec![("BB", 13), ("CC", 2)].into_iter().collect::<HashMap<_, _>>());
        assert_eq!(valve_system.distances, vec![("AA", vec![("AA", 0), ("BB", 1), ("CC", 2)].into_iter().collect::<HashMap<_, _>>()), ("BB", vec![("AA", 1), ("BB", 0), ("CC", 1)].into_iter().collect::<HashMap<_, _>>()), ("CC", vec![("AA", 2), ("BB", 1), ("CC", 0)].into_iter().collect::<HashMap<_, _>>())].into_iter().collect::<HashMap<_, _>>());

        // Assert that every key in the bitmask has a unique value and is a power of 2
        assert_eq!(valve_system.mask_values.values().unique().count(), valve_system.mask_values.len());
        assert!(valve_system.mask_values.values().all(|x| x.is_power_of_two()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 1651);
        assert_eq!(part1(INPUT), 1767);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 1707);
        assert_eq!(part2(INPUT), 2528);
    }
}
