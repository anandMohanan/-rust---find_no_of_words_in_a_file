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
    let content: String = std::fs::read_to_string(&args.file_name).expect("cannot read file");

    let result: HashMap<String, i32> = map::find_words(content);
    println!("{:#?}", result);
}
