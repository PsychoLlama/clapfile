#![warn(clippy::print_stdout, clippy::print_stderr)]
#![deny(
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    clippy::correctness,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::self_named_module_files,
    clippy::shadow_reuse
)]

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {}

fn main() {
    Args::parse();
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_basic_command_parsing() {
        // Simply test that it does not crash.
        Args::command().get_matches_from(vec![""]);
    }
}
