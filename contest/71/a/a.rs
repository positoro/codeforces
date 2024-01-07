fn main() {
    let n_usize: usize = input_u32() as usize;
    let s_vec_string = input_string_vector_with_low(n_usize);

    for i in 0..n_usize {
        if s_vec_string[i].len() > 10 {
            println!("{:}", generate_string(&s_vec_string[i]));
        } else {
            println!("{:}", s_vec_string[i]);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn generate_string(s: &String) -> String {
    let mut return_string: String = String::new();
    let chars: Vec<char> = s.chars().collect();
    let num: i32 = (s.len() - 2) as i32;
    let num_string: String = num.to_string();

    return_string.push(chars[0]);
    return_string = return_string + &num_string;
    return_string.push(chars[s.len() - 1]);
    return return_string;
}

fn input_string_vector_with_low(low: usize) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
}

fn input_u32() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
