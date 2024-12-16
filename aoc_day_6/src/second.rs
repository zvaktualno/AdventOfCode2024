use std::collections::HashSet;


const UP: u8 = 1 << 0;
const DOWN: u8 = 1 << 1;
const LEFT: u8 = 1 << 2;
const RIGHT: u8 = 1 << 3;
const OBSTACLE: u8 = 1 << 4;

const PRETTY_PRINT: bool = true;
const INVALID_VAL: (i32, i32) = (-1, -1);

fn guard_find(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for line in 0..map.len(){
        for col in 0..map[0].len(){
            if map[line][col] <= RIGHT && map[line][col] >= UP{
                return (line, col);
            }
        }
    }

    assert!(false, "Couldn't find the guard");
    return (0, 0);
}

fn guard_next_coordinates(dir: u8, coor: (usize, usize)) -> (i32, i32){
    let mut new_coor = (coor.0 as i32, coor.1 as i32);
    match dir & (UP|DOWN|LEFT|RIGHT) {
        UP => new_coor.0 -= 1,
        DOWN => new_coor.0 += 1,
        LEFT => new_coor.1 -= 1,
        RIGHT => new_coor.1 += 1,
        _ => new_coor = INVALID_VAL,
    }
    return new_coor;
}

fn next_dir(dir: u8) -> u8 {
    let mut new_dir: u8 = 0;
    match dir {
        UP => new_dir = RIGHT,
        DOWN => new_dir = LEFT,
        LEFT => new_dir = UP,
        RIGHT => new_dir = DOWN,
        _ => assert!(false),
    }
    return  new_dir;
}

fn valid_coor(map: &Vec<Vec<u8>>, coor: (i32, i32)) -> bool{
    return coor.0 >= 0 && coor.0<map.len() as i32 && coor.1 >= 0 && coor.1 < map[0].len() as i32;
}

fn already_visited(moving_dir_map: &Vec<Vec<u8>>, dir: u8, l: usize, c: usize) -> bool {
    return (moving_dir_map[l][c] & dir) != 0;
}

fn print_dir_map(moving_dir_map: &Vec<Vec<u8>>) {
    for l in moving_dir_map{
        for c in l{
            if !PRETTY_PRINT {
                print!("| {c} |");
            }
            else {

                if (c & (DOWN | UP)) != 0 && (c & (LEFT | RIGHT) != 0){
                    print!("+");
                }
                else if (c & (DOWN | UP)) != 0 {
                    print!("|");
                }
                else if (c & (LEFT | RIGHT)) != 0 {
                    print!("-");
                }
                else if (c & OBSTACLE) != 0 {
                    print!("O");
                }
                else {
                    print!(" ");
                }
                }
        }
        println!("");
    }
    println!("");
}

fn check_looping(map: &mut Vec<Vec<u8>>, dirmap: &mut Vec<Vec<u8>>, c: (usize, usize)) -> bool{
    let mut coor = c;
    loop {
        let next = guard_next_coordinates(map[coor.0][coor.1], coor);
        if !valid_coor(&map, next) {
            dirmap[coor.0][coor.1] |= map[coor.0][coor.1];
            return false;
        }
        let next_line: usize = next.0 as usize;
        let next_col: usize = next.1 as usize;

        if map[next_line][next_col] == OBSTACLE{
            dirmap[next_line][next_col] = OBSTACLE;
            map[coor.0][coor.1] = next_dir(map[coor.0][coor.1]);
        }
        else
        {
            if already_visited(&dirmap, map[coor.0][coor.1], next_line, next_col)
            {
                return true;
            }
            dirmap[coor.0][coor.1] |= map[coor.0][coor.1];
            map[next_line][next_col] = map[coor.0][coor.1];
            coor = (next_line, next_col);
        }
    }
}

fn check_candidate(m: &Vec<Vec<u8>>, dm: &Vec<Vec<u8>>, coor: (usize, usize)) -> (bool, usize, usize) {
    let next: (i32, i32) = guard_next_coordinates(m[coor.0][coor.1], coor);

    if !valid_coor(m, next){
        return (false, 0, 0);
    }

    let next = (next.0 as usize, next.1 as usize);
    let mut map: Vec<Vec<u8>> = m.clone();
    let mut dirmap: Vec<Vec<u8>> = dm.clone();
    if dirmap[next.0][next.1] != 0
    {
        return (false, 0, 0);
    }
    map[next.0][next.1] = OBSTACLE;

    if check_looping(&mut map, &mut dirmap, coor){
        return (true, next.0, next.1);
    }
    return (false, 0, 0);
}

fn process_map(m: &Vec<Vec<u8>>) -> Vec<(usize, usize)>{
    let mut res: Vec<(usize, usize)> = vec![];

    let mut map = m.clone();
    let mut dirmap: Vec<Vec<u8>> = vec![vec![0; map[0].len()]; map.len()];
    let mut starting= true;
    let mut coor = guard_find(&map);
    println!("Found guard at {} and {}", coor.0, coor.1);

    loop {
        let next = guard_next_coordinates(map[coor.0][coor.1], coor);
        if !valid_coor(&map, next) {
            dirmap[coor.0][coor.1] |= map[coor.0][coor.1];
            println!("Broke out");
            break;
        }
        let next_line: usize = next.0 as usize;
        let next_col: usize = next.1 as usize;

        if map[next_line][next_col] == OBSTACLE{
            dirmap[next_line][next_col] = OBSTACLE;
            map[coor.0][coor.1] = next_dir(map[coor.0][coor.1]);
        }
        else
        {
            if starting {

                let (found, l,c) = check_candidate(&map, &dirmap, coor);
                if found{
                    res.push((l,c));
                }
                starting = true;
            }
            dirmap[coor.0][coor.1] |= map[coor.0][coor.1];
            map[next_line][next_col] = map[coor.0][coor.1];
            coor = (next_line, next_col);
        }
    }
    print_dir_map(&dirmap);
    return res;

}

fn remove_duplicates(vec: &mut Vec<(usize, usize)>) {
    let mut seen = HashSet::new();
    vec.retain(|item| seen.insert(item.clone()));
}

pub fn solve(lines: &Vec<String>){



    let mut map: Vec<Vec<u8>> = lines.clone()
        .iter_mut()
        .map(|x: &mut String| x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    for line in 0..map.len(){
        for col in 0..map[0].len(){
            if map[line][col] == b'^'{
                map[line][col] = UP;
            }
            else if map[line][col] == b'#'{
                map[line][col] = OBSTACLE;
            }
            else {
                map[line][col] = 0;
            }

        }
    }

    let guard_coor = guard_find(&map);
    let mut res: Vec<(usize, usize)> = process_map(&map);
    println!("Found {} points", res.len());
    remove_duplicates(&mut res);
    println!("{} of them are unique", res.len());

    for idx in 0..res.len(){
        if res[idx] == guard_coor{
            res.remove(res.iter().position(|x| *x==res[idx]).unwrap());
            println!("Same position");
            break;
        }
    }
    let res = res.len();

    println!("Part two result {}", res);
}

