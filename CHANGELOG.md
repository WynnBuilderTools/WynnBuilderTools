# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 04/10/2024
### Added
- Automatic fetching of the items' database from the Wynncraft API!
- Automatic creation of the items.json file if missing, with all the Hppeng important stats!

### Fixed
- Clippy formatting
- sqlx vulnerable version

## [0.5.2] - TBA
### Added
- More informational println calls to inform when the db folder and data.db file are being created.
- CI/CD Integration with GitHub actions

### Fixed
- Formatting reported by Clippy and cargo fmt
- Updated README.md for new use steps on Windows

### Removed
- A bunch of derive Hash that weren't needed

## [0.5.1] - TBA
### Fixed
- Bug in search_item where min and max level filters weren't being used.

## [0.5.0] - TBA
### Changed
- The db folder and the database file are now created automatically and prepared with the migrations without any sqlx command.

## [0.4.2] - TBA
### Changed
- Adjusted search_item to allow for multiple arguments when sorting, allowing for finer filtering.

### Removed
- The db will no longer be included in the releases. Users are expected to use sqlx from cargo to make the database themselves.

## [0.4.1] - TBA
### Added
- Exp Bonus filtering with the search_item binary.
- Minimum and maximum level filtering of items with the search_item binary.
- Exp bonus fetching and calculations to generate builds with high exp bonus.

## [0.4.0] - TBA
### Added
- Implemented Remaining Time as a moving average of the last 10 speed values calculated.
- Added the number of builds that are still to calculate.
- Partial documentation for some crucial methods.

### Changed
- For any of these binaries to run, the config and db folders must be present, as well as the up-to-date items.json and config.toml.

### Contributors
- @IlChitarrista made their first contribution.

[Unreleased]: https://github.com/TYTheBeast/WynnBuilderTools-Rekindled/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/TYTheBeast/WynnBuilderTools-Rekindled/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/TYTheBeast/WynnBuilderTools-Rekindled/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/TYTheBeast/WynnBuilderTools-Rekindled/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/TYTheBeast/WynnBuilderTools-Rekindled/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/TYTheBeast/WynnBuilderTools-Rekindled/releases/tag/v0.4.0