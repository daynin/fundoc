# Syntax

There are only two keywords for writing fundoc docstrings (for now):

- `@Article <Article name>` for marking documentation sections to tell in which articale this section should
be merged. You can use `markdown` syntax in documentation sections.
- `@Ignore` for ignoring a marked documentation section.

Example:

```rust
/**
* @Article How it works
*
* # Title of the article
*
* Some text
*/
fn main() {}
```
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L23-L42)
