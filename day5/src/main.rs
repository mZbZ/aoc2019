fn main() {
    let input = include_str!("../input")
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    // let _test_string0 = "1,9,10,3,2,3,11,0,99,30,40,50";
    // let _test_string1 = "1,0,0,0,99";
    // let _test_string2 = "2,3,0,3,99";
    // let _test_string3 = "2,4,4,5,99,0";
    // let _test_string4 = "1,1,1,4,99,5,6,0,99";

    // assert_eq!(part1(_test_string1, 0, 0), 2);
    // assert_eq!(part1(_test_string2, 3, 0), 2);
    // assert_eq!(part1(_test_string3, 4, 4), 2);
    // assert_eq!(part1(_test_string4, 1, 1), 30);
    println!("{:?}", part1(input.clone(), 12, 2));
    // println!("{:?}", part2(input, 19_690_720));
}

fn parse_instruction(instruc: &isize) -> (usize, bool, bool, bool) {
    let opcode = instruc / 100;
    let first_mode = match instruc / 1000 % 10 {
        0 => true,
        1 => false,
        _ => panic!("Should never happen"),
    };
    let second_mode = match instruc / 10_000 % 10 {
        0 => true,
        1 => false,
        _ => panic!("Should never happen"),
    };
    let third_mode = match instruc / 100_000 % 10 {
        0 => true,
        1 => false,
        _ => panic!("Should never happen"),
    };
    (opcode as usize, first_mode, second_mode, third_mode)
}

fn parse_params(instruc: &isize, f_mode: bool, s_mode: bool, t_mode: bool) -> (Option<usize>,Option<usize>,usize) {
    (Some(1),Some(2),3)
}

fn part1(mut prog: Vec<isize>, mut input: isize) -> usize {
    let mut output;
    let mut idx = 0;
    while idx < prog.len(){
        let mut step = 0;
        if prog[idx] == 99 {
            break;
        } else {
            let (op, f_mode, s_mode,t_mode) = parse_instruction(&prog[idx]);
            let (f_idx,s_idx,t_idx) = parse_params(&prog[idx], f_mode, s_mode,t_mode);
            match op {
                1 => prog[t_idx] = prog[f_idx] + prog[s_idx],
                2 => prog[t_idx] = prog[f_idx] * prog[s_idx],
                3 => ,
                4 => ,
                _ => panic!("Should never happen"),
            }
        }
    }
    output
}

// fn part2(input: Vec<usize>, find: usize) -> usize {
//     loop {
//         for x in 0..=99 {
//             for y in 0..=99 {
//                 if part1(input.clone(), x, y) == find {
//                     return (100 * x) + y;
//                 }
//             }
//         }
//     }
// }
