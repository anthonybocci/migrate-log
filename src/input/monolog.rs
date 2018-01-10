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

use ::log;
use ::reader;
use super::Importable;
use std::collections::HashMap;
use regex::Regex;

pub struct Monolog {}


impl Monolog {
    /// Returns a new instance of the Monolog importer.
    pub fn new() -> Monolog {
        Monolog {}
    }
}

impl Importable for Monolog {
    /// Imports the logs from the monolog format.
    ///
    /// # Parameters
    ///
    /// `&self` - The Monolog instance.
    /// `args` - The HashMap containing the cli args.
    ///
    /// # Returns
    ///
    /// A vector containing the logs.
    ///
    /// # Panics
    ///
    /// If the filename passed in args doesn't exist.
    fn import(&self, args: HashMap<String, String>) -> Vec<log::Log> {
        let filename = match args.get("file") {
            Some(f) => f,
            None    => {
                panic!("The Monolog module needs you to give the \"--input-file=<yourfile>\".");
            }
        };
        let mut logs: Vec<log::Log> = Vec::new();
        let mut file_content = String::new();

        //Reads the file content to get the lines
        reader::read_content(filename, &mut file_content);
        for line in file_content.lines() {
            let mut log = log::Log::new();
            //Uses lazy_static in order to compile the regex only once.
            lazy_static! {
                /// The regex used to find and capture a Log line.
                static ref LOG_REGEX: Regex = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{1,2}:\d{1,2}:\d{1,2})\] (.+)\.([A-Z]+): (.+) \[(.*)\] \[(.*)\]").unwrap();
            }
            //Checks if the regex matches and if we found all the datas.
            let captured_dates = match LOG_REGEX.captures(line) {
                Some(m) => m,
                None    => {
                    continue;
                }
            };

            log.date = captured_dates[1].to_owned();
            log.name = captured_dates[2].to_owned();
            log.level = captured_dates[3].to_owned();
            log.message = captured_dates[4].to_owned();
            log.context_raw = captured_dates[5].to_owned();
            log.extra_raw = captured_dates[6].to_owned();
            logs.push(log);
        }
        logs
    }
}

