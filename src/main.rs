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

#[macro_use] extern crate lazy_static;
extern crate regex;

mod log;
mod cli;
mod reader;
mod input;

use input::Importable;

fn main() {

    let args = cli::get_args();

    let filtered = cli::get_params(&args, "--input-");
    
    let monolog = input::monolog::Monolog::new();
    let logs = monolog.import(filtered);

    
    //Prints the SQL statements
    for line in &logs {
        println!("{}", line.to_sql());
    }
}
