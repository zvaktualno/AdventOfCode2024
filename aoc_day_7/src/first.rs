use std::collections::HashMap;

fn parse_value_pairs(lines: &Vec<String>) -> HashMap<u64, Vec<u64>> {
    let mut data = HashMap::new();
    for line in lines {
        let split_string: Vec<&str>= line.split(": ").collect();
        let result = split_string[0].parse::<u64>().unwrap();
        let values: Vec<u64> = split_string[1].split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

        data.insert(result, values);
    }

    return data;
}

const ADD: u8 = 0;
const MULT: u8 = 1;
const JOIN: u8 = 2;
const POSSIBLE_COMBINATIONS: &'static [u8] = &[ADD, MULT];

fn generate_combinations(members: usize) -> Vec<Vec<u8>>{
    let mut combinations = vec![];
    let num_combinations = (POSSIBLE_COMBINATIONS.len() as u64).pow((members-1) as u32);
    let mut single_combination: Vec<u8> = vec![ADD; members-1];

    for _ in 0..num_combinations{
        combinations.push(single_combination.clone());
        for sc in &mut single_combination {
            *sc += 1;
            if *sc > POSSIBLE_COMBINATIONS[POSSIBLE_COMBINATIONS.len()-1] {
                *sc = ADD;
            }
            else {
                break;
            }
        }
    }
    return combinations;
}

fn check_single_combinations(val: u64, rules: &Vec<u64>) -> u64{

    let combinations = generate_combinations(rules.len());
    for combination in combinations{
        let mut neki = rules[0];
        for cidx in 0..combination.len(){
            if combination[cidx] == ADD {
                neki += rules[cidx+1];
            }
            else if combination[cidx] == MULT {
                neki *= rules[cidx+1];
            }
            else if combination[cidx] == JOIN{
                neki = neki * 10 + rules[cidx+1];
            }
        }
        if neki == val{
            return  val;
        }
    }

    return 0;
}



pub fn solve(lines: &Vec<String>) {
    let mut res: u64 = 0;
    
    let parsed: HashMap<u64, Vec<u64>> = parse_value_pairs(lines);

    for (val, rules) in parsed{
        res += check_single_combinations(val, &rules);
    }

    println!("Part two result {}", res);
}