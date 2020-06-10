use std::collections::HashMap;

fn main() {
    let s = String::from("251985");

    let s2 = clear_string(s);
    println!("{}", &s2.as_str()[0..1]);

    let p1 = clear_string(String::from("Z"));
    cal_part2_remainder(s2, get_char_sum(p1));
}

fn clear_string(s: String) -> String {
    s.trim().to_uppercase()
}

fn get_char_map() -> HashMap<char, i32> {
    let mut char_num = HashMap::new();
    for (num, c) in (b'A'..=b'Z').enumerate() {
        let key = c as char;
        let num2 = num as i32;
        char_num.insert(key, 10 + num2);
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
            let mut total: i32 = 0;
            for (key, value) in char_list.iter().enumerate() {
                total += weight[&key] * *char_map.get(&value).unwrap();
            }
            total
        }
        _ => 0, // how to return error?
    }
}

fn cal_part2_remainder(s: String, char_sum: i32) -> i32 {
    let mut sum = 0;

    for (i, v) in s.chars().enumerate() {
        let index = i as i32;
        println!("{}:{}", i, v.to_digit(10).unwrap());
        let value = v.to_digit(10).unwrap() as i32;
        sum += (7 - index) * value;
    }

    let x = 11;
    println!("sum is {}", sum);

    let y = x - ((char_sum + sum) % x);
    //todo : seem wrong
    println!("haha:: {}", y);
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_weight() {
        let weight = get_char_weight();
        let first: usize = 0;
        let second: usize = 1;
        assert_eq!(weight[&first], 9);
        assert_eq!(weight[&second], 8);
    }

    #[test]
    fn test_get_char_sum() {
        let a = String::from("B");
        assert_eq!(get_char_sum(a), 412);

        let a = String::from("Z");
        assert_eq!(get_char_sum(a), 604);

        let a = String::from("CA");
        assert_eq!(get_char_sum(a), 188);
    }
}
