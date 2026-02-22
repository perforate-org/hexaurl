# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-22

### Breaking Changes

- `hexaurl-config`:
  - `Config` is now a compiled, size-typed config: `Config<const N: usize>`.
  - `Config` is built directly via `Config::<N>::builder().build()`.
  - `DelimiterRules` leading/trailing controls are split:
    - removed `allow_leading_trailing_hyphens` / `allow_leading_trailing_underscores`
    - added `allow_leading_hyphens`, `allow_trailing_hyphens`, `allow_leading_underscores`, `allow_trailing_underscores`

- `hexaurl-validate`:
  - `validate_with_config` now accepts `&hexaurl_config::Config<N>`.
  - `compile_config::<N>` now takes/returns `Config<N>` (compatibility helper).
  - `validate_with_compiled_config` is retained as an alias-style entry point for the compiled path.

- `hexaurl`:
  - `decode_with_config` and `decode_into_with_config` now accept `&hexaurl_config::Config<N>`.
  - `encode_with_config` now accepts `&hexaurl_config::Config<N>`.
  - `encode_with_raw_config` has been removed.

### Docs

- Updated `crates/hexaurl-config/README.md` for `Config::<N>::builder()` workflow.
- Updated `crates/hexaurl-validate/README.md` for `Config<N>` validation workflow.

### Build

- Updated Rust edition to `2024`:
  - `hexaurl-config` (`crates/hexaurl-config/Cargo.toml`) from `2018` to `2024`
  - `hexaurl-validate` (`crates/hexaurl-validate/Cargo.toml`) from `2018` to `2024`
  - `hexaurl` (`crates/hexaurl/Cargo.toml`) from `2021` to `2024`
- Updated minimum supported Rust version (`rust-version`) to `1.85.0`:
  - `hexaurl-config` from `1.31.0` to `1.85.0`
  - `hexaurl-validate` from `1.74.0` to `1.85.0`
  - `hexaurl` from `1.71.0` to `1.85.0`
