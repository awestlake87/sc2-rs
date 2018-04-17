use std::{path::PathBuf, process::Command};

use futures::prelude::*;
use tokio_core::reactor;
use tokio_process::CommandExt;

use {Error, Result};

#[async]
pub fn convert_to_wine_path(
    path: PathBuf,
    handle: reactor::Handle,
) -> Result<PathBuf> {
    let stdout = await!(
        Command::new("winepath")
            .arg("-w")
            .arg(&path)
            .output_async(&handle)
    )?.stdout;

    Ok(PathBuf::from(String::from_utf8(stdout)?.trim()))
}
