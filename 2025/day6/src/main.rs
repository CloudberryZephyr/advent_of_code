use helper_scripts::get_puzzle_input;

fn main() {
    let mut opers: Vec<char> = Vec::new();
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let succ = load_input(&mut problems, &mut opers);
    // let succ = load_dummy_input(&mut problems, &mut opers);

    if !succ {
        println!("Exiting...");
        return;
    }

    part_1(opers, &problems);
    // part_2();
}

fn part_1(opers: Vec<char>, problems : &Vec<Vec<u64>>) {
    let mut sum:u64 = 0;
    for idx in 0..problems.len() {
        if opers[idx] == '+' {
            sum += problems[idx].iter().fold(0, |acc: u64, x| acc + x);
        } else {
            sum += problems[idx].iter().fold(1, |acc: u64, x| acc * x);
        }
    }    

    println!("Sum is {}", sum);
}

fn part_2() {

}


// get input
fn load_input(problems : &mut Vec<Vec<u64>>, opers : &mut Vec<char>) -> bool {
    let mut data: String = String::from("");

    if false == get_puzzle_input::fetch_input_string("https://adventofcode.com/2025/day/6/input", &mut data) {
        println!("{}", data);
        return false;
    } else {
        println!("Successful read of data");
    }

    let lines: Vec<&str> = data.split('\n').collect();

    let oper_line: Vec<char> = lines[lines.len()-2]
        .split_whitespace()
        .map(|s: &str| s.chars().nth(0)
        .expect("Could not transform operation symbol to char"))
        .collect(); 

    // println!("opers are {:?}", oper_line);  
    opers.extend(oper_line);


    let numbers_lines = &lines[..lines.len()-2];
    // println!("numbers_lines {:?}", numbers_lines);

    let num_problems: usize = numbers_lines[0].split_whitespace().count();

    // initialize problem vecs in problems
    for _ in 0..num_problems {
        problems.push(Vec::new());
    }

    for line in numbers_lines {
        let line: Vec<u64> = line.split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
        // println!("parsing line: {:?}", line);
        for num in 0..num_problems {
            problems[num].push(line[num]);
        }
    }
    
    true
}

fn load_dummy_input(problems : &mut Vec<Vec<u64>>, ids : &mut Vec<char>) -> bool {
    problems.extend_from_slice(&[vec![123,45,6],vec![328,64,98],vec![51,387,215],vec![64,23,314]]);
    ids.extend(['*','+','*','+']);

    true
}