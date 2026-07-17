use helper_scripts::get_puzzle_input;

struct Instruction {
    dir : char,
    inc : i32
}

fn main() {
    let mut data_list: Vec<Instruction> = Vec::new();
    let succ = load_input(&mut data_list);

    if !succ {
        println!("Exiting...");
        return;
    }

    let mut count : i32 = 0;
    let mut sum : i32 = 50;

    for instr in data_list {
        if instr.dir == 'R' {
            sum += instr.inc;
        } else {
            sum -= instr.inc;
        }
        sum = sum.rem_euclid(100);

        if sum == 0 {
            count += 1;
        }

        println!("{}{}: sum now: {}, count now: {}", instr.dir, instr.inc, sum, count)
    }
    
    println!("Number 0s is {}", count);
}

// get input
fn load_input(data_list : &mut Vec<Instruction>) -> bool {
    let mut data: String = String::from("");

    if false == get_puzzle_input::fetch_input_string("https://adventofcode.com/2025/day/1/input", &mut data) {
        println!("{}", data);
        return false;
    } else {
        println!("Successful read of data");
    }
     
    for instr in data.split_whitespace() {
        data_list.push(Instruction { dir: instr.chars().nth(0).expect("bad letter"), inc: instr[1..].parse().unwrap() });
    }

    return true;
}