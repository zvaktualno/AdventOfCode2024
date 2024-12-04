use std::collections::{btree_map::OccupiedEntry, HashMap};

pub fn solve(_lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut list1: Vec<i32> = Vec::new();
    let mut ocurrances2: HashMap<i32, i32> = HashMap::new();

    for line in _lines{
        let s = line.split("   ").collect::<Vec<&str>>();
        let i1: i32 = s[0].parse::<i32>().unwrap();
        let i2: i32 = s[1].parse::<i32>().unwrap();

        if !ocurrances2.contains_key(&i2) {
            ocurrances2.insert(i2, 0);
        }

        list1.push(i1);
        *ocurrances2.get_mut(&i2).unwrap() += 1;
    }

    for ele in list1{
        let oc = ocurrances2.get(&ele).unwrap_or(&0);
        res += ele * oc;
    }
    println!("Part two result {}", res);
}
