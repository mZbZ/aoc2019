fn main() {
    println!("AOC 2019 Day7");

    let input = parse(include_str!("../input"));

    let _t_str0 = parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(part1(_t_str0), 43210);
    let _t_str1 = parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(part1(_t_str1), 54321);
    let _t_str2 = parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(part1(_t_str2), 65210);

    println!("Part1: {:?}", part1(input.clone()));
    let _t_str3 = parse(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    );
    println!("Test 1 part two");
    assert_eq!(part2(_t_str3), 139_629_729);
    let _t_str4 = parse(
        "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    );
    println!("Test 2 part two");

    assert_eq!(part2(_t_str4), 18216);

    println!("Part2: {:?}", part2(input.clone()));
}

fn parse(input: &str) -> Vec<isize> {
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

fn part1(prog: Vec<isize>) -> isize {
    let mut max_thrust = 0;
    let range = 0..5;
    for a in range.clone() {
        for b in range.clone() {
            for c in range.clone() {
                for d in range.clone() {
                    for e in range.clone() {
                        if a != b
                            && a != c
                            && a != d
                            && a != e
                            && b != c
                            && b != d
                            && b != e
                            && c != d
                            && c != e
                            && d != e
                        {
                            let amp1 = intcode(&mut prog.clone(), &mut vec![0, a], &mut 0).1;
                            let amp2 = intcode(&mut prog.clone(), &mut vec![amp1, b], &mut 0).1;
                            let amp3 = intcode(&mut prog.clone(), &mut vec![amp2, c], &mut 0).1;
                            let amp4 = intcode(&mut prog.clone(), &mut vec![amp3, d], &mut 0).1;
                            let amp5 = intcode(&mut prog.clone(), &mut vec![amp4, e], &mut 0).1;
                            let thrust = amp5;

                            if thrust > max_thrust {
                                max_thrust = thrust
                            }
                        }
                    }
                }
            }
        }
    }
    max_thrust
}

enum Output {
    Halt,
    Continue,
}

fn part2(prog: Vec<isize>) -> isize {
    let mut max_thrust = 0;
    let range = 5..10;
    for a in range.clone() {
        for b in range.clone() {
            for c in range.clone() {
                for d in range.clone() {
                    for e in range.clone() {
                        if a != b
                            && a != c
                            && a != d
                            && a != e
                            && b != c
                            && b != d
                            && b != e
                            && c != d
                            && c != e
                            && d != e
                        {
                            let (mut amp1, mut amp2, mut amp3, mut amp4, mut amp5) =
                                (vec![a], vec![b], vec![c], vec![d], vec![0, e]);
                            let (
                                mut amp1_idx,
                                mut amp2_idx,
                                mut amp3_idx,
                                mut amp4_idx,
                                mut amp5_idx,
                            ) = (0, 0, 0, 0, 0);

                            let (
                                mut amp1_prog,
                                mut amp2_prog,
                                mut amp3_prog,
                                mut amp4_prog,
                                mut amp5_prog,
                            ) = (
                                prog.clone(),
                                prog.clone(),
                                prog.clone(),
                                prog.clone(),
                                prog.clone(),
                            );
                            loop {
                                match intcode(&mut amp1_prog, &mut amp5, &mut amp1_idx) {
                                    (Output::Continue, val) => amp1.insert(0, val),
                                    (Output::Halt, _) => {
                                        break;
                                    }
                                };
                                match intcode(&mut amp2_prog, &mut amp1, &mut amp2_idx) {
                                    (Output::Continue, val) => amp2.insert(0, val),
                                    (Output::Halt, _) => {
                                        break;
                                    }
                                };
                                match intcode(&mut amp3_prog, &mut amp2, &mut amp3_idx) {
                                    (Output::Continue, val) => amp3.insert(0, val),
                                    (Output::Halt, _) => {
                                        break;
                                    }
                                };
                                match intcode(&mut amp4_prog, &mut amp3, &mut amp4_idx) {
                                    (Output::Continue, val) => amp4.insert(0, val),
                                    (Output::Halt, _) => {
                                        break;
                                    }
                                };
                                match intcode(&mut amp5_prog, &mut amp4, &mut amp5_idx) {
                                    (Output::Continue, val) => amp5.insert(0, val),
                                    (Output::Halt, _) => {
                                        break;
                                    }
                                };
                            }
                            let thrust = amp5.pop().unwrap();
                            if thrust > max_thrust {
                                max_thrust = thrust
                            }
                        }
                    }
                }
            }
        }
    }
    max_thrust
}

fn intcode(prog: &mut Vec<isize>, input: &mut Vec<isize>, idx: &mut usize) -> (Output, isize) {
    let output = 0;
    while *idx < prog.len() {
        if prog[*idx] % 100 == 99 {
            return (Output::Halt, output);
        } else {
            let (op, f_mode, s_mode) = parse_instruction(prog[*idx]);
            let f_idx = match f_mode {
                0 => prog[*idx + 1] as usize,
                1 => *idx + 1,
                _ => panic!("Should never happen"),
            };
            let s_idx = match s_mode {
                0 => prog[*idx + 2] as usize,
                1 => *idx + 2,
                _ => panic!("Should never happen"),
            };

            let t_idx = *prog.get(*idx + 3).unwrap_or(&0isize) as usize;
            // println!("Prog {:?}", prog);
            // println!("Index {}", idx);
            match op {
                1 => {
                    prog[t_idx] = prog[f_idx] + prog[s_idx];
                    *idx += 4;
                }
                2 => {
                    prog[t_idx] = prog[f_idx] * prog[s_idx];
                    *idx += 4;
                }
                3 => {
                    prog[f_idx] = input.pop().expect("Not enough input for program!");
                    *idx += 2
                }
                4 => {
                    *idx += 2;
                    return (Output::Continue, prog[f_idx]);
                }
                5 => {
                    if prog[f_idx] != 0 {
                        *idx = prog[s_idx] as usize;
                    } else {
                        *idx += 3;
                    }
                }
                6 => {
                    if prog[f_idx] == 0 {
                        *idx = prog[s_idx] as usize;
                    } else {
                        *idx += 3;
                    }
                }
                7 => {
                    if prog[f_idx] < prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    *idx += 4;
                }
                8 => {
                    if prog[f_idx] == prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    *idx += 4;
                }
                _ => panic!("Should never happen"),
            }
        }
    }
    (Output::Continue, output)
}
