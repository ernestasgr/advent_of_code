use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read file");

    let (mut reg_a, mut reg_b, mut reg_c, program) = parse_input(&input);
    let output = execute_program(&mut reg_a, &mut reg_b, &mut reg_c, &program);
    print_output(&output);

    let (_, reg_b, reg_c, program) = parse_input(&input);
    reg_a = 0b000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000;
    let mut results = Vec::new();
    get_solution(reg_a, reg_b, reg_c, &program, 15, &mut results);
    println!("{:?}", results);
}

fn get_solution(
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    expected_output: &[u8],
    depth: i32,
    results: &mut Vec<i64>,
) {
    const VALID_MODIFICATIONS: [i64; 8] = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];

    if depth < 0 {
        results.push(reg_a);
        return;
    }

    let chunk_position = depth * 3;
    let chunk_mask = 0b111 << chunk_position;

    for &modification in &VALID_MODIFICATIONS {
        let new_reg_a = (reg_a & !chunk_mask) | (modification << chunk_position);

        let output = execute_program(
            &mut new_reg_a.clone(),
            &mut reg_b.clone(),
            &mut reg_c.clone(),
            expected_output,
        );

        if compare_outputs(expected_output, &output, depth) {
            get_solution(new_reg_a, reg_b, reg_c, expected_output, depth - 1, results);
        }
    }
}

fn compare_outputs(expected: &[u8], actual: &[i64], depth: i32) -> bool {
    if expected.len() != actual.len() {
        return false;
    }
    expected[depth as usize..]
        .iter()
        .enumerate()
        .all(|(i, &p)| p as i64 == actual[i + depth as usize])
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<u8>) {
    let mut lines = input.lines();
    
    let reg_a = parse_register(&mut lines, "Register A");
    let reg_b = parse_register(&mut lines, "Register B");
    let reg_c = parse_register(&mut lines, "Register C");

    let program = lines
        .skip(1)
        .next()
        .and_then(|line| line.strip_prefix("Program: "))
        .expect("Missing Program")
        .split(',')
        .map(|x| x.trim().parse().expect("Invalid program input"))
        .collect();

    (reg_a, reg_b, reg_c, program)
}

fn parse_register<'a, I>(lines: &mut I, name: &str) -> i64
where
    I: Iterator<Item = &'a str>,
{
    lines
        .next()
        .and_then(|line| line.strip_prefix(&format!("{}: ", name)))
        .expect(&format!("Missing {}", name))
        .parse()
        .expect(&format!("Invalid value for {}", name))
}

fn get_combo_value(operand: u8, reg_a: &i64, reg_b: &i64, reg_c: &i64) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => *reg_a,
        5 => *reg_b,
        6 => *reg_c,
        _ => panic!("Invalid combo operand: {}", operand),
    }
}

fn execute_program(
    reg_a: &mut i64,
    reg_b: &mut i64,
    reg_c: &mut i64,
    program: &[u8],
) -> Vec<i64> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        instruction_pointer += 2;

        match opcode {
            0 => *reg_a /= 2_i64.pow(get_combo_value(operand, reg_a, reg_b, reg_c) as u32),
            1 => *reg_b ^= operand as i64,
            2 => *reg_b = get_combo_value(operand, reg_a, reg_b, reg_c) % 8,
            3 => if *reg_a != 0 { instruction_pointer = operand as usize },
            4 => *reg_b ^= *reg_c,
            5 => output.push(get_combo_value(operand, reg_a, reg_b, reg_c) % 8),
            6 => *reg_b = *reg_a / 2_i64.pow(get_combo_value(operand, reg_a, reg_b, reg_c) as u32),
            7 => *reg_c = *reg_a / 2_i64.pow(get_combo_value(operand, reg_a, reg_b, reg_c) as u32),
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    output
}

fn print_output(output: &[i64]) {
    let output_string = output
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    println!("[{}]", output_string);
}