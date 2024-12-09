use std::usize;

const invalid_val: usize = 999;

fn check_increasing_violation(ints: &Vec<i32>) -> usize {
    for n in 1..ints.len(){
        if ints[n]<=ints[n-1]{
            return n-1;
        }
    }
    return invalid_val;
}

fn check_decreasing_violation(ints: &Vec<i32>) -> usize {
    for n in 1..ints.len(){
        if ints[n]>=ints[n-1]{
            return n-1;
        }
    }
    return invalid_val;
}

fn check_distance_violation(ints: &Vec<i32>) -> usize {

    for n in 1..ints.len(){
        let diff: i32 = i32::abs(ints[n]-ints[n-1]);
        if diff < 1 || diff > 3{
            return n-1;
        }
    }
    return invalid_val;
}

fn check_increasing_violation2(ints: &Vec<i32>) -> usize {
    for n in 1..ints.len(){
        if ints[n]<=ints[n-1]{
            return n;
        }
    }
    return invalid_val;
}

fn check_decreasing_violation2(ints: &Vec<i32>) -> usize {
    for n in 1..ints.len(){
        if ints[n]>=ints[n-1]{
            return n;
        }
    }
    return invalid_val;
}

fn check_distance_violation2(ints: &Vec<i32>) -> usize {

    for n in 1..ints.len(){
        let diff: i32 = i32::abs(ints[n]-ints[n-1]);
        if diff < 1 || diff > 3{
            return n;
        }
    }
    return invalid_val;
}

fn check_strategy(ints: &Vec<i32>, strat: &Vec<fn(&Vec<i32>) -> usize>) -> bool {
    let mut retries: u8 = 0;
    let mut ints_cpy: Vec<i32> = ints.clone();
    let mut res: usize = 0;
    let mut strat_cpy = strat.clone();

    while strat_cpy.len() > 0{
        res = strat_cpy[0](&ints_cpy);

        if res == invalid_val{
            strat_cpy.remove(0);
        }
        else if res != invalid_val && retries == 0 {
            retries += 1;
            ints_cpy.remove(res);
        }
        else {
            return false;
        }
    }
    return res == invalid_val;
}

fn line_ok(ints: &Vec<i32>) -> bool {
    let strategy1: Vec<fn(&Vec<i32>) -> usize> = vec![check_increasing_violation, check_distance_violation];
    let strategy2: Vec<fn(&Vec<i32>) -> usize> = vec![check_decreasing_violation, check_distance_violation];
    let strategy3: Vec<fn(&Vec<i32>) -> usize> = vec![check_increasing_violation2, check_distance_violation];
    let strategy4: Vec<fn(&Vec<i32>) -> usize> = vec![check_decreasing_violation2, check_distance_violation];
    let strategy5: Vec<fn(&Vec<i32>) -> usize> = vec![check_increasing_violation, check_distance_violation2];
    let strategy6: Vec<fn(&Vec<i32>) -> usize> = vec![check_decreasing_violation, check_distance_violation2];
    let strategy7: Vec<fn(&Vec<i32>) -> usize> = vec![check_increasing_violation2, check_distance_violation2];
    let strategy8: Vec<fn(&Vec<i32>) -> usize> = vec![check_decreasing_violation2, check_distance_violation2];

    return check_strategy(ints, &strategy1) ||
           check_strategy(ints, &strategy2) ||
           check_strategy(ints, &strategy3) ||
           check_strategy(ints, &strategy4) ||
           check_strategy(ints, &strategy5) ||
           check_strategy(ints, &strategy6) ||
           check_strategy(ints, &strategy7) ||
           check_strategy(ints, &strategy8);
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;

    for line in lines{
        let vars = line.split(" ").collect::<Vec<&str>>();
        let ints: Vec<i32> = vars.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        //print!("Line {:?}", ints);
        if line_ok(&ints) {
            res += 1;
            //println!("   OK");
        }
        else {
            //println!("   NOK")
        }
    }
    println!("Part two result {}", res);
}

// 604