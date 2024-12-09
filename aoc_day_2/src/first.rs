fn check_increasing(ints: &Vec<i32>) -> bool {

    for n in 1..ints.len(){
        if ints[n]<=ints[n-1]{
            return false;
        }
    }
    return true;
}

fn check_decreasing(ints: &Vec<i32>) -> bool {

    for n in 1..ints.len(){
        if ints[n]>=ints[n-1]{
            return false;
        }
    }
    return true;
}

fn check_distance(ints: &Vec<i32>) -> bool {

    for n in 1..ints.len(){
        let diff: i32 = i32::abs(ints[n]-ints[n-1]);
        if diff < 1 || diff > 3{
            return false;
        }
    }
    return true;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;

    for line in lines{
        let vars = line.split(" ").collect::<Vec<&str>>();
        let ints: Vec<i32> = vars.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        if (check_increasing(&ints) || check_decreasing(&ints)) && check_distance(&ints) {
            res += 1;
        }
    }
    println!("Part one result {}", res);
}