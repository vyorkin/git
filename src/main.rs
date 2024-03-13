use clap::Parser;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use git::cli::Args;
use git::cli::Command;
use git::command::cat_file;
use git::command::init;

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
