use std::env;

/// Gets the arguments given to the executable.
pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn get_input_file() -> Option<String> {
    let args = get_args();
    for (_index, value) in args.iter().enumerate() {
        if &value[..8] == "--input=" {
            return Some(value[8..].to_owned());
        }
    }
    None
}
