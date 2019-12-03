fn main() {
    let input = include_str!("../input")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let _test_string0 = "1,9,10,3,2,3,11,0,99,30,40,50";
    let _test_string1 = "1,0,0,0,99";
    let _test_string2 = "2,3,0,3,99";
    let _test_string3 = "2,4,4,5,99,0";
    let _test_string4 = "1,1,1,4,99,5,6,0,99";

    // assert_eq!(part1(_test_string1, 0, 0), 2);
    // assert_eq!(part1(_test_string2, 3, 0), 2);
    // assert_eq!(part1(_test_string3, 4, 4), 2);
    // assert_eq!(part1(_test_string4, 1, 1), 30);
    println!("{:?}", part1(input.clone(), 12, 2));
    println!("{:?}", part2(input, 19_690_720));
}

fn part1(mut input: Vec<usize>, pos_1: usize, pos_2: usize) -> usize {
    input[1] = pos_1;
    input[2] = pos_2;

    for i in (0..input.len()).step_by(4) {
        if input[i] == 99 {
            break;
        } else {
            let (change, first, second) = (input[i + 3], input[i + 1], input[i + 2]);
            match input[i] {
                1 => input[change] = input[first] + input[second],
                2 => input[change] = input[first] * input[second],
                _ => panic!("Should never happen"),
            }
        }
    }

    input[0]
}

fn part2(input: Vec<usize>, find: usize) -> usize {
    loop {
        for x in 0..=99 {
            for y in 0..=99 {
                if part1(input.clone(), x, y) == find {
                    return (100 * x) + y;
                }
            }
        }
    }
}
