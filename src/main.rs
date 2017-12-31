#[macro_use] extern crate lazy_static;
extern crate regex;

mod log;
mod cli;


fn main() {

    let input_filename = match cli::get_input_file() {
        Some(f) => f,
        None    => {
            panic!("There is no input file. Consider using --input=")
        }
    };
    println!("Reading log file {}...", input_filename);

    let mut lines : Vec<log::Log> = Vec::new();
    log::read_lines("/home/anthony/Bureau/test.log", &mut lines);

    //let log = log::Log::from_line("[2017-12-31 10:24:42] Clima logger.WARNING: Foo [] []");
    for line in &lines {
        println!("{}", line.to_sql());
    }

}
