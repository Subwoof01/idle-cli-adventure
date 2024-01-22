use adventurer::Adventurer;
use clap::{Args, Parser, Subcommand};
use color_eyre::eyre::Result;
use debug_print::*;
use io_handler::Data;
use stat::Stat;
use std::{
    fs::{self, File},
    io::Write,
};

pub mod adventurer;
pub mod io_handler;
pub mod stat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(required = true)]
    name: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Go(Go),
    Stats,
}

#[derive(Args)]
struct Go {
    place: Option<String>,
}

fn main() -> Result<()> {
    // Load error reporting.
    color_eyre::install()?;
    // Parse commandline arguments.
    let args = Cli::parse();

    // TODO: Make this more globally accessible somehow?
    let mut data = Data {
        adventurers: Vec::new(),
    };

    // Test adventurer
    let steven = Adventurer {
        name: String::from("Steven"),
        level: 1,
        experience: 0,
        stats: vec![Stat::new("Strength", 10), Stat::new("Dexterity", 8)],
    };

    // Create the data directory if it doesn't exist yet.
    // TODO: Probably handle this better.
    match fs::create_dir("data") {
        Ok(()) => {
            debug_println!("Created 'data' directory.");
        }
        Err(_) => {
            debug_eprintln!("'data' directory already exists!");
        }
    };

    let mut file = File::create("data/steven.json")?;
    let toml = serde_json::to_string(&steven).unwrap();
    file.write_all(toml.as_bytes())?;

    // Go through arguments.
    match &args.command {
        Commands::Go(place) => match &place.place {
            Some(place) => {
                println!("Place {}", place);
            }
            None => {
                eprintln!("Not a valid string.");
            }
        },
        Commands::Stats => {
            io_handler::load_data(&mut data)?;
            println!("{:?}", data);
            println!("Stats!");
        }
    }

    Ok(())
}
