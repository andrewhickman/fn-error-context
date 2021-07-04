use fn_error_context::context;

#[context(move, "context")]
fn foo(x: &mut u32) -> anyhow::Result<&u32> {
    Ok(&*x)
}

fn main() {}
