use helper_scripts::get_puzzle_input;

struct Range {
    first : String,
    last: String
}

fn main() {

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
            let i_str = i.to_string();
            if i_str.len() % 2 == 1 {continue;} // conditions cannot be fulfilled by odd numbers of digits

            let first = i_str[..i_str.len()/2].to_string();
            let second = i_str[i_str.len()/2..].to_string();

            if first != second {continue;}

            sum += i;            
        }
    }

    println!("sum of invalid ids = {}", sum);
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
