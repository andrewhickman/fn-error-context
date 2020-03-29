use fn_error_context::error_context;

#[error_context("context")]
fn do_stuff(arg: u32) -> anyhow::Result<()> {
    anyhow::bail!("error {}", arg)
}

fn main() {}
