use std::array::TryFromSliceError;

fn vector_compare(v1: &[u8], v2: &[u8]) -> bool{
    if v1.len() != v2.len(){
        return false
    }
    return v1.iter().zip(v2.iter()).filter(|&(a, b)| a == b).count() == v1.len();
}

fn find_horizontally(map: &Vec<Vec<u8>>) -> i32 {
    // Horizontal search is done by moving a window over each line and scanning for the word and its reverse value.
    let mut result = 0;
    for line in map{
        let word1 = String::from("XMAS");
        let word2 = word1.chars().rev().collect::<String>();
        let res1 = line.windows(word1.len()).filter(|sw| vector_compare(sw, word1.as_bytes())).count();
        let res2 = line.windows(word1.len()).filter(|sw| vector_compare(sw, word2.as_bytes())).count();
        result += res1 + res2;
    }
    return result as i32;
}

fn find_vertically(map: &Vec<Vec<u8>>) -> i32 {
    // Vertical search is done by rotating the 2D matrix and searching horizontally

    let mut flipped_map: Vec<Vec<u8>> = vec![];
    for cidx in 0..map[0].len(){
        let mut new_line: Vec<u8> = vec![];
        for lidx in 0..map.len() {
            new_line.push(map[lidx][cidx]);
        }
        flipped_map.push(new_line);
    }

    return find_horizontally(&flipped_map);
}

fn find_diagonally_minus45(map: &Vec<Vec<u8>>) -> i32 {
    // Vertical search is done by rotating the 2D matrix and searching horizontally

    assert!(map.len() == map[0].len(), "The input matrix must be a square.");
    let mut flipped_map: Vec<Vec<u8>> = vec![];


    for i in 0..map.len(){
        let mut new_line: Vec<u8> = vec![];

        let mut line = 0;
        let mut col: usize = map[0].len() - 1 - i;

        while line < map[0].len() && col < map[0].len() {
            new_line.push(map[line][col]);
            line +=  1;
            col +=  1;
        }
        flipped_map.push(new_line);
    }

    for i in 0..map.len(){
        let mut new_line: Vec<u8> = vec![];

        let mut line: usize = i + 1;
        let mut col: usize = 0;

        while line < map[0].len() && col < map[0].len() {
            new_line.push(map[line][col]);
            line +=  1;
            col +=  1;
        }
        flipped_map.push(new_line);
    }


    return find_horizontally(&flipped_map);
}

fn find_diagonally_plus45(map: &Vec<Vec<u8>>) -> i32 {
    // Vertical search is done by rotating the 2D matrix and searching horizontally

    assert!(map.len() == map[0].len(), "The input matrix must be a square.");
    let mut flipped_map: Vec<Vec<u8>> = vec![];


    for i in 0..map.len(){
        let mut new_line: Vec<u8> = vec![];

        let mut line = 0;
        let mut col: usize = i;

        loop {
            new_line.push(map[line][col]);
            if col == 0 {
                break;
            }
            line +=  1;
            col -=  1;
        }
        flipped_map.push(new_line);
    }

    for i in 0..map.len()-1{
        let mut new_line: Vec<u8> = vec![];

        let mut line: usize = i + 1;
        let mut col: usize = map.len() -1 ;


        loop {
            new_line.push(map[line][col]);
            if line == map.len() - 1 {
                break;
            }
            line +=  1;
            col -=  1;
        }
        flipped_map.push(new_line);
    }


    return find_horizontally(&flipped_map);
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut charmap: Vec<Vec<u8>> = vec![];

    for line in lines{
        let l = line.as_bytes();
        let l = Vec::from(l);
        charmap.push(l);
    }

    res += find_horizontally(&charmap);
    res += find_vertically(&charmap);

    //let testmap: Vec<Vec<u8>> = vec![
    //    vec![1,2,3],
    //    vec![4,5,6],
    //    vec![7,8,9],
    //];
    //dbg!(&testmap);
    res += find_diagonally_minus45(&charmap);
    res += find_diagonally_plus45(&charmap);
        
    println!("Part one result {}", res);
}