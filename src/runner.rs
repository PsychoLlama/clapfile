use clap::Parser;
use std::ffi::OsString;

use crate::config_file;

#[derive(Parser, Debug)]
pub struct Args {
    /// Configuration file.
    #[arg(short, long)]
    config: OsString,

    /// Arguments to pass to the CLI.
    #[arg(last = true)]
    rest: Vec<OsString>,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let config = config_file::load(args.config)?;
    let command: clap::Command = config.clone().into();

    let mut synthetic_argv = vec![command.get_name().into()];
    synthetic_argv.extend(args.rest);

    // Simply getting matches implements `--help` and friends.
    command.clone().get_matches_from(synthetic_argv);

    todo!("run command");
}
