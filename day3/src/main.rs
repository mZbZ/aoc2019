extern crate elapsed;
use elapsed::measure_time;

fn main() {
    println!("AOC 2019 Day3");
    let input = include_str!("../input");

    let _test_strings = [
        "R8,U5,L5,D3\nU7,R6,D4,L4",
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    ];

    let _test_results = [(6, 30), (159, 610), (135, 410)];
    for (test_string, (test_result1, test_result2)) in
        _test_strings.iter().zip(_test_results.iter())
    {
        assert_eq!(
            part_both(parse_input(test_string)),
            (*test_result1, *test_result2)
        );
    }
    let (parse_time, p_input) = measure_time(|| parse_input(input));
    let (solve_time, output) = measure_time(|| part_both(p_input));
    println!("Parse time = {}", parse_time);
    println!("Both parts {:?}: Solve time = {}", output, solve_time);
}

fn part_both(mut input: Vec<Vec<(isize, isize)>>) -> (isize, usize) {
    // let mut part1_min = ;
    input.pop().unwrap().into_iter().enumerate().fold(
        (isize::max_value(), usize::max_value()),
        |(mut p1_min, mut p2_min), (n, ref x)| {
            input[0].iter().enumerate().for_each(|(m, ref y)| {
                if x == *y {
                    let manhat = x.0.abs() + x.1.abs();
                    if p1_min > manhat {
                        p1_min = manhat;
                    }
                    let time = n + m + 2;
                    if p2_min > time {
                        p2_min = time;
                    }
                }
            });
            (p1_min, p2_min)
        },
    )
}

fn parse_input(input: &str) -> Vec<Vec<(isize, isize)>> {
    input.lines().take(2).fold(vec![], |mut pipes, x| {
        pipes.push(x.split(',').fold(vec![], |mut p, y| {
            let dir = match &y[0..1] {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Should never happen"),
            };
            let count = y[1..].parse::<usize>().unwrap();
            for _ in 0..count {
                let new_dir;
                if let Some(last) = p.last() {
                    new_dir = (last.0 + dir.0, last.1 + dir.1);
                } else {
                    new_dir = dir;
                };
                p.push(new_dir);
            }
            p
        }));
        pipes
    })
}
