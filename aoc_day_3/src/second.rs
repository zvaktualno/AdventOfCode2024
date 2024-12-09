
use regex::Regex;



fn find_valid_commands(line: &str, accepting:  &mut bool) -> Vec<(i32, i32)>{
    let re_string = r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))";

    let re: Regex = Regex::new(re_string).unwrap();

    let mut mults_dos_donts = vec![];
    for (_, [a]) in re.captures_iter(line).map(|c| c.extract()) {
        mults_dos_donts.push(a);
    }


    let mut filtered_mults: Vec<(i32, i32)> = vec![];
    for m in mults_dos_donts{
        if m=="don't()"{
            *accepting = false;
        }
        else if m=="do()" {
            *accepting = true;
        }
        else if *accepting{
            let s: Vec<i32> = m.replace("mul(", "")
                               .replace(")", "")
                               .split(",")
                               .map(|x| x.parse::<i32>().unwrap())
                               .collect();
            filtered_mults.push((s[0], s[1]));
        }
    }

    return filtered_mults;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut accepting: bool = true;
    for line in lines{
        for (n1, n2) in find_valid_commands(line, &mut accepting){
            res += n1*n2;
        }
    }
    println!("Part one result {}", res);
}
