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
use log;

/// A module that contains the necessary to manage a MySQL file as output.
pub mod mysql;

/// A trait that every "exporter" has to derive to. If a module or a struct
/// wants to be able to be called to export logs, it will derive this trait.
pub trait Exportable {
    /// Exports many logs into the format specified by the struct the method is called on.
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
    fn export(&self, args: HashMap<String, String>, logs: Vec<log::Log>);
}

/// Instantiates the output module according to the module name.
///
/// # Parameters
///
/// `module_name` - The module name.
///
/// # Panics
///
/// If module_name is an invalid module.
pub fn get_output_module(module_name: &str) -> Box<Exportable> {
    match module_name {
        "mysql" =>  Box::new(mysql::MySQL::new()),
        _       =>  {
            panic!("The output module is not implemented.");
        }
    }
}
