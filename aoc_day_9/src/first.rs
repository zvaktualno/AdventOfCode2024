
fn expand(inp: &Vec<u8>) -> Vec<u8> {
    let mut alternating = true;
    let mut res: Vec<u8> = vec![];
    let mut id: u8 = 0;
    for i in inp{
        if alternating{
            alternating = false;
            res.extend(vec![id; *i as usize]);
            id += 1;
        }
        else {
            alternating = true;
            res.extend(vec![b'.'; *i as usize]);
        }
    }

    return res;
}

const INVALID: usize = 999;

fn compress(inp: &mut Vec<u8>)
{
    loop {
        // Find .
        let mut dotidx: usize = INVALID;
        for i in 0..inp.len(){
            if inp[i] == b'.'{
                dotidx = i;
                break;
            }
        }    

        let mut charidx: usize = INVALID;
        for i in (0..inp.len()).rev(){
            if inp[i] != b'.'{
                charidx = i;
                break;
            }
        } 
        if dotidx < charidx
        {
            inp[dotidx] = inp[charidx];
            inp[charidx] = b'.';
        }
        else
        {
            break;
        }
    }
}

fn checksum(inp: &Vec<u8>) -> u32
{
    let mut res = 0;
    for n in 0..inp.len(){
        if inp[n] == b'.'{
            return res;
        }
        res += (inp[n] as u32) * (n as u32);
    }
    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let line = lines[0].as_bytes().iter().map(|x| x - b'0').collect::<Vec<u8>>();
    
    let mut expanded = expand(&line);

    for c in &expanded{
        if *c == b'.'{
            print!(".");
        }
        else {
            print!("{c}");
        }
    }
    compress(&mut expanded);

    res = checksum(&expanded);

    println!("");
    println!("Part one result {}", res);
}