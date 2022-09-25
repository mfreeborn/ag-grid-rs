# Changelog
All notable changes to this project will be documented in this file. For a detailed breakdown, see the repository's [commit history](https://github.com/mfreeborn/ag-grid-rs/commits/main).

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2022-09-25
### Added
- Pad out the implementation of the `GridOptions` struct.
- Add support for filtering when using the Infinite Row Model.

## [0.2.1] - 2022-09-19
### Fixed
- Some bugs with deployment.

## [0.2.0] - 2022-09-19
### Added
- Start to pad out the implementation of the `ColumnDef` struct.
- Implement serialization of types to `wasm_bindgen::JsValue`s with the `ToJsValue` trait and associated macro.

## [0.1.0] - 2022-09-14
An early draft with some proof of concept functionality.