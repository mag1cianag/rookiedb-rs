use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
#[clap(author,version,about,long_about = None)]
struct Cli {}

impl Cli {
    fn run(&self) {
        loop {
            print!("Rsdb>: ");
            io::stdout().flush().expect("Flush Error");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Repl: Failed to read input");
            let input = input.trim();
            match input {
                ".exit" => {
                    println!("Bye Bye :)");
                    break;
                }
                _ => {
                    println!("get input ");
                }
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    cli.run();
}
