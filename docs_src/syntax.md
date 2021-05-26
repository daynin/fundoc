# Syntax

`@Article <Article name>` is for marking documentation sections to tell in which articale this section should
be merged. You can use `markdown` syntax in documentation sections.

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
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L34-L50)

`@FileArtcile` allows you to mark a whole file is a source of documentation for a specified
article.

Example:

```rust
/**
* @FileArticle How it works
*/

/**
 * Documentation for `main`
 */
fn main() {}

/**
 * Documentation for `parse_files`
 */
fn parse_files() {}
```
In that case all comments from a file will be parsed in the same way if they were marked with
`@Article How it works`

If you want to exclude some comment from parsing you need to use `@Ignore` attribute in that
section.

Example:

```rust
/**
* @FileArticle How it works
*/

/**
 * Documentation for `main`
 */
fn main() {}

/**
 * @Ignore
 * This comment will be ignored.
 */
fn parse_files() {}
```
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L54-L98)

`@Ignore` is for ignoring a marked documentation section.
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L102-L103)

`@CodeBlockStart` and `@CodeBlockEnd` allow to include code from a current file as an
example.

Example:

```rust
/**
* @Article Usage examples
* Here you can see a function call:
* @CodeBlockStart rust
*/
calc_size(item)
/**
* @CodeBlockEnd
*/
```
[[~]](https://github.com/daynin/fundoc/blob/master/src/parser.rs#L107-L123)
