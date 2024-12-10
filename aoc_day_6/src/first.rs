const UP: u8 = b'^';
const DOWN: u8 = b'v';
const LEFT: u8 = b'>';
const RIGHT: u8 = b'<';
const invalid_val: (i32, i32) = (9999, 9999);

fn find_guard_xy(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for line in 0..map.len(){
        for col in 0..map.len(){
            if map[line][col] > b'<'{
                return (line, col);
            }
        }    
    }

    assert!(false);
    return (0, 0);
}

fn guard_next_coordinates(dir: u8, line: i32, col: i32) -> (i32, i32){
    let mut new_line = (line, col);
    match dir {
        UP => new_line.0 -= 1,
        DOWN => new_line.0 += 1,
        LEFT => new_line.1 -= 1,
        RIGHT => new_line.1 += 1,
        _ => new_line = invalid_val,
    }
    return new_line;
}

fn turn(dir: u8) -> u8 {
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

fn process_map(map: &mut Vec<Vec<u8>>){

    let (mut line, mut col) = find_guard_xy(map);
    loop {
        let (next_line, next_col) = guard_next_coordinates(map[line][col], line as i32, col as i32);
        if (next_line, next_col) == invalid_val || next_line == map.len() as i32 || next_col == map[0].len() as i32|| next_line == -1 || next_col == -1 {
            break;
        }
        let next_line: usize = next_line as usize;
        let next_col: usize = next_col as usize;

        if map[next_line][next_col] == b'#'{
            map[line][col] = turn(map[line][col]);
        }
        else
        {   
            map[next_line][next_col] = map[line][col];
            map[line][col] = b'X';
            line = next_line;
            col = next_col;
        }
    }
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 1;
    let mut map = lines.clone()
        .iter()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    process_map(&mut map);

    for line in 0..map.len(){
        for col in 0..map.len(){
            if map[line][col] == b'X'{
                res += 1;
            }
        }    
    }
    println!("Part one result {}", res);
}