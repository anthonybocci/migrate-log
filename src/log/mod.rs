
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

/// Represents a log item. It can be either a line in a file text,
///in a table, the schema is the same.
#[derive(Debug)]
pub struct Log {
    /// The date and time of the log.
    pub date: String,
    /// The name of the logger.
    pub name: String,
    /// The log level. A list of the most used is present
    /// [here](https://github.com/Seldaek/monolog/blob/master/doc/01-usage.md#log-levels)
    pub level: String,
    /// The log message.
    pub message: String,
    /// The log context. Usually a user identifier or something like that.
    pub context: HashMap<String, String>,
    /// The log extra informations.
    pub extra: HashMap<String, String>,
    /// The log context as a raw string. It's necessary because the context is
    /// usually read as JSON.
    pub context_raw: String,
    /// The log extra as a raw string. It's necessary because the context is
    /// usually read a JSON.
    pub extra_raw: String,
}


impl Log {
    /// Returns a new Log instance.
    ///
    /// All the fields are set to their zero value.
    ///
    /// # Example
    ///
    /// ```
    ///    use log::Log;
    ///    let log = Log::new();
    /// ```
    pub fn new() -> Log {
        Log {
            date: String::new(),
            name: String::new(),
            level: String::new(),
            message: String::new(),
            context: HashMap::new(),
            extra: HashMap::new(),
            context_raw: String::new(),
            extra_raw: String::new()
        }
    }

    /// Returns a SQL String representation of the log.
    pub fn to_sql(&self) -> String {
        format!("INSERT INTO `log` 
                (date, name, level, message)
                VALUES ({}, \"{}\", \"{}\", \"{}\");",
                self.date, self.name, self.level, self.message)
    }

    /// Instanciates a new Log from a line.
    ///
    /// The line schema is the one of Monolog.
    ///
    /// # Arguments
    ///
    /// * `line` - A string slice that is the line to parse.
    pub fn from_line(line: &str) -> Log {
        let mut log = Log::new();
        lazy_static! {
            static ref LOG_REGEX: Regex = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{1,2}:\d{1,2}:\d{1,2})\] (.+)\.([A-Z]+): (.+) \[(.*)\] \[(.*)\]").unwrap();
        }
        let captured_dates = match LOG_REGEX.captures(line) {
            Some(m) => m,
            None    => { panic!("The log line as not the right pattern.");}
        };

        log.date = captured_dates[1].to_owned();
        log.name = captured_dates[2].to_owned();
        log.level = captured_dates[3].to_owned();
        log.message = captured_dates[4].to_owned();
        log.context_raw = captured_dates[5].to_owned();
        log.extra_raw = captured_dates[6].to_owned();
        log
    }
}





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
pub fn read_lines(filename: &str, lines: &mut Vec<Log>) {
    let mut content = String::new();
    read_content(filename, &mut content);

    for line in content.lines() {
        lines.push(Log::from_line(line))
    }
}

