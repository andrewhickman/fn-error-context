use std::path::Path;

use fn_error_context::error_context;

#[error_context("failed to parse config")]
fn parse_config() -> anyhow::Result<()> {
    anyhow::bail!("nope")
}

fn main() -> anyhow::Result<()> {
    parse_config()
}
