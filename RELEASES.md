# Releases
## Cadence
We usually release a new major `egui_plot` whenever there is a new major `egui` release, which is usually once every two months or so.


## Versioning
The version in `main` is always the version of the last published crate.
This is so that users can easily patch their `egui_plot` to the version on `main` if they want to.


## Governance
Releases are generally done by [emilk](https://github.com/emilk/), but the [rerun-io](https://github.com/rerun-io/) organization (where emilk is CTO) also has publish rights to all the crates.


## Rust version policy
Our Minimum Supported Rust Version (MSRV) is always _at least_ two minor release behind the latest Rust version. This means users of `egui_plot` aren't forced to update to the very latest Rust version.

We don't update the MSRV in a patch release, unless we really, really need to.


# Release process
## Patch release
* [ ] Make a branch off of the latest release
* [ ] cherry-pick what you want to release
* [ ] run `cargo semver-checks`

## Optional polish before a major release
* [ ] improve the demo a bit
* [ ] `cargo update`
* [ ] `cargo outdated` (or manually look for outdated crates in each `Cargo.toml`)

## Release testing
* [ ] IMPORTANT: test with [Rerun](https://github.com/rerun-io/rerun/)
* [ ] test the demo app
* [ ] test the web demo
  - test on mobile
  - test on chromium
* [ ] `./scripts/check.sh`
* [ ] check that CI is green

## Preparation
* [ ] optionally record gif or take a screenshot for `CHANGELOG.md` release note (and later twitter post)
* [ ] update changelogs using `scripts/generate_changelog.py`
  - For major releases, always use the upcoming release, e.g. `--version 0.x.0`
* [ ] bump version numbers in workspace `Cargo.toml`

## Actual release
I usually do this all on the `main` branch, but doing it in a release branch is also fine, as long as you remember to merge it into `main` later.

* [ ] `./scripts/generate_changelog.py --version 0.x.0`
* [ ] bump version number in `Cargo.toml`
* [ ] `cargo clippy`
* [ ] `git commit -m 'Release 0.x.0 - summary'`
* [ ] `cargo publish -p egui_plot`
* [ ] `git tag -a 0.x.0 -m 'Release 0.x.0 - summary'`
* [ ] `git pull --tags ; git tag -d latest && git tag -a latest -m 'Latest release' && git push --tags origin latest --force ; git push --tags`
* [ ] merge release PR or push to `main`
* [ ] check that CI is green
* [ ] do a GitHub release: https://github.com/emilk/egui_plot/releases/new
* [ ] wait for documentation to build: https://docs.rs/releases/queue
