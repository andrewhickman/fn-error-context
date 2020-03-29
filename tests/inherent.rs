use fn_error_context::context;

struct Foo(u32);

impl Foo {
    #[context("context")]
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
