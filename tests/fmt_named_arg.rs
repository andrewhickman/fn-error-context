use fn_error_context::error_context;

#[error_context("context {arg}", arg = arg)]
fn do_stuff(arg: u32) -> anyhow::Result<()> {
    anyhow::bail!("error {}", arg)
}

fn main() {
    assert_eq!(
        format!("{:#}", do_stuff(1).unwrap_err()),
        "context 1: error 1"
    );
}
