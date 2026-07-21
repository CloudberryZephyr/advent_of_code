use helper_scripts::get_puzzle_input;

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    let mut data_list : Vec<String> = Vec::new();
    let succ = load_input(&mut data_list);
    // let succ = load_dummy_input(&mut data_list);

    let mut sum : i32 = 0;

    if !succ {
        println!("Exiting...");
        return;
    }

    for num in data_list {
        let digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let offset = 0;
        let (tens_val, offset) = get_max(digits[..digits.len()-1].to_vec(), offset);

        let (ones_val, _offset) = get_max(digits[offset+1..].to_vec(), offset);

        let batteries = tens_val*10 + ones_val;

        println!("Adding {}", batteries);

        sum += i32::try_from(batteries).unwrap();
    }

    println!("Part 1 sum is {}", sum);
}

fn part_2() {
    let mut data_list : Vec<String> = Vec::new();
    let succ = load_input(&mut data_list);
    // let succ = load_dummy_input(&mut data_list);

    let mut sum : i64 = 0;

    if !succ {
        println!("Exiting...");
        return;
    }

    for num in data_list {
        let digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut offset = 0;
        let mut batteries: i64 = 0;

        for digit in (0..12).rev() {
            let (val, new_offset) = get_max(digits[offset..digits.len()-digit].to_vec(), offset);
            offset = new_offset+1;

            batteries += 10i64.pow(digit.try_into().unwrap())*i64::try_from(val).unwrap();
        }
        println!("Adding {}", batteries);

        sum += batteries;
    }

    println!("Part 1 sum is {}", sum);
}

fn get_max(digits: Vec<u32>, offset : usize) -> (i32, usize) {
    let mut max_val = digits[0];
    let mut max_index = 0;
    for (index, &val) in digits.iter().enumerate().skip(1) {
        // println!("Current max is {} at index {}", max_val, max_index+offset);
        if val > max_val {
            max_val = val;
            max_index = index;
        }
    }

    (i32::try_from(max_val).unwrap(), offset+max_index)
}

// get input
fn load_input(data_list : &mut Vec<String>) -> bool {
    let mut data: String = String::from("");

    if false == get_puzzle_input::fetch_input_string("https://adventofcode.com/2025/day/3/input", &mut data) {
        println!("{}", data);
        return false;
    } else {
        println!("Successful read of data");
    }

    let numbers : Vec<String> = data.split_whitespace().map(String::from).collect();

    data_list.extend(numbers);
    
    true
}

fn load_dummy_input(data_list : &mut Vec<String>) -> bool {
    data_list.extend([
        "987654321111111".to_owned(),
        "811111111111119".to_owned(),
        "234234234234278".to_owned(),
        "818181911112111".to_owned(),
    ]);

    true
}
