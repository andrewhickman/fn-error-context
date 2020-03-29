use fn_error_context::error_context;

#[error_context]
fn do_stuff() -> anyhow::Result<()> {
    anyhow::bail!("error")
}

fn main() {}
