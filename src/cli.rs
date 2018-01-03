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



use std::env;
use std::collections::HashMap;

/// Gets the arguments given to the executable.
pub fn get_args() -> Vec<String> {
    env::args().collect()
}


/// Gets the key => value pairs of the cli arguments, filtered by a prefix.
/// It can be useful to find all the parameters to give to the "input" module,
/// all the cli arguments beginning with "--input-" are given as parameter to the
/// needed input module.
///
/// # Examples
///
/// ```
///  //Simulates get_args()
///  let args = vec!["--input-file=myfile.log", "--input-other=other-param"];
///  let filtered_params = get_params(&args, "--input-");
///  assert_eq!(filtered_params.get("file"), Some("myfile.log"));
///  assert_eq!(filtered_params.get("other"), Some("other-param"));
/// ```
///
pub fn get_params(args: &Vec<String>, arg_prefix: &str) -> HashMap<String, String> {
    let mut params : HashMap<String, String> = HashMap::new();
    //Checks every cli argument in order to get the argument name and the
    //associated value in a HashMap.
    for (_index, value) in args.iter().enumerate() {
        if value.starts_with(arg_prefix) && value.contains('=') {
            let end_prefix_index: usize = arg_prefix.len();
            let arg_parts: Vec<&str> = value.split('=').collect();
            params.insert(arg_parts[0][end_prefix_index..].to_owned(), arg_parts[1].to_owned());
        }
    }
    params
}
