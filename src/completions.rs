use clap::Parser;
use clap_complete::Shell;
use std::ffi::OsString;

use crate::config_file;

#[derive(Parser, Debug)]
pub struct Args {
    /// Target shell.
    #[arg()]
    shell: Shell,

    /// Configuration file.
    #[arg(short, long)]
    config: OsString,
}

pub fn gen_to_stdout(args: Args) -> anyhow::Result<()> {
    let mut command = config_file::load(args.config)?;
    let command_name = command.get_name().to_string();

    clap_complete::generate(
        args.shell,
        &mut command,
        command_name,
        &mut std::io::stdout(),
    );

    Ok(())
}
