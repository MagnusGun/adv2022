use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut map: HashMap<usize, Vec<char>> = HashMap::new();
    map.insert(1, vec!['M','J','C','B','F','R','L','H']);
    map.insert(2, vec!['Z','C','D']);
    map.insert(3, vec!['H','J','F','C','N','G','W']);
    map.insert(4, vec!['P','J','D','M','T','S','B']);
    map.insert(5, vec!['N','C','D','R','J']);
    map.insert(6, vec!['W','L','D','Q','P','J','G','Z']);
    map.insert(7, vec!['P','Z','T','F','R','H']);
    map.insert(8, vec!['L','V','M','G']);
    map.insert(9, vec!['C','B','G','P','F','Q','R','J']);

    let input = read_to_string("./resources/input.txt").unwrap();

    print_vec(&mut map);
    decode(input, &mut map);
    print_key(&mut map);

}

fn decode(input:String, map: &mut HashMap<usize, Vec<char>>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Invalid regex");
    

    for line in input.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some(captures) = re.captures(line) {
            println!("{}", line);
            let no = captures[1].parse::<usize>().unwrap();
            let src = captures[2].parse::<usize>().unwrap();
            let dst = captures[3].parse::<usize>().unwrap();
            
            let src_v = map.get_mut(&src).expect("couldnt find the src vector");
            let mut crates: Vec<char> = src_v.drain(src_v.len()-no..).collect();
    
            let dst_v = map.get_mut(&dst).expect("couldnt find the dst vector");
            dst_v.append(&mut crates);
            // while !crates.is_empty() {
            //     dst_v.push(crates.pop().unwrap());
            // }
       }
       print_vec(map)
    }
}

fn print_key(map:&mut HashMap<usize, Vec<char>>) {
    for k in 1..map.len()+1 {
        print!("{}",map.get_mut(&k).unwrap().pop().unwrap());
    }
    println!("");
}

fn print_vec(map:&mut HashMap<usize, Vec<char>>) {
    for k in 1..map.len()+1 {
        println!("{}:: {:?}",k,map.get_mut(&k).unwrap());
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_without (){
        let mut map: HashMap<usize, Vec<char>> = HashMap::new();
        map.insert(1, vec!['M','J','C','B','F','R','L','H']);
        map.insert(2, vec!['Z','C','D']);

        let src = map.get_mut(&1).expect("couldnt find the src vector");
        let mut crates: Vec<char> = src.drain(src.len()-2..).collect();

        let dst = map.get_mut(&2).expect("couldnt find the dst vector");
        dst.append(&mut crates);

        assert_eq!(*map.get(&1).expect("msg"), vec!['M','J','C','B','F','R']);
        assert_eq!(*map.get(&2).expect("msg"), vec!['Z','C','D','L','H']);
    }
}