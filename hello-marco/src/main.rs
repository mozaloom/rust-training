//A command-line tool to play Marco Polo
use clap::Parser; //Imports the Parser trait from the clap crate.

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Mohammed Zaloom",
    about = "A Marco Polo game"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,   
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Mohammed Zaloom")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command given"),
    }
}
