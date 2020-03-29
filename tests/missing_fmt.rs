use fn_error_context::context;

#[context]
fn do_stuff() -> anyhow::Result<()> {
    anyhow::bail!("error")
}

fn main() {}
