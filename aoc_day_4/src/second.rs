fn vector_compare(v1: &[u8], v2: &[u8]) -> bool{
    if v1.len() != v2.len(){
        return false
    }
    return v1.iter().zip(v2.iter()).filter(|&(a, b)| a == b).count() == v1.len();
}

fn is_mas(v1: &Vec<u8>) -> bool{
    let mas = Vec::from("MAS");
    let sam = Vec::from("SAM");

    return vector_compare(v1, &mas) || vector_compare(v1, &sam);
}

fn create_cross(map: &Vec<Vec<u8>>, lidx: usize, cidx: usize) -> (Vec<u8>, Vec<u8>){

    assert!(lidx > 0 && lidx < map.len() - 1 && cidx > 0 && cidx < map.len() - 1, "Wrong index");

    let v1 = vec![map[lidx-1][cidx-1], map[lidx][cidx], map[lidx+1][cidx+1]];
    let v2 = vec![map[lidx+1][cidx-1], map[lidx][cidx], map[lidx-1][cidx+1]];

    return (v1, v2);
}

fn scan_map(map: &Vec<Vec<u8>>) -> i32 {
    let mut res = 0;
    for lidx in 1..map.len()-1{
        for cidx in 1..map.len()-1{
            if map[lidx][cidx] == b'A'{
                let (v1, v2) = create_cross(map, lidx, cidx);
                if is_mas(&v1) && is_mas(&v2) {
                    res += 1;
                }
            }
        }
    }

    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut charmap: Vec<Vec<u8>> = vec![];

    for line in lines{
        let l = line.as_bytes();
        let l = Vec::from(l);
        charmap.push(l);
    }

    res = scan_map(&charmap);
    println!("Part two result {}", res);
}