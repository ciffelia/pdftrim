use std::{fs, io, path::PathBuf};

use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;

include!("src/cli.rs");

fn main() -> Result<(), io::Error> {
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().parse().unwrap();
    let completions_dir = out_dir.join("completions");
    fs::create_dir_all(&completions_dir)?;

    let mut cmd = Cli::command();

    for &shell in Shell::value_variants() {
        let completion_file = completions_dir.join(shell.to_string());
        let mut buf = io::BufWriter::new(fs::File::create(&completion_file)?);

        clap_complete::generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut buf);
    }

    Ok(())
}
