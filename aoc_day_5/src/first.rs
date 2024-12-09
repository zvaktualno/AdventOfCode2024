fn parse_update(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> Vec<i32>{
    let mut res = update.clone();
    let mut current_idx = 0;

    let mut did_something = false;
    loop {
        let significant_rules = rules.iter().filter(|x| x.1 == res[current_idx]).collect::<Vec<&(i32,i32)>>();
        for (a,b) in significant_rules{
            let idx = res.iter().position(|&x| x == *a);
            if idx.is_some() {
                dbg!(&res, a);
                res.remove(idx.unwrap());
                res.insert(current_idx, *a);
                did_something = true;
                dbg!(&res);
            }
        }   
        current_idx+=1;
        current_idx = current_idx % update.len();

        if !did_something && current_idx == update.len()-1 {
            break;
        }
    }
    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut rules: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

    let mut first_part = true;
    for line in lines {
        if line == ""{
            first_part = false;
            continue;
        }
        
        if first_part {
            let parsed_line = line.split("|").map(|x: &str| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            rules.push((parsed_line[0], parsed_line[1]));
        }
        else {
            let mut parsed_line = line.split(",")
                                                .map(|x: &str| x.parse::<i32>().unwrap())
                                                .collect::<Vec<i32>>();
            updates.push(parsed_line);
        }
    }

    for update in updates{
        parse_update(&rules, &update);
    }

    println!("Part one result {}", res);
}