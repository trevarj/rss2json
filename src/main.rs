use std::fs::File;
use std::io::{stdin, stdout, IsTerminal, Read};
use std::path::PathBuf;

use anyhow::{bail, Result};
use argh::FromArgs;
use atom_syndication::Feed;
use rss::Channel;

/// A tool to parse and output RSS feeds
#[derive(Debug, FromArgs)]
pub(crate) struct Args {
    /// rss feed input file path
    #[argh(positional)]
    pub(crate) input_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let mut input = String::new();
    if let Some(path) = &argh::from_env::<Args>().input_file {
        let mut f = File::open(path)?;
        f.read_to_string(&mut input)?;
    } else if !stdin().is_terminal() {
        stdin().read_to_string(&mut input)?;
    } else {
        bail!("No input")
    }

    match Channel::read_from(input.as_bytes()) {
        Ok(rss) => serde_json::to_writer_pretty(stdout(), &rss)?,
        Err(_) => serde_json::to_writer_pretty(stdout(), &Feed::read_from(input.as_bytes())?)?,
    }
    Ok(())
}
