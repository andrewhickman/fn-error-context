use fn_error_context::error_context;

struct Foo(u32);

impl Foo {
    #[error_context("context")]
    fn do_stuff(&self) -> anyhow::Result<()> {
        anyhow::bail!("error {}", self.0)
    }
}

fn main() {
    assert_eq!(
        format!("{:#}", Foo(1).do_stuff().unwrap_err()),
        "context: error 1"
    );
}
