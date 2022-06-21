# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## 0.6.6 – 2022-06-21

- Fix: The alloc crate uses the Rust 2021 edition now ([#105](https://github.com/rust-osdev/cargo-xbuild/pull/105))

## 0.6.5 – 2021-01-25

- Ensure copied Cargo.lock is writable ([#98](https://github.com/rust-osdev/cargo-xbuild/pull/98))

## 0.6.4 – 2020-12-28

- Don't panic on metadata errors ([#100](https://github.com/rust-osdev/cargo-xbuild/pull/100))

## 0.6.3 – 2020-10-21

- Upgrade the crate to edition 2018 ([#97](https://github.com/rust-osdev/cargo-xbuild/pull/97))

## 0.6.2 – 2020-09-10

- Fix winapi issues from flock() rework ([#94](https://github.com/rust-osdev/cargo-xbuild/pull/94))

## 0.6.1 – 2020-09-10 (yanked)

- Remove fs2 dependency for broader platform support ([#91](https://github.com/rust-osdev/cargo-xbuild/pull/91))
- Cleanup: Use eprintln! instead of writeln! with stderr ([#86](https://github.com/rust-osdev/cargo-xbuild/pull/86))

## 0.6.0 – 2020-08-02

- **Breaking:** Update cargo-xbuild to new rust directory layout ([#87](https://github.com/rust-osdev/cargo-xbuild/pull/87))

## 0.5.35 - 2020-06-28

- Replace deprecated `tempdir` dependency with `tempfile` ([#84](https://github.com/rust-osdev/cargo-xbuild/pull/84))

## 0.5.34 - 2020-06-09

- Propagate `cargo-features` from project's Cargo.toml ([#82](https://github.com/rust-osdev/cargo-xbuild/pull/82))

## 0.5.33 - 2020-05-24

- Don't print warning about missing root package in quiet mode ([#79](https://github.com/rust-osdev/cargo-xbuild/pull/79))

## 0.5.32 - 2020-05-17

- Pass -Cembed-bitcode=yes instead of -Clinker-plugin-lto for sysroot build ([#73](https://github.com/rust-osdev/cargo-xbuild/pull/73))
- Respect Cargo.lock file for sysroot build ([#75](https://github.com/rust-osdev/cargo-xbuild/pull/75))

## 0.5.31 - 2020-05-12

- Set `-Clinker-plugin-lto` for the sysroot build ([#71](https://github.com/rust-osdev/cargo-xbuild/pull/71))
  - Second try to fix missing bitcode error for LTO builds (see [#69](https://github.com/rust-osdev/cargo-xbuild/issues/69))
  - Reverts "Enable lto for sysroot build to fix missing bitcode error ([#70](https://github.com/rust-osdev/cargo-xbuild/pull/70))"

## 0.5.30 - 2020-05-11

- Enable lto for sysroot build to fix missing bitcode error ([#70](https://github.com/rust-osdev/cargo-xbuild/pull/70))

## 0.5.29 - 2020-04-14

- Add an environment variable to keep the temp dir ([#67](https://github.com/rust-osdev/cargo-xbuild/pull/67))

## 0.5.28 - 2020-02-21

- Update dependencies ([#65](https://github.com/rust-osdev/cargo-xbuild/pull/65))

## 0.5.27 - 2020-02-21

- Add `cargo xfix` command ([#64](https://github.com/rust-osdev/cargo-xbuild/pull/64))

## 0.5.26 - 2020-02-19

- Improvements to args and config for lib usage ([#62](https://github.com/rust-osdev/cargo-xbuild/pull/62))

## 0.5.25 - 2020-02-17

- Fix: Not all projects have a root package ([#61](https://github.com/rust-osdev/cargo-xbuild/pull/61))

## [v0.5.24] - 2020-02-12

- Make `fn build` and `Args` public to enable use as lib ([#59](https://github.com/rust-osdev/cargo-xbuild/pull/59))

## [v0.5.23] - 2020-02-12

- Pick up xbuild config from workspace manifest ([#57](https://github.com/rust-osdev/cargo-xbuild/pull/57))

## [v0.5.22] - 2020-02-11

- Add new `panic_immediate_abort` option to the configuration table([#56](https://github.com/rust-osdev/cargo-xbuild/pull/56))

## [v0.5.21] - 2020-01-21

- Override target path for building sysroot ([#52](https://github.com/rust-osdev/cargo-xbuild/pull/52))

## [v0.5.20] - 2019-12-20

- Fix wrong feature name for memcpy=false ([#50](https://github.com/rust-osdev/cargo-xbuild/pull/50))

## [v0.5.19] - 2019-12-13

- Add `--quiet` flag that suppresses "waiting for file lock" message ([#43](https://github.com/rust-osdev/cargo-xbuild/pull/43))

## [v0.5.18] - 2019-10-08

- Add support for publishing and installing cross compiled crates ([#47](https://github.com/rust-osdev/cargo-xbuild/pull/47))

## [v0.5.17] - 2019-09-15

- Fix warning about implicit trait objects ([#46](https://github.com/rust-osdev/cargo-xbuild/pull/46))

## [v0.5.16] - 2019-09-10

- Print a warning when building for the host system because this is often unintended.
    - Building for the host system will also cause errors in build scripts of dependencies, which try to use the standard library. The reason is that [cargo applies the `RUSTFLAGS` environment variable also to build scripts in native compilation mode](https://github.com/rust-lang/cargo/blob/d9ff5762fe2a08d329fd869c6bb6b073796666cc/src/cargo/core/compiler/build_context/target_info.rs#L394-L395), thereby overriding the sysroot.

## [v0.5.15] - 2019-07-17

- Add xb, xt, xc, and xr subcommands ([#42](https://github.com/rust-osdev/cargo-xbuild/pull/42))

## [v0.5.14] - 2019-07-11

- Don't append a `--sysroot` argument to `RUSTFLAGS` if it already contains one ([#40](https://github.com/rust-osdev/cargo-xbuild/pull/40))

## [v0.5.13] - 2019-07-09

- Add `cargo xdoc` command for invoking `cargo doc` ([#39](https://github.com/rust-osdev/cargo-xbuild/pull/39)).

## [v0.5.12] - 2019-06-13

- Fix incorrect joining of paths that caused some problems on Windows ([`a1ff0331`](https://github.com/rust-osdev/cargo-xbuild/commit/a1ff03311dd74447e8e845b4b96f2e137850027d)).

## [v0.5.11] - 2019-05-31

- Fix an issue with new `XBUILD_SYSROOT_PATH` environment variable ([#34](https://github.com/rust-osdev/cargo-xbuild/pull/34))

## [v0.5.10] - 2019-05-31

- Error when sysroot contains spaces ([#32](https://github.com/rust-osdev/cargo-xbuild/pull/32))
- Allow overriding the sysroot path through a `XBUILD_SYSROOT_PATH` environment variable ([#33](https://github.com/rust-osdev/cargo-xbuild/pull/33))

## [v0.5.9] - 2019-05-24

- Make backtraces optional through a new opt-in `backtrace` feature. This removes the dependency on `cc` by default, which has special compile-time requirements.

## [v0.5.8] - 2019-04-11

- Add `cargo xcheck`/`cargo xtest` commands for invoking `cargo check`/`cargo test`.

## [v0.5.7] - 2019-03-27

- Respect `CARGO` environment variable
- Canonicalize default specified in .cargo/config files if they end with `.json`
- Update dependencies in `Cargo.lock` file

## [v0.5.6] - 2019-03-23

- Add `cargo xrun` command for invoking `cargo run`

## [v0.5.5] - 2019-02-04
- Fix build on latest nightly: `liballoc` was updated to Rust 2018

## [v0.5.4] - 2019-01-17
- Don't fail when the `lib` or `bin` directories don't exist; only emit a warning

## [v0.5.3] - 2018-12-21
- Fix a bug introduced in 0.5.2: Backslash escaped quotes don't work on Windows, use single quotes instead

## [v0.5.2] - 2018-12-20
- Use the official approach for building the sysroot to avoid overwriting RUSTFLAGS ([#25](https://github.com/rust-osdev/cargo-xbuild/pull/25))

## [v0.5.1] - 2018-12-14
- Fix the sysroot build: the compiler_builtins crate now lives on crates.io

## [v0.5.0] - 2018-10-01
- Error instead of warn when `cargo xbuild` is executed with a stable or beta compiler.

## [v0.4.9] - 2018-08-20
- Add `cargo xclippy` command for invoking `cargo clippy`

## [v0.4.8] - 2018-07-03
- Add `cargo xrustc` command for invoking `cargo rustc`

## [v0.4.7] - 2018-06-17
- Update configuration parsing to remove special treatment for `sysroot_path`

## [v0.4.6] - 2018-06-17
- Unset RUSTFLAGS for sysroot building to support using the `--emit-link` flag.

## [v0.4.5] - 2018-06-17
- Add support for spaces in the sysroot path on Windows

## [v0.4.4] - 2018-06-01
- Add config option `sysroot_path` that defines where the sysroot should be placed. Defaults to the `target/sysroot`.

## [v0.4.3] - 2018-05-31
- Make behavior configurable through a `package.metadata.cargo-xbuild` table in the `Cargo.toml`
  - Add a `memcpy` flags that specifies whether the `compiler_builtins` crate should be built with the `mem` feature enabled. Defaults to true.

## [v0.4.2] - 2018-05-07
- Implement `--help`

## [v0.4.1] - 2018-05-06
- Fix docs link in Cargo.toml

## [v0.4.0] - 2018-05-05
- Forked as `cargo-xbuild`
- Cargo subcommand instead of standalone tool: `cargo xbuild` insetad of `xargo build`
- Remove support for all subcommands other than `build`
- Always build `core`, `compiler_builtins` and `alloc`
  - Building `std` is no longer possible
- Configuration via `Xargo.toml` is no longer possible
- Build sysroot inside `target` folder instead of global `~/.xargo`
- Paths can be passed to `--target` now: `cargo xbuild src/../my-custom-target.json`
  - The `RUST_TARGET_PATH` is not needed for paths

## [v0.3.12] - 2018-04-08

### Changed

- The `core` and `compiler_builtins` crates are built when no Xargo.toml is present.

## [v0.3.11] - 2018-03-09

### Added

- Xargo now copies the `bin` directory from the original sysroot, the host sysroot, into its own.
  This lets you use binaries shipped with the Rust toolchain, like LLD.

## [v0.3.10] - 2017-12-28

### Added

- Print a warning when the stable or beta toolchain, which are not supported, is used.

### Changed

- Set RUST_TARGET_PATH when building the sysroot. This fixes builds when using custom targets with a
  recent toolchain.

### Removed

- The lock file included in the rust-src component is no longer used when building the sysroot. This
  fixes building a sysroot that contains the compiler-builtins crate.

## [v0.3.9] - 2017-09-06

### Added

- Use Cargo.lock from the `rust-src` component if available. With this change
  the Xargo sysroot will be built using the exact same set of dependencies that
  the official sysroot distributed via rustup uses.

- The `RUSTFLAGS` variable internally used by Xargo is now printed when verbose
  (`-v`) mode is enabled.

### Changed

- Updated the documentation about building `std` with recent nightlies.

## [v0.3.8] - 2017-05-30

### Changed

- Improved the error message when `--target foo.json` is used.

## [v0.3.7] - 2017-05-13

### Changed

- Improved the error message when the `rust-src` component is missing.

## [v0.3.6] - 2017-04-07

### Fixed

- Xargo on Windows. The layout of the default / rustc sysroot recently changed
  on Windows on broke the code that copied the host part of the rustc sysroot
  into the Xargo sysroot.

## [v0.3.5] - 2017-01-20

### Fixed

- Relative paths in `dependencies.{}.path` were not being correctly handled.

## [v0.3.4] - 2017-01-18

### Added

- A `[dependencies.{}.stage]` (or `[target.{}.dependencies.{}.stage]`) entry in
  Xargo.toml. This lets you build a sysroot in "stages". This is required, for
  instance, to build the `test` crate whose dependency on the `std` crate is not
  explicitly listed in its Cargo.toml. Example:

To make `xargo test` work

``` toml
# Xargo.toml
[dependencies.std]
features = ["panic_unwind"]  # `test` depends on this `std` feature
# stage = 0  # implicit

[dependencies.test]
stage = 1
```

- Support for `[dependencies.{}.git]` or `[dependencies.{}.path]` (and their
  `target.{}.dependencies` variants) in Xargo.toml. With this feature you can
  inject foreign crates (crates which are not part of the `rust-src` component)
  into the sysroot. The main use case is replacing the `std` crate with a drop
  in replacement. Example:

Replace `std` with [`steed`](https://github.com/japaric/steed)

``` toml
[dependencies.collections]  # `steed` depends on `collections`

[dependencies.std]
git = "https://github.com/japaric/steed"
stage = 1
```

## [v0.3.3] - 2017-01-09

### Added

- Support for building a custom sysroot when compiling natively.
- Support for specifying dependencies as `[dependencies]` in Xargo.toml.

## [v0.3.2] - 2017-01-03

### Changed

- `XARGO_RUST_SRC` is now used when working with nightly Rust and it has
  precedence over the `rust-src` component.

## [v0.3.1] - 2016-12-30

### Added

- You can now specify the location where Xargo stores the sysroots via the
  `XARGO_HOME` environment variable. If unspecified, the sysroots will be stored
  in `$HOME/.xargo`

## [v0.3.0] - 2016-12-28

### Changed

- [breaking-change] By default, Xargo now only compiles the `core` crate. To
  build more crates, use a `Xargo.toml` file

- [breaking-change] Xargo will now build a sysroot for any target that's not the
  host.

- The verbose flag, `-v`, makes Xargo print all the shell commands it invokes
  to stderr.

## [v0.2.3] - 2016-12-19

### Added

- Support for the 'dev' channel. When using the dev channel, you must specify
  the path to the Rust source directory via the XARGO_RUST_SRC environment
  variable.

### Changed

- The rust-src search logic to account for recent changes in the Rust
  distribution.

## [v0.2.2] - 2016-12-12

### Changed

- Xargo will now try to build every crate "below" `std`, i.e. all its
  dependencies, in topological order. This makes Xargo robust against changes in
  the `std` facade as it no longer depends on hard coded crate names like
  `rustc_unicode`.

- Xargo won't rebuild the sysroot if the only thing that changed in Cargo.toml
  is profile.*.lto. Enabling/disabling LTO doesn't change how dependencies are
  compiled.

- Xargo won't rebuild the sysroot if the linker flags (`-C link-arg`) have
  changed. Those don't affect how the dependencies are compiled.

## [v0.2.1] - 2016-10-22

### Changed

- No weird `()` output in `xargo -V` if Xargo was built via `cargo install`
- Better formatted error messages. Mention the RUST_BACKTRACE env variable which
  is used to get backtraces on errors.

## [v0.2.0] - 2016-10-16

### Added

- Statically linked binary releases for Linux (x86 musl targets)
- `xargo -V` output now includes the commit hash and date

### Changed

- Xargo now depends on the `rust-src` component being installed. Install it with
  `rustup component add rust-src`.
- Xargo no longer depends on libcurl, libssh or libssl and, therefore, it's now
  much easier to build.
- Xargo now respects the existing rustdocflags (RUSTDOCFLAGS env var,
  build.rustdocflags, etc) when passing --sysroot to rustdoc.
- File locking logic has been revised/simplied and now lock periods are shorter

## [v0.1.14] - 2016-10-09

### Added

- `xargo -V` and `xargo --version` now report Xargo's version as well as
  Cargo's.

## [v0.1.13] - 2016-10-06

### Added

- Xargo now builds a sysroot for the new built-in `thumbv*-none-eabi*` targets
  which don't ship with a binary release of the standard crates.

## [v0.1.12] - 2016-10-04

### Added

- Xargo now supports per-target rustflags:
  `target.thumbv7em-none-eabihf.rustflags` in .cargo/config.

## [v0.1.11] - 2016-09-30

### Fixed

- `xargo clean` and other commands not associated to building stuff no longer
  trigger a sysroot rebuild.

## [v0.1.10] - 2016-09-28

### Fixed

- `xargo doc`, which wasn't working because we didn't pass --sysroot to rustdoc.
  Note that rustdoc gained support for '--sysroot' as of nightly-2016-06-28, so
  that version or newer is required to use `xargo doc`.

## [v0.1.9] - 2016-09-27

### Fixed

- "error: Invalid cross-device link (os error 18)" which occurred when
  `$CARGO_HOME` was mounted in a different device than "`$XARGO_HOME`"
  (~/.xargo). The solution was to stop using hard links to place the host
  libraries in the Xargo sysroot and instead just copy them. This is a
  regression in disk usage but this problem was coming up in common Docker usage
  patterns (-v A:B).

## [v0.1.8] - 2016-09-04

### Changed

- All the status messages are now printed to stderr instead of to stdout. Cargo
  did the same change (from stdout to stderr) a while ago. Let's follow suit.

### Fixed

- When compiling crate `foo` with Xargo, the profile section of `foo`'s
  Cargo.toml is also "taken into account" when compiling the sysroot. For
  example, if `foo` has set `panic = "abort"` for all its profiles, then the
  sysroot will also be compiled with `-C panic=abort`. Previously, this wasn't
  the case.

## [v0.1.7] - 2016-09-03

### Fixed

- The sysroot now gets rebuilt when rust-src changes.

## [v0.1.6] - 2016-08-29

### Added

- Xargo can now use the source code installed by rustup. When available, this is
  the preferred way to fetch the source code and saves network bandwidth by not
  having to fetch the source tarball.

## [v0.1.5] - 2016-08-11

### Fixed

- Xargo now works properly when called from a `rustup override`n directory.

## [v0.1.4] - 2016-08-06

### Added

- Support targets that don't support atomics (`"max-atomic-width": 0`). For
  these targets, Xargo only compiles the `core` and `rustc_unicode` crates as
  the other crates depend on atomics (e.g. `alloc::Arc`).

## [v0.1.3] - 2016-04-24

### Added

- `xargo (..) --verbose` passes `--verbose` to the `cargo` call that builds the
  sysroot.
- the sysroot now gets rebuilt when RUSTFLAGS or build.rustflags is modified.

### Fixed

- Xargo now respects the build.rustflags value set in .cargo/config.
- A bug where the hash/date file didn't get properly truncated before updating
  it leading to Xargo to *always* trigger a sysroot rebuild.

## [v0.1.2] - 2016-04-24 [YANKED]

### Added

- Xargo now uses file locking and can be executed concurrently.
- Xargo now print its current status to the console while building a sysroot.
- Xargo now reports errors to the console instead of panicking.

### Removed

- Logging via `RUST_LOG` has been removed now that Xargo prints its status to
  the console.

## v0.1.1 - 2016-04-10

- Initial release

[v0.5.24]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.23...v0.5.24
[v0.5.23]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.22...v0.5.23
[v0.5.22]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.21...v0.5.22
[v0.5.21]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.20...v0.5.21
[v0.5.20]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.19...v0.5.20
[v0.5.19]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.18...v0.5.19
[v0.5.18]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.17...v0.5.18
[v0.5.17]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.16...v0.5.17
[v0.5.16]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.15...v0.5.16
[v0.5.15]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.14...v0.5.15
[v0.5.14]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.13...v0.5.14
[v0.5.13]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.12...v0.5.13
[v0.5.12]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.11...v0.5.12
[v0.5.11]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.10...v0.5.11
[v0.5.10]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.9...v0.5.10
[v0.5.9]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.8...v0.5.9
[v0.5.8]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.7...v0.5.8
[v0.5.7]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.6...v0.5.7
[v0.5.6]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.5...v0.5.6
[v0.5.5]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.4...v0.5.5
[v0.5.4]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.3...v0.5.4
[v0.5.3]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.2...v0.5.3
[v0.5.2]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.9...v0.5.0
[v0.4.9]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.8...v0.4.9
[v0.4.8]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.7...v0.4.8
[v0.4.7]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.6...v0.4.7
[v0.4.6]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.5...v0.4.6
[v0.4.5]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.4...v0.4.5
[v0.4.4]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.3...v0.4.4
[v0.4.3]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.2...v0.4.3
[v0.4.2]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.1...v0.4.2
[v0.4.1]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.12...v0.4.0
[v0.3.12]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.11...v0.3.12
[v0.3.11]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.10...v0.3.11
[v0.3.10]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.9...v0.3.10
[v0.3.9]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.8...v0.3.9
[v0.3.8]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.7...v0.3.8
[v0.3.7]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.6...v0.3.7
[v0.3.6]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.5...v0.3.6
[v0.3.5]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.4...v0.3.5
[v0.3.4]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.3...v0.3.4
[v0.3.3]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.2.3...v0.3.0
[v0.2.3]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.14...v0.2.0
[v0.1.14]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.13...v0.1.14
[v0.1.13]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.12...v0.1.13
[v0.1.12]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.11...v0.1.12
[v0.1.11]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.10...v0.1.11
[v0.1.10]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.9...v0.1.10
[v0.1.9]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.8...v0.1.9
[v0.1.8]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.7...v0.1.8
[v0.1.7]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.6...v0.1.7
[v0.1.6]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/rust-osdev/cargo-xbuild/compare/v0.1.1...v0.1.2
