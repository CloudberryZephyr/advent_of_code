use helper_scripts::get_puzzle_input;

fn main() {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    let succ = load_input(&mut ranges, &mut ids);
    // let succ = load_dummy_input(&mut ranges, &mut ids);

    if !succ {
        println!("Exiting...");
        return;
    }
    // part_1(ranges, ids);
    part_2(ranges);
}

fn part_1(ranges: Vec<(i64, i64)>, ids: Vec<usize>) {
    let mut sum = 0;
    for id in ids {
        for range in &ranges {
            if (range.0..=range.1).contains(&id.try_into().unwrap()) {
                sum += 1;
                break;
            }
        }
    }

    println!("ids in range is {}", sum)
}

fn part_2(ranges : Vec<(i64,i64)>) {
    let mut condensed : Vec<(i64,i64)> = Vec::new();
    condense(&ranges, &mut condensed);

    // now, all immediate inputs have been condensed. but this may have left some overlapping ranges within condensed, eg with 10-14. 16-20. 12-18
    // so do the same thing to condensed until no changes are made
    let mut new_condensed: Vec<(i64,i64)> = Vec::new();
    while condense(&condensed, &mut new_condensed) {
        condensed = new_condensed;
        new_condensed = Vec::new();
    }

    println!("Final ranges: {:?} ", new_condensed);

    // get count of numbers in remaining range (uppers inclusive) found in new_condensed
    let mut count = 0;
    for range in new_condensed {
        count += range.1-range.0+1;
    }

    println!("Number of ids in ranges is {}", count);
}

fn condense(ranges: &Vec<(i64,i64)>, new : &mut Vec<(i64,i64)>) -> bool {
    let mut condensed: Vec<(i64, i64)> = Vec::new();
    let mut changed = false;
    for range in ranges {
        let mut merged = false;
        for r in &mut condensed {
            // if lower limit in existing condensed range, expand that range to include upper limit
            if (r.0..=r.1).contains(&range.0) {
                println!("Merging {}-{} into {}-{}", r.0, r.1, range.0, range.1);
                r.1 = r.1.max(range.1);
                merged = true;
            }
            // if upper limit in existing condensed range, expand that range to include lower limit
            if (r.0..=r.1).contains(&range.1) {
                println!("Merging {}-{} into {}-{}", r.0, r.1, range.0, range.1);
                r.0 = r.0.min(range.0);
                merged = true;
            }
            // double check if range completely containes r
            if (range.0..=range.1).contains(&r.0) && (range.0..=range.1).contains(&r.1) {
                r.0 = range.0;
                r.1 = range.1;
                merged=true;
            }
            if merged {break;}
        }
        if !merged {
            condensed.push(*range);
            changed = true;
        }
    }

    *new = condensed;
    if ranges == new {changed = false;}
    changed
}

// get input
fn load_input(ranges : &mut Vec<(i64, i64)>, ids : &mut Vec<usize>) -> bool {
    let mut data: String = String::from("");

    if false == get_puzzle_input::fetch_input_string("https://adventofcode.com/2025/day/5/input", &mut data) {
        println!("{}", data);
        return false;
    } else {
        println!("Successful read of data {}", data);
    }

    let (ranges_str, ids_str) = data.split_once("\n\n").expect("Could not parse data");

    for range in ranges_str.split_whitespace() {
        let (low, high) = range.trim().split_once('-').expect("Could not parse ranges");
        let low: i64 = low.parse().unwrap();
        let high: i64 = high.parse().unwrap();
        ranges.push((low, high));
    }

    for id in ids_str.split_whitespace() {
        ids.push(id.trim().parse::<usize>().unwrap());
    }
    
    true
}

fn load_dummy_input(ranges : &mut Vec<(i64, i64)>, ids: &mut Vec<usize>) -> bool {
    ranges.extend([(3,5),(12,18),(9,20)]);
    ids.extend([1,5,8,11,17,32]);

    true
}
