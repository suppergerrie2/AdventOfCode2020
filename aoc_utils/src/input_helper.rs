use std::str::FromStr;

pub fn read_string_from_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

pub fn read_vector_from_file<T: FromStr>(filename: &str, separator: &str) -> Option<Vec<T>> {
    let file_data = read_string_from_file(filename);

    match file_data {
        Ok(r) => {
            let split: Vec<String> = r.split(separator).map(str::to_string).collect();
            let parsed = parse_str_vec(split);

            match parsed {
                Ok(i) => Some(i),
                Err(..) => None
            }
        }
        Err(..) => None
    }
}

fn parse_str_vec<T: FromStr>(v: Vec<String>) -> Result<Vec<T>, <T as FromStr>::Err> {
    let mut parsed_vec: Vec<T> = Vec::new();

    for string in v.iter() {
        match string.parse() {
            Ok(i) => parsed_vec.push(i),
            Err(e) => return Err(e)
        }
    }

    Ok(parsed_vec)
}

pub fn ask_read_predicate<T: FromStr, F>(message: &str, pred: F) -> T where F: Fn(&T) -> bool {
    loop {
        let i = ask_read(message);

        if pred(&i) {
            return i;
        }
    }
}

pub fn read_predicate<T: FromStr, F>(pred: F) -> Option<T> where F: Fn(&T) -> bool {
    match read() {
        Some(i) => {
            if pred(&i) {
                Some(i)
            } else {
                None
            }
        }
        None => None
    }
}

pub fn ask_read<T: FromStr>(message: &str) -> T {
    let result;

    loop {
        println!("{}", message);

        match read() {
            Some(x) => {
                result = x;
                break;
            }
            None => continue
        }
    }

    result
}

pub fn read<T: FromStr>() -> Option<T> {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse() {
        Ok(i) => Some(i),
        Err(..) => None,
    }
}
