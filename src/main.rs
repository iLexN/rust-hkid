use std::collections::HashMap;

fn main() {
    let s = String::from("B");

    let char_weight = get_char_weight();
    for (key, _value) in s.chars().enumerate() {
        println!("c: {}", char_weight.get(&key).unwrap());
    }

    let a = get_char_map();
    for (c , v) in a {
        println!("k: {} , v : {}" , c, v);
    }

    let s2 = clear_string(s);
    println!("{}", &s2.as_str()[0..1]);

    println!("total char sum : {}", get_char_sum(s2));

    let char_num = get_char_map();

    let c = 'C';

    let m = char_num.get(&c).unwrap();
    println!("c: {}", m);
}


fn clear_string(s: String) -> String {
    let clear_str = s.trim().to_uppercase();
    clear_str
}

fn get_char_map() -> HashMap<char, i32> {
    let mut char_num = HashMap::new();
    let mut num = 0;
    for c in b'A'..=b'Z' {
        let key = c as char;
        char_num.insert(key, 10 + num);
        num = num + 1;
    }
    char_num
}

pub fn get_char_weight() -> HashMap<usize, i32> {
    let mut char_weight = HashMap::new();

    char_weight.insert(0, 9);
    char_weight.insert(1, 8);

    char_weight
}

fn get_char_sum(part1: String) -> i32 {
    let char_map = get_char_map();
    let char_list: Vec<char> = part1.chars().collect();
    let weight = get_char_weight();

    match char_list.len() {
        1 => 324 + *char_map.get(&char_list[0]).unwrap() * weight[&1],
        2 => {
            let mut total:i32 = 0;
            for (key ,value) in char_list.iter().enumerate() {
                total += weight[&key] * *char_map.get(&value).unwrap();
            }
            total
        }
        _ => 0 // how to return error?
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_weight(){
        let weight = get_char_weight();
        let first : usize = 0;
        let second : usize = 1;
        assert_eq!(weight[&first], 9);
        assert_eq!(weight[&second], 8);
    }

    #[test]
    fn test_get_char_sum(){
        let a = String::from("B");
        assert_eq!(get_char_sum(a), 412);

        let a = String::from("Z");
        assert_eq!(get_char_sum(a), 604);

        let a = String::from("CA");
        assert_eq!(get_char_sum(a), 188);
    }
}
