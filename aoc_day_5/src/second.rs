fn put_in_front(update: &mut Vec<i32>, last: usize, first: usize){
    
    let ele = update.remove(first);
    update.insert(last, ele);
}

fn parse_update(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> Vec<i32>{
    let mut res = update.clone();
    let mut current_idx = 0;

    let mut did_something = false;

    loop {
        // Filter out all rules that contain element at current index
        let significant_rules = rules.iter().filter(|x| x.1 == res[current_idx]).collect::<Vec<&(i32,i32)>>();

        // Go over all these rules and sort them if rule is broken
        for (a,_) in significant_rules{
            let idx = res.iter().position(|&x| x == *a);
            if idx.is_some() {
                if idx.unwrap() > current_idx{
                    put_in_front(&mut res, current_idx, idx.unwrap());
                    did_something = true;
                }
            }
        }   
        // Go to next element
        current_idx+=1;

        // Since probably all rules won't be applied first iteration, repeat until full iteration is without a rule fix
        if current_idx == update.len(){
            did_something = false;
            current_idx = 0;
        }

        // If nothing was fixed for the whole iteration, break
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
            let parsed_line = line.split(",")
                                                .map(|x: &str| x.parse::<i32>().unwrap())
                                                .collect::<Vec<i32>>();
            updates.push(parsed_line);
        }
    }

    for update in updates{
        let result = parse_update(&rules, &update);
        if result != update{
            let mid_idx  =update.len() - update.len()/2-update.len()%2;
            res += result[mid_idx];
        }
    }

    println!("Part two result {}", res);
}