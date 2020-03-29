use fn_error_context::context;

#[context("context")]
fn do_stuff(arg: u32) -> anyhow::Result<()> {
    anyhow::bail!("error {}", arg)
}

fn main() {
    assert_eq!(
        format!("{:#}", do_stuff(1).unwrap_err()),
        "context: error 1"
    );
}
