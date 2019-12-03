extern crate elapsed;
use elapsed::measure_time;

fn main() {
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
            part1(parse_input(test_string)),
            (*test_result1, *test_result2)
        );
    }
    let (elapsed, output) = measure_time(|| part1(parse_input(input)));

    println!("Part2 game [{:?}] Elapsed = {}", output, elapsed);
}

fn part1(input: Vec<Vec<(isize, isize)>>) -> (isize, usize) {
    let mut part1_min = isize::max_value();
    let mut part2_min = usize::max_value();
    for n in 0..input[0].len() {
        for m in 0..input[1].len() {
            if input[0][n] == input[1][m] {
                let manhat = calc_manhat(input[0][n]);
                if part1_min > manhat {
                    part1_min = manhat;
                }
                let time = n + m + 2;
                if part2_min > time {
                    part2_min = time;
                }
            }
        }
    }
    (part1_min, part2_min)
}

fn calc_manhat(coord: (isize, isize)) -> isize {
    coord.0.abs() + coord.1.abs()
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
                if let Some(last) = p.last() {
                    p.push((last.0 + dir.0, last.1 + dir.1));
                } else {
                    p.push(dir);
                };
            }
            p
        }));
        pipes
    })
}
