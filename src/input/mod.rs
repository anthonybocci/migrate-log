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

/// A module that contains the necessary to manage a Monolog log file as input.
pub mod monolog;

/// A trat that every "importer" has to derive to. If a module or a struct
/// wants to be able to be called to import logs, it will derive this trait.
pub trait Importable {
    /// Imports many logs from any source specified in the arguments or other
    /// and returns the logs in a vector.
    ///
    /// # Parameters
    ///
    /// `&self` - The struct.
    /// `args` - A &HashMap that contains the cli parameters after being filtered.
    /// In the case of Importable, they are filtered on "--input-". Because
    /// they all have the same prefix ("--input-"), this prefix was removed
    /// before being passed into the HashMap.
    /// For example "--input-file=myfile.log" has a key of "file" and a value of "myfile.log".
    ///
    /// # Returns
    ///
    /// Returns a vector of log::Log, the universal Log for this program.
    fn import(&self, args: HashMap<String, String>) -> Vec<log::Log>;
}

/// Instantiates the input module according to the module name.
///
/// # Pameters
///
/// `module_name` - The module name.
///
/// # Panics
///
/// If module_name is an invalid module.
pub fn get_input_module(module_name: &str) -> Box<Importable> {
    match module_name {
        "monolog"   =>  Box::new(monolog::Monolog::new()),
        _           =>  {
            panic!("The input module is not implemented.");
        }
    }
}
