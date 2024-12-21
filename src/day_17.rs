use scanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_combo_value(combo: usize, registers: &[usize; 3]) -> usize {
    match combo {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("WTF"),
    }
}

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut registers: [usize; 3] = [0, 0, 0];
    let mut val = 0;
    sscanf!(&reader.next().unwrap().unwrap(), "Register A: {}", val);
    registers[0] = val;
    sscanf!(&reader.next().unwrap().unwrap(), "Register B: {}", val);
    registers[1] = val;
    sscanf!(&reader.next().unwrap().unwrap(), "Register C: {}", val);
    registers[2] = val;
    println!("{:?}", registers);
    let mut program = "A".to_owned();
    sscanf!(&reader.next().unwrap().unwrap(), "Program: {}", program);
    let instructions: Vec<usize> = program
        .chars()
        .filter(|e| e.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();
    let mut pc = 0;
    let mut output = "A".to_owned();
    while pc < instructions.len() {
        match instructions[pc] {
            0 => {
                registers[0] /=
                    2usize.pow(get_combo_value(instructions[pc + 1], &registers) as u32);
                pc += 2;
            }
            1 => {
                registers[1] ^= instructions[pc + 1];
                pc += 2;
            }
            2 => {
                registers[1] = get_combo_value(instructions[pc + 1], &registers) % 8;
                pc += 2;
            }
            3 => {
                if registers[0] == 0 {
                    pc += 2;
                } else {
                    pc = instructions[pc + 1];
                }
            }
            4 => {
                registers[1] ^= registers[2];
                pc += 2;
            }
            5 => {
                output
                    .push_str(&(get_combo_value(instructions[pc + 1], &registers) % 8).to_string());
                output.push(',');
                pc += 2;
            }
            6 => {
                registers[1] = registers[0]
                    / 2usize.pow(get_combo_value(instructions[pc + 1], &registers) as u32);
                pc += 2;
            }
            7 => {
                registers[2] = registers[0]
                    / 2usize.pow(get_combo_value(instructions[pc + 1], &registers) as u32);
                pc += 2;
            }
            _ => {
                panic!("wtf")
            }
        }
    }
    output.pop();
    println!("{}", output);
    0
}

pub fn part_2(_file: String) -> usize {
    let mut registers: [usize; 3] = [0, 0, 0];
    let program = "2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0".to_owned();
    //let program = "0,3,5,4,3,0".to_owned();
    let instructions: Vec<usize> = program
        .chars()
        .filter(|e| e.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();
    let mut reg_a_value = 100000000000000;
    let mut output = "A".to_owned();
    while output != program {
        reg_a_value += 1;
        println!("{}", reg_a_value);
        registers = [reg_a_value, 0, 0];
        output = "A".to_owned();
        let mut pc = 0;
        while pc < instructions.len() {
            match instructions[pc] {
                0 => {
                    registers[0] /=
                        2usize.pow(get_combo_value(instructions[pc + 1], &registers) as u32);
                    pc += 2;
                }
                1 => {
                    registers[1] ^= instructions[pc + 1];
                    pc += 2;
                }
                2 => {
                    registers[1] = get_combo_value(instructions[pc + 1], &registers) % 8;
                    pc += 2;
                }
                3 => {
                    if registers[0] == 0 {
                        pc += 2;
                    } else {
                        pc = instructions[pc + 1];
                    }
                }
                4 => {
                    registers[1] ^= registers[2];
                    pc += 2;
                }
                5 => {
                    output.push_str(
                        &(get_combo_value(instructions[pc + 1], &registers) % 8).to_string(),
                    );
                    output.push(',');
                    if !program.starts_with(&output) {
                        break;
                    }
                    pc += 2;
                }
                6 => {
                    registers[1] = registers[0]
                        / 2usize.pow(get_combo_value(instructions[pc + 1], &registers) as u32);
                    pc += 2;
                }
                7 => {
                    registers[2] = registers[0]
                        / 2usize.pow(get_combo_value(instructions[pc + 1], &registers) as u32);
                    pc += 2;
                }
                _ => {
                    panic!("wtf")
                }
            }
        }
        output.pop();
    }
    reg_a_value
}
