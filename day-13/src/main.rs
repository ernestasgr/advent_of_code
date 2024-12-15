use std::fs::read_to_string;

struct Machine { ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64 }

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    for chunk in lines.chunks(4) {
        if chunk.len() < 3 {
            continue;
        }

        let a_line = chunk[0];
        let b_line = chunk[1];
        let p_line = chunk[2];

        let ax_ay: Vec<i64> = a_line
            .strip_prefix("Button A: ")
            .unwrap()
            .split(", ")
            .map(|s| s.trim_start_matches(|c| c == 'X' || c == 'Y' || c == '+').parse().unwrap())
            .collect();

        let bx_by: Vec<i64> = b_line
            .strip_prefix("Button B: ")
            .unwrap()
            .split(", ")
            .map(|s| s.trim_start_matches(|c| c == 'X' || c == 'Y' || c == '+').parse().unwrap())
            .collect();

        let px_py: Vec<i64> = p_line
            .strip_prefix("Prize: ")
            .unwrap()
            .split(", ")
            .map(|s| s.trim_start_matches(|c| c == 'X' || c == 'Y' || c == '=').parse().unwrap())
            .collect();

        let adjusted_px = px_py[0] + 10000000000000;
        let adjusted_py = px_py[1] + 10000000000000;

        machines.push(Machine {ax: ax_ay[0], ay: ax_ay[1], bx: bx_by[0], by: bx_by[1], px: adjusted_px, py: adjusted_py});
    }

    machines
}

fn solve_linear_equations(machine: &Machine) -> Option<(i64, i64)> {
    let det = machine.ax * machine.by - machine.ay * machine.bx;

    if det == 0 {
        return None;
    }

    let det_x = machine.px * machine.by - machine.py * machine.bx;
    let det_y = machine.ax * machine.py - machine.ay * machine.px;

    if det_x % det == 0 && det_y % det == 0 {
        let x = det_x / det;
        let y = det_y / det;
        Some((x, y))
    } else {
        None
    }
}
fn main() {
    let input = read_to_string("input.txt").expect("Unable to open file");
    let machines = parse_input(&input);

    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in machines {
        if let Some((a, b)) = solve_linear_equations(&machine) {
            let cost = 3 * a + b;
            total_tokens += cost;
            prizes_won += 1;
        }
    }

    println!("Prizes won: {}", prizes_won);
    println!("Total tokens: {}", total_tokens);
}
