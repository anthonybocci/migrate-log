#[macro_use] extern crate lazy_static;
extern crate regex;

mod log;
mod cli;
mod reader;

fn main() {
    //Gets the input filename from the command line.
    let input_filename: String = match cli::get_input_file() {
        Some(f) => f,
        None    => {
            panic!("There is no input file. Consider using --input=")
        }
    };
    println!("Reading log file {}...", input_filename);
    
    //Reads all the lines from the input filename
    let mut lines : Vec<log::Log> = Vec::new();
    reader::read_lines(&input_filename[..], &mut lines);

    //Prints the SQL statements
    for line in &lines {
        println!("{}", line.to_sql());
    }

}
