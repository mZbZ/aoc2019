fn main() {
    let input = include_str!("../input");

    let _test_string = "12
14
1969
100756";

    assert_eq!(part1(_test_string), 2 + 2 + 654 + 33583);
    println!("{:?}", part1(input));
    assert_eq!(part2(_test_string), 2 + 2 + 966 + 50346);
    println!("{:?}", part2(input));
}

fn part2(input: &str) -> isize {
    input.lines().fold(0isize, |mut acc, x| {
        let mut mass = x.parse::<isize>().unwrap();
        loop {
            mass = mass_calc(mass);
            if mass > 0 {
                acc += mass;
            } else {
                break;
            }
        }
        acc
    })
}

fn mass_calc(fuel: isize) -> isize {
    (fuel / 3) - 2
}

fn part1(input: &str) -> isize {
    input.lines().fold(0isize, |mut acc, x| {
        acc += mass_calc(x.parse::<isize>().unwrap());
        acc
    })
}
