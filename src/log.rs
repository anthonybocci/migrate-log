/// This file is part of Migrate Log.
///
/// Migrate Log is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// any later version.
///
/// Migrate Log is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with Migrate Log.  If not, see <http://www.gnu.org/licenses/>.


use std::collections::HashMap;

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
}


