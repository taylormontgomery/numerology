#![allow(unused)]
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    //The pattern to look for
    pattern: String,
    //the path to the file to read
    #[structopt(parse(from_os_str))]
    path:std::path::PathBuf,
    birthday: String,
}
fn main() {
    let _args = Cli::from_args();
    check_pattern(&_args.birthday);
    println!("{:?}", &_args.path);
    println!("{:?}", &_args.pattern);
    let content = std::fs::read_to_string(&_args.path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&_args.pattern){
            println!("{:?}", line)
        }
    }

    //iterate over string and if a stringed number matches, convert to real number
    //set up expected argument pattern
    //run program, wait for response, calculate or fail
    //return final number

    //parse, error message "Please try again with date format MM-dd-yyyy", then quit program
}

fn check_pattern(s: &str){

}



