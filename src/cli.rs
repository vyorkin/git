use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Command {
    Init,
    CatFile {
        object_type: String,
        object_hash: String,
        #[clap(short = 'p')]
        pretty_print: bool,
    },
    HashObject {
        file_path: PathBuf,
        #[clap(short = 'w')]
        write: bool,
    },
    LsTree {
        name_only: bool,
    },
}
