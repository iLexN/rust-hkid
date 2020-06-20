fn main() {
    let p1 = String::from("r");
    let p2 = String::from("000004");
    let p3 = String::from("2");

    let mut ss = p1;
    ss.push_str(p2.as_str());
    ss.push_str("(");
    ss.push_str(p3.as_str());
    ss.push_str(")");

    let result = rust_hkid::check_str(ss.as_str());
    println!("result: {}", result);
}
