# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Breaking Changes

- `hexaurl-config`:
  - `Config` is now a compiled, size-typed config: `Config<const N: usize>`.
  - `Config` is built directly via `Config::<N>::builder().build()`.

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
