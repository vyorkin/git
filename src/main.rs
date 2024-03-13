use clap::Parser;
use git::cli::Args;
use git::cli::Command;
use git::command;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            command::init::run()?;
        }
        Command::CatFile {
            object_type,
            object_hash,
            pretty_print,
        } => {
            command::cat_file::run(&object_type, &object_hash, pretty_print)?;
        }
        Command::HashObject { file_path, write } => {
            command::hash_object::run(file_path, write)?;
        }
        Command::LsTree { name_only } => {
            command::ls_tree::run(name_only)?;
        }
    }
    Ok(())
}
