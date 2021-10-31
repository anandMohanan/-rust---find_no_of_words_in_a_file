use colored::*;
use std::collections::HashMap;
use structopt::StructOpt;
mod map;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    file_name: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content: Result<String, std::io::Error> = std::fs::read_to_string(&args.file_name);
    let content = match content {
        Ok(res) => res,
        Err(error) => {
            println!("{}", "Error occured:".red());
            println!("{}", error.to_string().yellow());
            panic!("{}", "Enter a valid file name,Closing the program!!".red());
        }
    };
    let result: HashMap<String, i32> = map::find_words(content);
    println!("{:#?}", result);
}
