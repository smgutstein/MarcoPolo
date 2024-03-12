// A Command Line Tool To Play Marco Polo
// Now that we have a library that can play Marco Polo, let's create a command line tool that uses the library to play the game.

// Create a new binary project called marco-polo by running the following command:
// $ cargo new marco-polo --bin
// This command creates a new binary project called marco-polo. The --bin flag tells Cargo to create a binary project instead of a library project.
use clap::{Command, Parser};
use hello_marco::marco_polo;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option(Commands),
}

#[derive(Parser)]
enum Commands {
    Play {
        #[clap(short, long)]
        name: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            print!("{}", result)
        }
        None => println!("No command provided"),
    }
}
