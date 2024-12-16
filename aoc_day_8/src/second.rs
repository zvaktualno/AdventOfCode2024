use std::{clone, collections::HashMap, vec};

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

fn get_antinodes(coor1: (i32, i32), coor2: (i32, i32), limits: (i32, i32)) -> Vec<(i32, i32)>{

    let mut res = vec![];
    let mut p1 = coor1;
    let p1_vector = subtract(coor2, coor1);

    while valid(limits, p1) {
        res.push(p1);
        p1 = add(p1, p1_vector);
    }


    let mut p2 = coor2;
    let p2_vector = subtract(coor1, coor2);

    while valid(limits, p2) {
        res.push(p2);
        p2 = add(p2, p2_vector);
    }

    return res;
}
fn valid(limits: (i32, i32), coor: (i32, i32)) -> bool{
    return (coor.0 >= 0) && (coor.1 >= 0) && (coor.0 < limits.0) && (coor.1 < limits.1);
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut map = lines.iter()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    println!("Found {}x{}", map.len(), map[0].len());
    let antennas = find_all_antennas(lines);
    let mut antenna_locations: Vec<(i32, i32)> = vec![];
    let limits: (i32, i32) = (map.len() as i32, map[0].len() as i32);

    for (_, ant) in antennas{
        for i in 0..ant.len()-1{
            for j in (i+1)..ant.len(){
                let antinodes: Vec<(i32, i32)> = get_antinodes(ant[i], ant[j], limits);

                for antinode in antinodes{
                    if valid(limits, antinode) && !antenna_locations.contains(&antinode){
                        map[antinode.0 as usize][antinode.1 as usize] = b'#';
                        antenna_locations.push(antinode);
                    }
                }
            }
        }
    }
    res = antenna_locations.len() as i32;

    println!("Part two result {}", res);
}