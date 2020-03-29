use fn_error_context::context;

#[context("context", 1)]
fn do_stuff() -> anyhow::Result<()> {
    anyhow::bail!("error")
}

fn main() {}
