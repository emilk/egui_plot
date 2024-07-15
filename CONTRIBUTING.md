# Contribution Guidelines


## Discussion

You can ask questions, share screenshots and more at [GitHub Discussions](https://github.com/emilk/egui_plot/discussions).

There is an `egui` discord at <https://discord.gg/vbuv9Xan65>.


## Filing an issue

[Issues](https://github.com/emilk/egui_plot/issues) are for bug reports and feature requests. Issues are not for asking questions (use [Discussions](https://github.com/emilk/egui_plot/discussions) or [Discord](https://discord.gg/vbuv9Xan65) for that).

Always make sure there is not already a similar issue to the one you are creating.

If you are filing a bug, please provide a way to reproduce it.


## Making a PR

For small things, just go ahead an open a PR. For bigger things, please file an issue first (or find an existing one) and announce that you plan to work on something. That way we will avoid having several people doing double work, and you might get useful feedback on the issue before you start working.

You can test your code locally by running `./scripts/check.sh`.

When you have something that works, open a draft PR. You may get some helpful feedback early!
When you feel the PR is ready to go, do a self-review of the code, and then open it for review.

Don't worry about having many small commits in the PR - they will be squashed to one commit once merged.

Please keep pull requests small and focused. The smaller it is, the more likely it is to get merged.

## PR review
It is very easy to add complexity to a project, but remember that each line of code added is code that needs to be maintained in perpetuity, so we have a high bar on what get merged!

When reviewing, we look for:
* The PR title and description should be helpful
* Breaking changes are documented in the PR description
* The code should be readable
* The code should have helpful docstrings
* The code should follow the [Code Style](CONTRIBUTING.md#code-style)

Note that each new egui_plot release have some breaking changes, so we don't mind having a few of those in a PR.
f course, we still try to avoid them if we can, and if we can't we try to first deprecate old code using the `#[deprecated]` attribute.


## Code Style
* Read some code before writing your own
* Leave the code cleaner than how you found it
* Write idiomatic rust
* Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* Add blank lines around all `fn`, `struct`, `enum`, etc
* `// Comment like this.` and not `//like this`
* Use `TODO` instead of `FIXME`
* Add your github handle to the `TODO`:s you write, e.g: `TODO(emilk): clean this up`
* Avoid `unsafe`
* Avoid `unwrap` and any other code that can cause panics
* Use good names for everything
* Add docstrings to types, `struct` fields and all `pub fn`
* Add some example code (doc-tests)
* Before making a function longer, consider adding a helper function
* If you are only using it in one function, put the `use` statement in that function. This improves locality, making it easier to read and move the code
* When importing a `trait` to use it's trait methods, do this: `use Trait as _;`. That lets the reader know why you imported it, even though it seems unused
* Avoid double negatives
* Flip `if !condition {} else {}`
* Sets of things should be lexicographically sorted (e.g. crate dependencies in `Cargo.toml`)
* Break the above rules when it makes sense


### Good:
``` rust
/// The name of the thing.
pub fn name(&self) -> &str {
    &self.name
}

fn foo(&self) {
    // TODO(emilk): this can be optimized
}
```

### Bad:
``` rust
//gets the name
pub fn get_name(&self) -> &str {
    &self.name
}
fn foo(&self) {
    //FIXME: this can be optimized
}
```

### Coordinate system
The left-top corner of the screen is `(0.0, 0.0)`,
with `Vec2::X` increasing to the right and `Vec2::Y` increasing downwards.

`egui` uses logical _points_ as its coordinate system.
Those related to physical _pixels_ by the `pixels_per_point` scale factor.
For example, a high-dpi screen can have `pixels_per_point = 2.0`,
meaning there are two physical screen pixels for each logical point.

Angles are in radians, and are measured clockwise from the X-axis, which has angle=0.


### Avoid `unwrap`, `expect` etc.
The code should never panic or crash, which means that any instance of `unwrap` or `expect` is a potential time-bomb. Even if you structured your code to make them impossible, any reader will have to read the code very carefully to prove to themselves that an `unwrap` won't panic. Often you can instead rewrite your code so as to avoid it. The same goes for indexing into a slice (which will panic on out-of-bounds) - it is often preferable to use `.get()`.

For instance:

``` rust
let first = if vec.is_empty() {
    return;
} else {
    vec[0]
};
```
can be better written as:

``` rust
let Some(first) = vec.first() else {
    return;
};
```
