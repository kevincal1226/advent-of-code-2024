use cached::proc_macro::cached;
use std::collections::HashMap;
use std::collections::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::LazyLock;

static CONTROLLERS: LazyLock<HashMap<char, HashMap<char, &'static str>>> = LazyLock::new(|| {
    HashMap::from([
        (
            'A',
            HashMap::from([
                ('A', "A"),
                ('^', "<A"),
                ('>', "vA"),
                ('v', "<vA"),
                ('<', "v<<A"),
            ]),
        ),
        (
            '^',
            HashMap::from([
                ('A', ">A"),
                ('^', "A"),
                ('>', "v>A"),
                ('v', "vA"),
                ('<', "v<A"),
            ]),
        ),
        (
            '>',
            HashMap::from([
                ('A', "^A"),
                ('^', "<^A"),
                ('>', "A"),
                ('v', "<A"),
                ('<', "<<A"),
            ]),
        ),
        (
            'v',
            HashMap::from([
                ('A', "^>A"),
                ('^', "^A"),
                ('>', ">A"),
                ('v', "A"),
                ('<', "<A"),
            ]),
        ),
        (
            '<',
            HashMap::from([
                ('A', ">>^A"),
                ('^', ">^A"),
                ('>', ">>A"),
                ('v', ">A"),
                ('<', "A"),
            ]),
        ),
    ])
});

static KEYPAD: LazyLock<HashMap<char, HashMap<char, &'static str>>> = LazyLock::new(|| {
    HashMap::from([
        (
            'A',
            HashMap::from([
                ('A', "A"),
                ('0', "<A"),
                ('1', "^<<A"),
                ('2', "<^A"),
                ('3', "^A"),
                ('4', "^^<<A"),
                ('5', "<^^A"),
                ('6', "^^A"),
                ('7', "^^^<<A"),
                ('8', "<^^^A"),
                ('9', "^^^A"),
            ]),
        ),
        (
            '0',
            HashMap::from([
                ('A', ">A"),
                ('0', "A"),
                ('1', "^<A"),
                ('2', "^A"),
                ('3', "^>A"),
                ('4', "^^<A"),
                ('5', "^^A"),
                ('6', "^^>A"),
                ('7', "^^^<A"),
                ('8', "^^^A"),
                ('9', "^^^>A"),
            ]),
        ),
        (
            '1',
            HashMap::from([
                ('A', ">>vA"),
                ('0', ">vA"),
                ('1', "A"),
                ('2', ">A"),
                ('3', ">>A"),
                ('4', "^A"),
                ('5', "^>A"),
                ('6', "^>>A"),
                ('7', "^^A"),
                ('8', "^^>A"),
                ('9', "^^>>A"),
            ]),
        ),
        (
            '2',
            HashMap::from([
                ('A', "v>A"),
                ('0', "vA"),
                ('1', "<A"),
                ('2', "A"),
                ('3', ">A"),
                ('4', "<^A"),
                ('5', "^A"),
                ('6', "^>A"),
                ('7', "<^^A"),
                ('8', "^^A"),
                ('9', "^^>A"),
            ]),
        ),
        (
            '3',
            HashMap::from([
                ('A', "vA"),
                ('0', "<vA"),
                ('1', "<<A"),
                ('2', "<A"),
                ('3', "A"),
                ('4', "<<^A"),
                ('5', "^<A"),
                ('6', "^A"),
                ('7', "<<^^A"),
                ('8', "<^^A"),
                ('9', "^^A"),
            ]),
        ),
        (
            '4',
            HashMap::from([
                ('A', ">>vvA"),
                ('0', ">>vA"),
                ('1', "vA"),
                ('2', "v>A"),
                ('3', "v>>A"),
                ('4', "A"),
                ('5', ">A"),
                ('6', ">>A"),
                ('7', "^A"),
                ('8', "^>A"),
                ('9', "^>>A"),
            ]),
        ),
        (
            '5',
            HashMap::from([
                ('A', "vv>A"),
                ('0', "vvA"),
                ('1', "<vA"),
                ('2', "vA"),
                ('3', "v>A"),
                ('4', "<A"),
                ('5', "A"),
                ('6', ">A"),
                ('7', "<^A"),
                ('8', "^A"),
                ('9', "^>A"),
            ]),
        ),
        (
            '6',
            HashMap::from([
                ('A', "vvA"),
                ('0', "<vvA"),
                ('1', "<<vA"),
                ('2', "<vA"),
                ('3', "vA"),
                ('4', "<<A"),
                ('5', "<A"),
                ('6', "A"),
                ('7', "<<^A"),
                ('8', "<^A"),
                ('9', "^A"),
            ]),
        ),
        (
            '7',
            HashMap::from([
                ('A', ">>vvvA"),
                ('0', ">vvvA"),
                ('1', "vvA"),
                ('2', "vv>A"),
                ('3', "vv>>A"),
                ('4', "vA"),
                ('5', "v>A"),
                ('6', "v>>A"),
                ('7', "A"),
                ('8', ">A"),
                ('9', ">>A"),
            ]),
        ),
        (
            '8',
            HashMap::from([
                ('A', "vvv>A"),
                ('0', "vvvA"),
                ('1', "<vvA"),
                ('2', "vvA"),
                ('3', "vv>A"),
                ('4', "<vA"),
                ('5', "vA"),
                ('6', "v>A"),
                ('7', "<A"),
                ('8', "A"),
                ('9', ">A"),
            ]),
        ),
        (
            '9',
            HashMap::from([
                ('A', "vvvA"),
                ('0', "<vvvA"),
                ('1', "<<vvA"),
                ('2', "<vvA"),
                ('3', "vvA"),
                ('4', "<<vA"),
                ('5', "<vA"),
                ('6', "vA"),
                ('7', "<<A"),
                ('8', "<A"),
                ('9', "A"),
            ]),
        ),
    ])
});

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| "A".to_string() + &l)
        .map(|line| {
            let vec_line: Vec<_> = line.chars().collect();

            let mut keypad_robot: String = vec_line
                .windows(2)
                .flat_map(|window| {
                    KEYPAD
                        .get(&window[0])
                        .unwrap()
                        .get(&window[1])
                        .unwrap()
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>()
                })
                .collect();

            println!("{:?}", keypad_robot);

            keypad_robot.insert(0, 'A');

            let mut first_cont_robot: String = keypad_robot
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .flat_map(|window| {
                    CONTROLLERS
                        .get(&window[0])
                        .unwrap()
                        .get(&window[1])
                        .unwrap()
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>()
                })
                .collect();

            println!("{first_cont_robot:?}");

            first_cont_robot.insert(0, 'A');

            let mut second_cont_robot: String = first_cont_robot
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .flat_map(|window| {
                    CONTROLLERS
                        .get(&window[0])
                        .unwrap()
                        .get(&window[1])
                        .unwrap()
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>()
                })
                .collect();

            println!("{second_cont_robot:?}");

            println!("{}", second_cont_robot.len());

            let parsed_num: usize = line
                .chars()
                .filter(|e| e.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            println!("{parsed_num}\n");

            second_cont_robot.len() * parsed_num
        })
        .sum()
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| "A".to_string() + &l)
        .map(|line| {
            let vec_line: Vec<_> = line.chars().collect();

            let mut keypad_robot: String = vec_line
                .windows(2)
                .flat_map(|window| {
                    KEYPAD
                        .get(&window[0])
                        .unwrap()
                        .get(&window[1])
                        .unwrap()
                        .to_string()
                        .chars()
                        .collect::<Vec<char>>()
                })
                .collect();

            println!("{:?}", keypad_robot);

            keypad_robot.insert(0, 'A');

            let mut first_cont_robot = keypad_robot.clone();
            let mut second_cont_robot = "A".to_string();

            (0..25).for_each(|i| {
                println!("Iteration: {i}");

                let split_by_as: Vec<String> = first_cont_robot
                    .split_inclusive("A")
                    .map(|e| e.to_string())
                    .collect();
                println!("{split_by_as:?}");

                split_by_as.iter().for_each(|s| {
                    second_cont_robot.push_str(&get_path(s.clone()));
                });

                println!("{second_cont_robot}");

                first_cont_robot = second_cont_robot.to_owned();

                second_cont_robot = "A".to_string();
            });

            //println!("{second_cont_robot:?}");

            println!("{}", first_cont_robot.len());

            let parsed_num: usize = line
                .chars()
                .filter(|e| e.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            println!("{parsed_num}\n");

            second_cont_robot.len() * parsed_num
        })
        .sum()
}

#[cached]
fn get_path(path: String) -> String {
    path.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .flat_map(|window| {
            CONTROLLERS
                .get(&window[0])
                .unwrap()
                .get(&window[1])
                .unwrap()
                .to_string()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect()
}
