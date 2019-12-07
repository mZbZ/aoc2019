extern crate elapsed;
use elapsed::measure_time;
use std::collections::HashMap;

fn main() {
    println!("AOC 2019 Day6");
    let input = include_str!("../input");

    let mut _test = parse(
        "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN",
    );
    calc_orbit(&mut _test, "COM", 0);
    println!("{:?}", &_test);
    assert_eq!(part2(&_test), 4);

    let (parse_time, mut p_input) = measure_time(|| parse(input));
    let (calc_time, _) = measure_time(|| calc_orbit(&mut p_input, "COM", 0));

    // println!("part1 parsed {:?}", p_input);

    let (solve_time, output) = measure_time(|| part1(&p_input));
    println!("Parse time = {}", parse_time);
    println!("Calc time = {}", calc_time);
    println!("Part 1 answer = {:?}, Solve time = {}", output, solve_time);

    let (solve_time, output) = measure_time(|| part2(&p_input));

    println!("Part 2 answer = {:?}, Solve time = {}", output, solve_time);
}

fn parse(input: &str) -> HashMap<&str, (Vec<&str>, usize)> {
    input
        .lines()
        .map(|x| x.trim().split(')').collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut acc, x| {
            let orbiters = acc.entry(x[0]).or_insert((vec![], 0));
            orbiters.0.push(x[1]);
            acc.entry(x[1]).or_insert((vec![], 0));
            acc
        })
}

fn calc_orbit(
    hash_map: &mut HashMap<&str, (Vec<&str>, usize)>,
    start_key: &str,
    start_depth: usize,
) {
    if let Some(depth) = hash_map.get_mut(start_key) {
        depth.1 += start_depth;
        let new_keys = depth.0.clone();
        let new_depth = depth.1 + 1;
        for key in new_keys {
            calc_orbit(hash_map, key, new_depth);
        }
    }
}

fn part1(input: &HashMap<&str, (Vec<&str>, usize)>) -> usize {
    let mut count = 0;
    for val in input.values() {
        count += val.1;
    }
    count
}

fn part2(input: &HashMap<&str, (Vec<&str>, usize)>) -> usize {
    let mut max_common_depth = usize::max_value();
    for (k, v) in input {}
    max_common_depth
}
