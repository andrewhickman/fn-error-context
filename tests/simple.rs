use fn_error_context::context;

#[context("context")]
fn do_stuff() -> anyhow::Result<()> {
    anyhow::bail!("error")
}

fn main() {
    assert_eq!(format!("{:#}", do_stuff().unwrap_err()), "context: error");
}
