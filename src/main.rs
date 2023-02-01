use clap::Parser;
use rnpm::{cmd, Commands, CLI};

fn main() {
    let args = CLI::parse();

    match &args.command {
        Some(command) => match command {
            Commands::Install(i) => cmd::install(i),
            Commands::Add(a) => cmd::add(a),
            Commands::Init => match cmd::init() {
                Ok(_) => println!("package.json created."),
                Err(e) => eprintln!("{}", e),
            },
        },
        None => println!("none!"),
    }
}
