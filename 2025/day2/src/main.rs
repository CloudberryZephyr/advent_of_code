use helper_scripts::get_puzzle_input;

struct Range {
    first : String,
    last: String
}

fn main() {
    // part_1();
    part_2();

}

fn part_1() {
    let mut data_list : Vec<Range> = Vec::new();
    // let succ = load_input(&mut data_list);
    let succ = load_dummy_input(&mut data_list);

    let mut sum : i64 = 0;

    if !succ {
        println!("Exiting...");
        return;
    }

    for range in data_list {
        let lower : i64 = range.first.parse().unwrap();
        let upper : i64 = range.last.parse().unwrap();
        for i in lower..=upper {
            let i_str = i.to_string();
            if i_str.len() % 2 == 1 {continue;} // conditions cannot be fulfilled by odd numbers of digits

            let first = i_str[..i_str.len()/2].to_string();
            let second = i_str[i_str.len()/2..].to_string();

            if first != second {continue;}

            sum += i;            
        }
    }

    println!("sum of invalid ids pt 1 = {}", sum);
}


fn part_2() {
    let mut data_list : Vec<Range> = Vec::new();
    let succ = load_input(&mut data_list);
    // let succ = load_dummy_input(&mut data_list);

    let mut sum : i64 = 0;

    if !succ {
        println!("Exiting...");
        return;
    }

    for range in data_list {
        let lower : i64 = range.first.parse().unwrap();
        let upper : i64 = range.last.parse().unwrap();
        for i in lower..=upper {
            if check_num(i) {
                println!("Adding {}", i);
                sum += i;
            }
        }
    }

    println!("sum of invalid ids pt 2 = {}", sum);
}


fn check_num(num: i64) -> bool {
    for j in 1..=(num.to_string().len()/2) { // check all possible interval lengths
        // check if there's a pattern in this interval
        if num.to_string().len() % j == 0 && check_size(num, j) {// if so, add to sum, break because we don't need to check any more intervals
            return true; // we found an interval with a repetition
        }
    }
    false // we never found a repeating interval for this number
}

fn check_size(num: i64, len: usize) -> bool {
    // println!("Checking {} with length {}", num, len);
    // get slice for matching
    let match_slice: String = num.to_string()[..len].to_string();
    // split i into  len/j slices
    let slices = split_num(num, len);
    for slice in slices {
        // println!("{} vs {}", match_slice, slice);
        if slice != match_slice {return false;} // no pattern at this level, check the next level
    }
    true
}

fn split_num(num : i64, slice_len : usize) -> Vec<String> {
    let mut slices : Vec<String> = Vec::new();
    let mut num : String = num.to_string();
    while num.len() >= slice_len {
        let slice: String = num[..slice_len].to_string();
        slices.push(slice);
        num = num[slice_len..].to_string();
    }

    slices
}


// get input
fn load_input(data_list : &mut Vec<Range>) -> bool {
    let mut data: String = String::from("");

    if false == get_puzzle_input::fetch_input_string("https://adventofcode.com/2025/day/2/input", &mut data) {
        println!("{}", data);
        return false;
    } else {
        println!("Successful read of data {}", data);
    }

    let ranges: Vec<&str> = data.trim().split(',').collect();
    for range in ranges {
        let limits: Vec<&str> = range.split('-').collect();
        let first  = limits[0];
        let last  = limits[1];
        println!("{} {}", first, last);
        data_list.push(Range{first:first.to_owned(), last:last.to_owned()});
    }
    
    return true;
}

fn load_dummy_input(data_list : &mut Vec<Range>) -> bool {
    data_list.extend([
        Range{first:"11".to_owned(),last:"22".to_owned()},
        Range{first:"95".to_owned(),last:"115".to_owned()},
        Range{first:"998".to_owned(),last:"1012".to_owned()},
        Range{first:"1188511880".to_owned(),last:"1188511890".to_owned()},
        Range{first:"222220".to_owned(),last:"222224".to_owned()},
        Range{first:"1698522".to_owned(),last:"1698528".to_owned()},
        Range{first:"446443".to_owned(),last:"446449".to_owned()},
        Range{first:"38593856".to_owned(),last:"38593862".to_owned()},
        Range{first:"565653".to_owned(),last:"565659".to_owned()},
        Range{first:"824824821".to_owned(),last:"824824827".to_owned()},
        Range{first:"2121212118".to_owned(),last:"2121212124".to_owned()}
    ]);

    true
}
