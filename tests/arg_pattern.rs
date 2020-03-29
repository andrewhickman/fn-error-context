use fn_error_context::error_context;

#[error_context("context")]
fn do_stuff((x, y): (i32, u32)) -> anyhow::Result<()> {
    anyhow::bail!("error {} {}", x, y)
}

fn main() {}
