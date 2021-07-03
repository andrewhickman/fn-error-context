use fn_error_context::context;

#[context("context {}", arg.as_ref())]
async fn do_stuff(arg: impl AsRef<str>) -> anyhow::Result<()> {
    anyhow::bail!("error {}", arg.as_ref())
}

fn main() {}
