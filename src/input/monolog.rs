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
            if let Some(l) = log::Log::from_line(line) {
                logs.push(l);
            }
        }

        logs
    }
}

