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



use std::fs::File;
use std::io::prelude::*;

/// Reads the content of a file and sets it `content`.
/// Panics if the file can't be open.
///
/// # Parameters
///
/// `filename` - The filename to read.
/// `content` - The content where write the file content.
///
/// # Panics
///
/// If filename can't be open.
pub fn read_content(filename: &str, content: &mut String) {
    let log_file_name = filename;
    let mut log_file = match File::open(log_file_name) {
        Ok(f)   =>  f,
        Err(_)  =>  {
            panic!("The file {} can't be open.", log_file_name);
        }
    };
    //Reads the log file
    log_file.read_to_string(content).unwrap();
}


