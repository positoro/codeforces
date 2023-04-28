fn main() {
    let input_w = input_w();

    if input_w % 2 == 0 && input_w != 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_w() -> u8 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u8> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
