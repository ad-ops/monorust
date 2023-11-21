use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};


mod git;
mod interactive;
mod server;

/// Working with monorepo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Shows all commands that would be run but does nothing
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Starts in interactive (wizard) mode
    Interactive,
    /// Checksout code to start working on
    Checkout {
        /// Module to work on
        #[arg(short, long)]
        module: String,

        /// Dir to clone code to
        #[arg(short, long)]
        target_dir: PathBuf,
    },
    /// Trigger compilation of code
    Build {
        /// Module to build
        #[arg(short, long)]
        module: String,

        /// Environment to build
        #[arg(short, long)]
        environment: String,
    },
    /// Trigger deploy of compiled package
    Deploy {
        /// Module to deploy
        #[arg(short, long)]
        module: String,

        /// Environment to deploy to
        #[arg(short, long)]
        environment: String,
    },
    /// Removes files
    Clean {
        /// Dir to clean
        #[arg(short, long)]
        target_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    match args.command {
        Commands::Interactive => {
            if args.dry_run {
                println!("Running interactive mode...");
            }
            else {
                interactive::run_interactive()?;
            }
        }
        Commands::Checkout { module, target_dir } => {
            match git::checkout(&target_dir, &module, args.dry_run) {
                Ok(text) => println!("Response: {text}"),
                Err(e) => eprintln!("Error: {e}"),
            };
            return Ok(());
        }
        Commands::Build {
            module: _,
            environment: _,
        } => todo!(),
        Commands::Deploy {
            module: _,
            environment: _,
        } => todo!(),
        Commands::Clean { target_dir } => {
            if args.dry_run {
                println!("Would remove dir: {target_dir:?}");
            } else {
                let dir_to_remove = target_dir.join("monorust");
                if dir_to_remove.exists() {
                    println!("Removing {dir_to_remove:?}...");
                    match std::fs::remove_dir_all(dir_to_remove) {
                        Ok(_) => println!("Everything removed!"),
                        Err(e) => eprintln!("Error removing files: {e}"),
                    }
                } else {
                    println!("Nothing to remove...");
                }
            }
            return Ok(());
        }
    }

    Ok(())
}

