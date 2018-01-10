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
use super::Exportable;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


pub struct MySQL {}

impl MySQL {
    /// Returns a new instance of the MySQL exporter.
    pub fn new() -> MySQL {
        MySQL {}
    }
}

impl Exportable for MySQL {
    /// Exports many logs into the MySQL format.
    ///
    /// # Parameters
    ///
    /// `&self` - The struct.
    /// `args` - A HashMap that contains the cli parameters after being filtered.
    /// In the case of Exportable, they are filtered on "--output-". Because
    /// they all have the same prefix ("--output-"), this prefix was removed
    /// before being passed into the HashMap.
    /// For example "--output-file=myfile.log" has a hey of "file" and a value of "myfile.log".
    /// `logs` - A vector of log::Log that contains all the logs to export.
    /// 
    /// TODO: Creates a better schema for the database.
    fn export(&self, args: HashMap<String, String>, logs: Vec<log::Log>) {
        let mut content: String = String::new();
        for log in logs {
            content.push_str(
                &format!("INSERT INTO `log` (date, name, level, message) VALUES (\"{}\", \"{}\", \"{}\", \"{}\");\n",
                &log.date[..], &log.name[..], &log.level[..], &log.message[..])    
            );
        }
        // Choose to write into a file or print.
        if let Some(filename) = args.get("file") {
            let mut file = match File::create(filename) {
                Ok(f)   => f,
                Err(_)  => {
                    panic!("The output file {} can't be open.", filename);
                }
            };
            match file.write_all(content.as_bytes()) {
                Ok(_)   =>  {},
                Err(_)  =>  {
                    panic!("The logs can't be written into the file {}.", filename);
                }
            };
        } else {
            println!("{}", content);
        }
    }
}
