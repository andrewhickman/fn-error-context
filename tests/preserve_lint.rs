// See https://github.com/andrewhickman/fn-error-context/issues/3
#![deny(dead_code)]

use fn_error_context::context;

#[context("context")]
fn do_stuff() -> anyhow::Result<()> {
    anyhow::bail!("error")
}

fn main() {}
