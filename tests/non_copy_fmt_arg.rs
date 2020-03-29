use fn_error_context::error_context;

#[error_context("context {}", arg)]
fn do_stuff(arg: String) -> anyhow::Result<()> {
    anyhow::bail!("error {}", arg)
}

fn main() {
    assert_eq!(
        format!("{:#}", do_stuff("hello".to_owned()).unwrap_err()),
        "context hello: error hello"
    );
}
