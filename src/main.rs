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
mod output;

use input::Importable;
use output::Exportable;

fn main() {

    let args = cli::get_args();

    let input_filtered = cli::get_params(&args, "--input-");
    
    
    if !input_filtered.contains_key("module") {
        panic!("An input module is required! Use --input-module=... to set one.");
    }
    let input_module_name = input_filtered.get("module").unwrap().to_owned();
    let input_module: Box<Importable> = input::get_input_module(&input_module_name[..]);
    let logs = input_module.import(input_filtered);

    let output_filtered = cli::get_params(&args, "--output-");
    if !output_filtered.contains_key("module") {
        panic!("An output module is required! Use --output-module=... to set one.");
    }
    let output_module_name = output_filtered.get("module").unwrap().to_owned();
    let output_module: Box<Exportable> = output::get_output_module(&output_module_name[..]);
    output_module.export(output_filtered, logs);
}

