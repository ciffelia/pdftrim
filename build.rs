use std::{fs, io};

use clap::CommandFactory;
use clap_complete::Shell;

include!("src/cli.rs");

fn main() -> Result<(), io::Error> {
    let mut cmd = Cli::command();

    for shell in [
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Zsh,
    ] {
        fs::create_dir_all("src/completion")?;
        let mut buf = io::BufWriter::new(fs::File::create(format!("src/completion/{shell}"))?);

        clap_complete::generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut buf);
    }

    Ok(())
}
