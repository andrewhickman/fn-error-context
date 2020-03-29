use fn_error_context::error_context;

#[error_context("context")]
fn do_stuff() -> failure::Fallible<()> {
    failure::bail!("error")
}

fn main() {
    let err = do_stuff().unwrap_err();
    assert_eq!(format!("{}", err), "context");
    assert_eq!(format!("{}", err.as_fail().cause().unwrap()), "error");
}
