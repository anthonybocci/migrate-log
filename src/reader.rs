use log;

use std::fs::File;
use std::io::prelude::*;

/// Reads the content of a file and sets it `content`.
/// Panics if the file can't be open.
pub fn read_content(filename: &str, content: &mut String) {
    let log_file_name = filename;
    let mut log_file = File::open(log_file_name).unwrap();
    //Reads the log file
    log_file.read_to_string(content).unwrap();
}

/// Reads all lines in a file and set a Vector of Log.
///
/// # Arguments
///
/// * filename  - The name of the file to read
/// * lines     - The Vector that will contain the Logs.
pub fn read_lines(filename: &str, lines: &mut Vec<log::Log>) {
    let mut content = String::new();
    read_content(filename, &mut content);

    for line in content.lines() {
        if let Some(l) = log::Log::from_line(line) {
            lines.push(l);
        }
    }
}
