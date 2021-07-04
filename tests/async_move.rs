use fn_error_context::context;

#[context(move, "context")]
async fn borrows(val: &mut u32) -> anyhow::Result<&u32> {
    Ok(&*val)
}

fn main() {}
