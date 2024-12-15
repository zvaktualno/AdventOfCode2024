use std::{collections::HashMap, vec};

fn find_all_antennas(lines: &Vec<String>) -> HashMap<u8, Vec<(i32, i32)>> {
    let mut hm: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();

    let map: Vec<Vec<u8>> = lines
                            .iter()
                            .map(|x| x.clone().into_bytes())
                            .collect::<Vec<Vec<u8>>>();

    for lidx in 0..map.len(){
        for cidx in 0..map.len(){
            let c = map[lidx][cidx];
            if c != b'.' {
                let location = (lidx as i32, cidx as i32);
                if !hm.contains_key(&c){
                    hm.insert(c, vec![location]);
                }
                else {
                    hm.get_mut(&c).unwrap().push(location);
                }
            }
        }
    }

    return hm;
}

fn subtract(coor1: (i32, i32), coor2: (i32, i32)) -> (i32, i32){
    return (coor2.0-coor1.0, coor2.1 - coor1.1);
}

fn add(coor1: (i32, i32), coor2: (i32, i32)) -> (i32, i32){
    return (coor1.0+coor2.0, coor1.1+coor2.1);
}

fn get_antinodes(coor1: (i32, i32), coor2: (i32, i32)) -> Vec<(i32, i32)>{

    let p1 = subtract(coor2, coor1);
    let p1 = add(coor2, p1);

    let p2 = subtract(coor1, coor2);
    let p2 = add(coor1, p2);

    return vec![p1, p2];
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let antennas = find_all_antennas(lines);

    for (_, ant) in antennas{
        for i in 0..ant.len()-1{
            for j in i+1 ..ant.len(){
                let an = get_antinodes(ant[i], ant[j]);
                if an[0].0 > 0 && an[0].1 > 0 {
                    dbg!(&an);
                }
            }
        }

    }
    println!("Part one result {}", res);
}