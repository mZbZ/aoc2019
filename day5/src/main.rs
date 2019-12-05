fn main() {
    let input = parse_day5(include_str!("../input"));

    // let mut input_d2 = include_str!("../../day2/input")
    //     .split(',')
    //     .map(|x| x.parse::<isize>().unwrap())
    //     .collect::<Vec<isize>>();
    // input_d2[1] = 53;
    // input_d2[2] = 79;
    let _t_str0 = parse_day5("3,9,8,9,10,9,4,9,99,-1,8");
    assert_eq!(part_both(_t_str0.clone(), 8), 1);
    assert_eq!(part_both(_t_str0, 7), 0);
    let _t_str1 = parse_day5("3,9,7,9,10,9,4,9,99,-1,8");
    assert_eq!(part_both(_t_str1.clone(), 8), 0);
    assert_eq!(part_both(_t_str1, 7), 1);
    let _t_str2 = parse_day5("3,3,1108,-1,8,3,4,3,99");
    assert_eq!(part_both(_t_str2.clone(), 8), 1);
    assert_eq!(part_both(_t_str2, 7), 0);
    let _t_str3 = parse_day5("3,3,1107,-1,8,3,4,3,99");
    assert_eq!(part_both(_t_str3.clone(), 8), 0);
    assert_eq!(part_both(_t_str3, 7), 1);
    let _t_str4 = parse_day5("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    assert_eq!(part_both(_t_str4.clone(), 0), 0);
    assert_eq!(part_both(_t_str4, 7), 1);
    let _t_str5 = parse_day5("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    assert_eq!(part_both(_t_str5.clone(), 0), 0);
    assert_eq!(part_both(_t_str5, 7), 1);

    let _t_str6 = parse_day5(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
         999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
    );
    assert_eq!(part_both(_t_str6.clone(), 8), 1000);
    assert_eq!(part_both(_t_str6.clone(), 7), 999);
    assert_eq!(part_both(_t_str6, 9), 1001);

    println!("Part1: {:?}", part_both(input.clone(), 1));
    println!("Part2: {:?}", part_both(input, 5));

    // println!("Part2: {:?}", part_both(input, 1));
}

fn parse_day5(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn parse_instruction(instruc: isize) -> (usize, usize, usize) {
    (
        (instruc % 100) as usize,
        (instruc / 100 % 10) as usize,
        (instruc / 1000 % 10) as usize,
    )
}

fn part_both(mut prog: Vec<isize>, input: isize) -> isize {
    let mut output = 0;
    let mut idx = 0;
    while idx < prog.len() {
        if prog[idx] % 100 == 99 {
            break;
        } else {
            let (op, f_mode, s_mode) = parse_instruction(prog[idx]);
            let f_idx = match f_mode {
                0 => prog[idx + 1] as usize,
                1 => idx + 1,
                _ => panic!("Should never happen"),
            };
            let s_idx = match s_mode {
                0 => prog[idx + 2] as usize,
                1 => idx + 2,
                _ => panic!("Should never happen"),
            };

            // dbg!(op, idx, f_idx, prog[f_idx], s_idx, prog[s_idx], t_idx, progt_idx);
            match op {
                1 => {
                    let t_idx = prog[idx + 3] as usize;
                    prog[t_idx] = prog[f_idx] + prog[s_idx];
                    idx += 4;
                }
                2 => {
                    let t_idx = prog[idx + 3] as usize;
                    prog[t_idx] = prog[f_idx] * prog[s_idx];
                    idx += 4;
                }
                3 => {
                    prog[f_idx] = input;
                    idx += 2
                }
                4 => {
                    assert_eq!(output, 0);
                    output = prog[f_idx];

                    idx += 2
                }
                5 => {
                    if prog[f_idx] != 0 {
                        idx = prog[s_idx] as usize;
                    } else {
                        idx += 3;
                    }
                }
                6 => {
                    if prog[f_idx] == 0 {
                        idx = prog[s_idx] as usize;
                    } else {
                        idx += 3;
                    }
                }
                7 => {
                    let t_idx = prog[idx + 3] as usize;
                    if prog[f_idx] < prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    idx += 4;
                }
                8 => {
                    let t_idx = prog[idx + 3] as usize;
                    if prog[f_idx] == prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    idx += 4;
                }
                _ => panic!("Should never happen"),
            }
        }
    }
    // println!("day 1 {:?}", prog[0]);
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
