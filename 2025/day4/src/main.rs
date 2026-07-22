use helper_scripts::get_puzzle_input;

fn main() {
    let mut data_list : Vec<String> = Vec::new();
    let succ = load_input(&mut data_list);
    // let succ = load_dummy_input(&mut data_list);

    if !succ {
        println!("Exiting...");
        return;
    }
    // part_1(data_list);
    part_2(&mut data_list);
}

fn part_1(data_list : Vec<String>) {
    let mut sum = 0;
    for row in 0..data_list.len() {
        for col in 0..data_list[0].len() {
            let this_cell = data_list[row].chars().nth(col).expect("Could not get current cell");
            if this_cell == '@' {
                if search_neighbors(&data_list, row.try_into().unwrap(), col.try_into().unwrap()) < 4 {sum += 1;}
            }
        }
    }

    println!("Reachable is {}", sum);
    
}

fn part_2(data_list : &mut Vec<String>) {
    let mut sum = 0;
    let mut remove: Vec<(usize,usize)> = Vec::new();
    loop {
        remove.clear();

        for row in 0..data_list.len() {
            for col in 0..data_list[0].len() {
                let this_cell = data_list[row].chars().nth(col).expect("Could not get current cell");
                if this_cell == '@' {
                    if search_neighbors(&data_list, row.try_into().unwrap(), col.try_into().unwrap()) < 4 {
                        sum += 1;
                        remove.push((row,col));
                    }
                }
            }
        }

        for (row, col) in &remove {
            if let Some((byte_idx, c)) = data_list[*row].char_indices().nth(*col) {
                let end_byte_idx = byte_idx + c.len_utf8();
                
                // Mutate the specific character range in-place
                data_list[*row].replace_range(byte_idx..end_byte_idx, ".");
            }
            println!("{}", data_list[*row]);
        }

        if remove.len() == 0 {break;}   
    }

    println!("Reachable with remove is {}", sum); 
}

fn search_neighbors(data_list : &Vec<String>, row: i32, col: i32) -> i32 {
    let mut sum = 0;

    let mut left: usize = 0;
    let right: usize = (col+1).try_into().unwrap();
    let mut upper: usize = 0;
    let lower: usize = (row+1).try_into().unwrap();
    let this: usize = row.try_into().unwrap();

    let has_left = col-1 >= 0;
    if has_left {left = (col-1).try_into().unwrap();}
    let has_right = col+1 < data_list[0].len().try_into().unwrap();
    let has_up = row-1 >= 0;
    if has_up {upper = (row-1).try_into().unwrap();}
    let has_down = row+1 < data_list.len().try_into().unwrap();

    println!("Finding sum at {}{}", row, col);

    // top three
    if has_up {
        let check_row = data_list[upper].clone();
        if has_left &&  get_char(&check_row, left) == '@' {sum += 1;}
        if              get_char(&check_row, col.try_into().unwrap()) == '@' {sum += 1;}
        if has_right && get_char(&check_row, right) == '@' {sum += 1;}
    }
    // println!("Found {} in row above", sum);
    // left and right
    {
        let check_row = data_list[this].clone();
        if has_left &&  get_char(&check_row, left) == '@' {sum += 1;}
        if has_right && get_char(&check_row, right) == '@' {sum += 1;}
    }
    // println!("Found {} in same row", sum);
    // bottom three
    if has_down {
        let check_row = data_list[lower].clone();
        if has_left &&  get_char(&check_row, left) == '@' {sum += 1;}
        if              get_char(&check_row, col.try_into().unwrap()) == '@' {sum += 1;}
        if has_right && get_char(&check_row, right) == '@' {sum += 1;}
    }
    // println!("Found {} in row below", sum);
    sum
}

fn get_char(check_row: &String, idx: usize) -> char {
    check_row.chars().nth(idx).expect("could not get char")
}



// get input
fn load_input(data_list : &mut Vec<String>) -> bool {
    let mut data: String = String::from("");

    if false == get_puzzle_input::fetch_input_string("https://adventofcode.com/2025/day/4/input", &mut data) {
        println!("{}", data);
        return false;
    } else {
        println!("Successful read of data {}", data);
    }

    let rows : Vec<String> = data.split_whitespace().map(String::from).collect();

    data_list.extend(rows);
    
    true
}

fn load_dummy_input(data_list : &mut Vec<String>) -> bool {
    data_list.extend("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".split_whitespace().map(String::from));

    true
}
