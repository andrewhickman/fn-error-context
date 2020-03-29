[![Crates.io][ci]][cl] [![Docs.rs][di]][dl]

[ci]: https://img.shields.io/crates/v/fn-error-context.svg
[cl]: https://crates.io/crates/fn-error-context/

[di]: https://docs.rs/fn-error-context/badge.svg
[dl]: https://docs.rs/fn-error-context/

# fn-error-context

An attribute macro to add context to errors from a function.

```
#[context("failed to parse config at `{}`", path.display())]
pub fn parse_config(path: &Path) -> anyhow::Result<u32> {
    let text = read_to_string(path)?;
    Ok(text.parse()?)
}
```