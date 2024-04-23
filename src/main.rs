use std::fs::File;
use std::io::{stdin, stdout, BufReader, IsTerminal, Read};
use std::path::PathBuf;

use anyhow::{bail, Result};
use argh::FromArgs;
use rss::Channel;

/// A tool to parse and output RSS feeds
#[derive(Debug, FromArgs)]
pub(crate) struct Args {
    /// rss feed input file path
    #[argh(positional)]
    pub(crate) input_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let input: Box<dyn Read> = if let Some(path) = &argh::from_env::<Args>().input_file {
        Box::new(File::open(path)?)
    } else if !stdin().is_terminal() {
        Box::new(stdin())
    } else {
        bail!("No input")
    };

    serde_json::to_writer_pretty(stdout(), &Channel::read_from(BufReader::new(input))?)?;
    Ok(())
}
