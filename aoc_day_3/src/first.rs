
use regex::Regex;


fn find_valid_commands(line: &str) -> Vec<(i32, i32)>{
    let re_string = r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))";

    let re: Regex = Regex::new(re_string).unwrap();

    let mut mult_pairs = vec![];
    for (_, [_, n1, n2]) in re.captures_iter(line).map(|c| c.extract()) {
        mult_pairs.push((n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap()));
    }
    return mult_pairs;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    for line in lines{
        for (n1, n2) in find_valid_commands(line){
            res += n1 * n2;
        }
    }
    println!("Part one result {}", res);
}