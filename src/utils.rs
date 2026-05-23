use anyhow::Context;
use std::path::PathBuf;

pub fn find_matching_sockets() -> anyhow::Result<Vec<PathBuf>> {
    let pattern = "/tmp/bridge-*.sock";
    Ok(glob::glob(pattern)
        .context("couldn't search for Neovim sockets")?
        .filter_map(Result::ok)
        .filter(|path| path.exists())
        .collect())
}
