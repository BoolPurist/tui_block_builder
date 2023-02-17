# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased] 

### Added

- Reexports of used tui-rs modules.
- example to show increasing block number as counter.
- API to generate line of number blocks by a given unsigned integer.
- builder for building a line of blocks, numbers, white space and : .
- specific API for tui rs to build blocks for rendering big numbers.
- example of showcasing these the rendering of numbers.

## [0.1.0] - 2023.02.16

### Added

- API to generate block based grids aka 2 dimensional arrays.
- API to build such grid more conveniently.
- API to generate 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, : and white spaces. 
  Those builder can be scaled in block size

[unreleased]: https://github.com/BoolPurist/daily_ruster_man/compare/v0.1.0...HEAD
[0.1.0]:https://github.com/BoolPurist/tui_block_builder/releases/tag/v0.1.0
