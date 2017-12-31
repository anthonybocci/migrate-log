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
    ///
    /// # Example
    ///
    /// ```
    ///    use log::Log;
    ///    let mut log = Log::new();
    ///    log.date = "2017-12-31 23:58:42";
    ///    log.name = "My logger name";
    ///    log.level = "CRITICAL";
    ///    log.message = "Houston we have a problem";
    ///    assert_eq!(log.to_sql(), "INSERT INTO `log` (date, name, level, message) VALUES (\"2017-12-31 23:59:42\", \"My logger name\", \"CRITICAL\", \"Houston we have a problem\")");
    /// ```
    pub fn to_sql(&self) -> String {
        format!("INSERT INTO `log` (date, name, level, message) VALUES (\"{}\", \"{}\", \"{}\", \"{}\");",
                self.date, self.name, self.level, self.message)
    }

    /// Instanciates a new Log from a line if this line has the right pattern.
    ///
    /// The line schema is the one of Monolog.
    ///
    /// # Arguments
    ///
    /// * `line` - A string slice that is the line to parse.
    pub fn from_line(line: &str) -> Option<Log> {
        let mut log = Log::new();
        //Uses lazy_static in order to compile the regex only once.
        lazy_static! {
            /// The regex used to find and capture a Log line.
            static ref LOG_REGEX: Regex = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{1,2}:\d{1,2}:\d{1,2})\] (.+)\.([A-Z]+): (.+) \[(.*)\] \[(.*)\]").unwrap();
        }
        //Checks if the regex matches and if we found all the datas.
        let captured_dates = match LOG_REGEX.captures(line) {
            Some(m) => m,
            None    => {
                return None;
            }
        };

        log.date = captured_dates[1].to_owned();
        log.name = captured_dates[2].to_owned();
        log.level = captured_dates[3].to_owned();
        log.message = captured_dates[4].to_owned();
        log.context_raw = captured_dates[5].to_owned();
        log.extra_raw = captured_dates[6].to_owned();
        Some(log)
    }
}


