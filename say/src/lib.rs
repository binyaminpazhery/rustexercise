pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let chunks = get_chunks(n);
    let mut result = String::new();
    for (idx, chunk) in chunks.iter().enumerate() {
        if *chunk == 0 {
            continue;
        }

        let digit_pos = digit_positions(idx);
        let hundreds_val = hundreds(*chunk);
        let cur_str = format!("{} {} ", hundreds_val, digit_pos);
        result.insert_str(0, &cur_str)
    }
    result.trim().to_string()
}

fn get_chunks(n: u64) -> Vec<u64> {
    let mut chunks = Vec::new();
    let mut cur_num = n;
    while cur_num != 0 {
        chunks.push(cur_num % 1000);
        cur_num = cur_num / 1000;
    }
    chunks
}

fn ones(n: u64) -> String {
    match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("Unexpected ones: {}", n),
    }
    .to_string()
}

fn tens(n: u64) -> String {
    match n {
        x if x < 10 => ones(n),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        x if x >= 10 && x <= 99 => {
            let digit = ones(n % 10);
            let ten = special_tens(n / 10).unwrap_or(ones(n / 10));
            match &digit[..] {
                "" => format!("{}ty", ten),
                _ => format!("{}ty-{}", ten, digit),
            }
        }
        _ => panic!("unexpected tens: {}", n),
    }
}

fn special_tens(n: u64) -> Option<String> {
    match n {
        2 => Some("twen".to_string()),
        3 => Some("thir".to_string()),
        4 => Some("for".to_string()),
        5 => Some("fif".to_string()),
        8 => Some("eigh".to_string()),
        _ => None,
    }
}

fn hundreds(n: u64) -> String {
    let hundred = ones(n / 100);
    let ten = tens(n % 100);
    match &hundred[..] {
        "" => ten,
        _ => format!("{} hundred {}", hundred, ten),
    }
}

fn digit_positions(idx: usize) -> String {
    match idx {
        0 => "",
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => panic!("Unsupported position: {}", idx),
    }
    .to_string()
}