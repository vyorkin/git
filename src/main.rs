#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use clap::Parser;
use clap::Subcommand;
use git::command::cat_file;
use git::command::init;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
#[clap(rename_all = "kebab-case")]
enum Command {
    Init,
    CatFile {
        object_type: String,
        object_hash: String,
        #[clap(short = 'p')]
        pretty_print: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            init()?;
        }
        Command::CatFile {
            object_type,
            object_hash,
            pretty_print,
        } => {
            cat_file(&object_type, &object_hash, pretty_print)?;
        }
    }
    Ok(())
}
