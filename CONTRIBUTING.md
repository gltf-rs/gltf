
# Contributing guide

## Code of conduct

Contributors are expected to abide by the [Rust code of conduct](https://www.rust-lang.org/en-US/conduct.html).

## Branches

 * `master` contains the code of the latest published crate version on crates.io.
   Please do not make pull requests into `master`.
 * `incoming` contains the code for the next major / minor release. Pull requests
   for new features should be submitted here.
 * `patches` contains the code for the next patch release. Pull requests for minor
   changes and bug fixes should be submitted here.

## Style guidelines

Generally, code should look similar to that of the standard library. Please use
American English for code and documentation.

Rationale: Consistency and familiarity.

### Per-module layout

Members of each module should be organized in alphabetical order and be grouped in
the following layout:

 * `extern`s, then
 * `use`s, then
 * `pub use`s, then
 * `type` aliases, then
 * `const`s, then
 * `trait`s, then
 * `enum`s, then
 * `struct`s, then
 * `fn`s, then
 * `impl x`s, then
 * `impl x for y`s

An exception to this rule is when defining default functions for serde, in which
case they should appear immediately after the struct they initialize.

Rationale: Ease of finding things.

### Import scoping

When importing members of other crates or modules, prefer the smallest scope
necessary to achieve the task. For example, prefer

```rust
use std;
... a few other imports ...

... lots of code ...

fn foo<P>(path: P) -> std::io::Result<Bar>
    where P: AsRef<std::path::Path>
{
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    ...
}
```

to


```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;
... lots more imports ...

... lots of code ...

fn foo<P>(path: P) -> io::Result<Bar>
    where P: AsRef<Path>
{
    let mut file = File::open(path)?;
    ...
}
```

Rationale: Reduced cognitive load and probability of name clashes.

### Documentation

Ensure all exported members of modules have at least a short description of what
they are for.

Rationale: User-friendliness.
