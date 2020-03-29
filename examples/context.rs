use std::fs::read_to_string;
use std::path::Path;

use fn_error_context::error_context;

#[error_context("failed to parse config at `{}`", path.as_ref().display())]
pub fn parse_config(path: impl AsRef<Path>) -> anyhow::Result<u32> {
    let text = read_to_string(path.as_ref())?;
    Ok(text.parse()?)
}

fn main() -> anyhow::Result<()> {
    println!("config: {}", parse_config("config")?);
    Ok(())
}
