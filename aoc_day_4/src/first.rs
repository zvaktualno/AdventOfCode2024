fn vector_compare(v1: &[u8], v2: &[u8]) -> bool{
    if v1.len() != v2.len(){
        return false
    }
    return v1.iter().zip(v2.iter()).filter(|&(a, b)| a == b).count() == v1.len();
}

fn find_horizontally(map: &Vec<Vec<u8>>) -> i32 {
    let mut result = 0;
    for line in map{
        let word1 = String::from("XMAS");
        let word2 = word1.chars().rev().collect::<String>();
        let res1 = line.windows(word1.len()).filter(|sw| vector_compare(sw, word1.as_bytes())).count();
        let res2 = line.windows(word1.len()).filter(|sw| vector_compare(sw, word2.as_bytes())).count();
        result += res1 + res2;
    }
    dbg!(result);
    return 0;
}
fn find_vertically(map: &Vec<Vec<u8>>) -> i32 {
    let flipped_map = map.clone().reverse();

    0
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut charmap: Vec<Vec<u8>> = vec![];
    for line in lines{
        let l = line.as_bytes();
        let l = Vec::from(l);
        charmap.push(l);
    }

    find_horizontally(&charmap);
    println!("Part one result {}", res);
}