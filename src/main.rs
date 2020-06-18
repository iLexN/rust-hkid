use std::char;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let p2 = String::from("000004");

    let s2 = clear_string(p2);

    let p1 = clear_string(String::from("r"));

    let check = get_part2_remainder(&p1, &s2).unwrap();
    println!("{}", check);

    let p3 = String::from("2");

    let mut ss = String::from(p1);
    ss.push_str(s2.as_str());
    ss.push_str("(");
    ss.push_str(p3.as_str());
    ss.push_str(")");

    let result = check_str(ss.as_str());
    println!("result: {}", result);
}

fn check_str(s: &str) -> bool {
    match validate(s) {
        Some(hkid) => {
            let check = get_part2_remainder(&hkid.part1, &hkid.part2).unwrap();
            check.to_string() == hkid.part3
        }
        None => false,
    }
}

struct Hkid {
    part1: String,
    part2: String,
    part3: String,
}

fn validate(s: &str) -> Option<Hkid> {
    let clear_str = clear_string(s.to_owned());
    let re = Regex::new(r"^(?P<p1>\D{1,2})(?P<p2>\d{6})\((?P<p3>[\w{1}0-9aA])\)$").unwrap();
    let c = re.captures(clear_str.as_str());
    match c {
        Some(cc) => {
            Some(Hkid {
                part1: cc.name("p1").unwrap().as_str().to_owned(),
                part2: clear_string(cc.name("p2").unwrap().as_str().to_owned()),
                part3: cc.name("p3").unwrap().as_str().to_owned(),
            })
        }
        None => None,
    }
}

fn clear_string(s: String) -> String {
    s.trim().to_uppercase()
}

fn get_char_map() -> HashMap<char, u32> {
    let mut char_num = HashMap::new();
    for (num, c) in (b'A'..=b'Z').enumerate() {
        char_num.insert(c as char, 10 + num as u32);
    }
    char_num
}

fn get_char_weight() -> HashMap<usize, u32> {
    let mut char_weight = HashMap::new();

    char_weight.insert(0, 9);
    char_weight.insert(1, 8);

    char_weight
}

fn get_char_sum(part1: &String) -> Option<u32> {
    let char_map = get_char_map();
    let char_list: Vec<char> = part1.chars().collect();
    let weight = get_char_weight();

    match char_list.len() {
        1 => Some(324 + *char_map.get(&char_list[0]).unwrap() * weight[&1]),
        2 => {
            let mut total: u32 = 0;
            for (key, value) in char_list.iter().enumerate() {
                total += weight[&key] * *char_map.get(&value).unwrap();
            }
            Some(total)
        }
        _ => None,
    }
}

fn cal_part2_remainder(s: &String, char_sum: u32) -> u32 {
    let mut sum: u32 = 0;

    for (i, v) in s.chars().enumerate() {
        sum += (7 - i as u32) * v.to_digit(10).unwrap() as u32;
    }

    let x: u32 = 11;
    x - ((char_sum + sum) % x)
}

fn get_part2_remainder(part1: &String, part2: &String) -> Option<char> {
    let remainder: u32 = cal_part2_remainder(part2, get_char_sum(part1).unwrap());
    match remainder {
        10 => Some('A'),
        11 => Some('0'),
        _ => match char::from_digit(remainder, 10) {
            Some(a) => Some(a),
            None => None,
        },
    }
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
        assert_eq!(get_char_sum(&a), Some(412));

        let a = String::from("Z");
        assert_eq!(get_char_sum(&a), Some(604));

        let a = String::from("CA");
        assert_eq!(get_char_sum(&a), Some(188));
    }

    #[test]
    fn test_check_str() {
        assert_eq!(check_str("B111111(1)"), true);
        assert_eq!(check_str("CA182361(1)"), true);
        assert_eq!(check_str("ZA182361(3)"), true);
        assert_eq!(check_str("B111112(A)"), true);
        assert_eq!(check_str("B111117(0)"), true);
        assert_eq!(check_str(" B111117(0)"), true);
        assert_eq!(check_str("z109852(8)"), true);

        assert_eq!(check_str("B111111(3)"), false);
        assert_eq!(check_str("CAC182361(1)"), false);
        assert_eq!(check_str("1B111117(0)"), false);
        assert_eq!(check_str("1111117(0)"), false);
        assert_eq!(check_str("B22(0)"), false);
        assert_eq!(check_str("B111111(G)"), false);
        assert_eq!(check_str("1111a(11)"), false);
        assert_eq!(check_str("1111a11(11)"), false);
    }
}
