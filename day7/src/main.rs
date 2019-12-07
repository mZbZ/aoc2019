fn main() {
    println!("AOC 2019 Day7");

    let input = parse(include_str!("../input"));

    // let mut input_d2 = include_str!("../../day2/input")
    //     .split(',')
    //     .map(|x| x.parse::<isize>().unwrap())
    //     .collect::<Vec<isize>>();
    // input_d2[1] = 53;
    // input_d2[2] = 79;
    let _t_str0 = parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(part1(_t_str0), 43210);
    let _t_str1 = parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(part1(_t_str1), 54321);
    let _t_str2 = parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(part1(_t_str2), 65210);

    println!("Part1: {:?}", part1(input.clone()));
    // println!("Part2: {:?}", part_both(input, 5));
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
                            let amp1 = intcode(prog.clone(), vec![0, a]);
                            let amp2 = intcode(prog.clone(), vec![amp1, b]);
                            let amp3 = intcode(prog.clone(), vec![amp2, c]);
                            let amp4 = intcode(prog.clone(), vec![amp3, d]);
                            let amp5 = intcode(prog.clone(), vec![amp4, e]);
                            if amp5 > max_thrust {
                                max_thrust = amp5
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
    Halt(isize),
    Continue(isize),
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
                            let mut stop;
                            let (mut stop, amp1) = match intcode(prog.clone(), vec![0, a]) {
                                Output::Continue(val) => (false, val),
                                Output::Halt(val) => (true, val),
                            };
                            let amp2 = intcode(prog.clone(), vec![amp1, b]);
                            let amp3 = intcode(prog.clone(), vec![amp2, c]);
                            let amp4 = intcode(prog.clone(), vec![amp3, d]);
                            let amp5 = intcode(prog.clone(), vec![amp4, e]);
                            if amp5 > max_thrust {
                                max_thrust = amp5
                            }
                        }
                    }
                }
            }
        }
    }
    max_thrust
}

fn intcode(mut prog: Vec<isize>, mut input: Vec<isize>) -> Output {
    let mut output = 0;
    let mut idx = 0;
    while idx < prog.len() {
        if prog[idx] % 100 == 99 {
            return Output::Halt(output);
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

            let t_idx = *prog.get(idx + 3).unwrap_or(&0isize) as usize;

            match op {
                1 => {
                    prog[t_idx] = prog[f_idx] + prog[s_idx];
                    idx += 4;
                }
                2 => {
                    prog[t_idx] = prog[f_idx] * prog[s_idx];
                    idx += 4;
                }
                3 => {
                    prog[f_idx] = input.pop().expect("Not enough input for program!");
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
                    if prog[f_idx] < prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    idx += 4;
                }
                8 => {
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
    Output::Continue(output);
}
