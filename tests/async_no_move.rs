use fn_error_context::context;

#[context("{}", context.as_ref())]
async fn no_move(context: impl AsRef<str>) -> anyhow::Result<()> {
    context.as_ref();
    Ok(())
}

fn main() {}
