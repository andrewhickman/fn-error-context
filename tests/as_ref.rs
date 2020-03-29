use fn_error_context::error_context;

#[error_context("context {}", arg.as_ref())]
fn do_stuff(arg: impl AsRef<str>) -> anyhow::Result<()> {
    anyhow::bail!("error {}", arg.as_ref())
}

fn main() {
    assert_eq!(
        format!("{:#}", do_stuff("hello").unwrap_err()),
        "context hello: error hello"
    );
}
