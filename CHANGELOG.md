# Changelog

## [0.3.2] - 2021-12-05

### Fixed

* fixed some doc links

## [0.3.1] - 2021-12-04

### Changed

* moved to `futures_lite` instead of `futures`

## [0.3.0] - 2021-12-04

### Changed

* reimplemented `Reader` and `Writer` using a modern async api

## [0.2.0] - 2019-11-04

### Added

* added `offset` method to `Parser` and `Reader` to get the amount of time to
  offset frame times by in order to pretend that the frames started at time 0

### Fixed

* fix panic when parsing a frame with a large timestamp

## [0.1.0] - 2019-10-27

### Added

* Initial release
