
pub fn solve(_lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in _lines{
        let s = line.split("   ").collect::<Vec<&str>>();
        let i1: i32 = s[0].parse::<i32>().unwrap();
        let i2: i32 = s[1].parse::<i32>().unwrap();

        list1.push(i1);
        list2.push(i2);

    }
    list1.sort();
    list2.sort();

    for n in 0..list1.len(){
        res += i32::abs(list1[n]-list2[n]);
    }
    println!("Part one result {}", res);
}