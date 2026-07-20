use std::i32::MIN;

use helper_scripts::get_puzzle_input;

struct Instruction {
    dir : char,
    inc : i32
}

fn main() {
    let mut data_list: Vec<Instruction> = Vec::new();
    let succ = load_input(&mut data_list);
    // let succ = load_dummy_input(&mut data_list);

    if !succ {
        println!("Exiting...");
        return;
    }

    let mut count : i32 = 0;
    let mut count_crossings : i32 = 0;
    let mut sum : i32 = 50;

    for instr in data_list {
        let old_sum = sum;
        if instr.dir == 'R' {
            sum += instr.inc;

            if instr.inc >= (100 - old_sum) {
                count_crossings += (instr.inc + old_sum) / 100;
            }
        } else {
            sum -= instr.inc;

            if instr.inc >= old_sum && old_sum != 0 {
                count_crossings += (old_sum - instr.inc).abs() / 100 + 1;
            } else if old_sum == 0 && instr.inc >= 100 {
                count_crossings += instr.inc / 100;
            }
        }
        
        sum = sum.rem_euclid(100);
        println!("{}{} old_sum {} sum {} crossings {}", instr.dir, instr.inc, old_sum, sum, count_crossings);

        if sum == 0 {
            count += 1;
        }
        // println!("{}{}: sum now: {}, count now: {}", instr.dir, instr.inc, sum, count)
    }
    
    println!("Number 0s is {}", count);
    println!("Number crossings is {}", count_crossings);
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

fn load_dummy_input(data_list : &mut Vec<Instruction>) -> bool {
    data_list.extend([
        Instruction{dir:'L', inc:68},
        Instruction{dir:'L', inc:30},
        Instruction{dir:'R', inc:48},
        Instruction{dir:'L', inc:5},
        Instruction{dir:'R', inc:60},
        Instruction{dir:'L', inc:55},
        Instruction{dir:'L', inc:1},
        Instruction{dir:'L', inc:99},
        Instruction{dir:'R', inc:14},
        Instruction{dir:'L', inc:82},
        Instruction{dir:'L', inc:300}
    ]);

    true
}