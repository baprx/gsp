# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2023-03-07

### CI/CD

- Add Cargo tests

### Miscellaneous Tasks

- Allow providing multi string project ID, (eg: proj prod)
- Don't switch project if current == target project
- Upload Cargo.lock

## [0.1.1] - 2023-03-06

### Miscellaneous Tasks

- Add license to cargo.toml

## [0.1.0] - 2023-03-06

### Documentation

- Update README.md

### Features

- Bootstrap the app with clap. Implement the refresh function.
- Add the `--refresh` argument to force a cache refresh before running the command
- Add command to generate shell completions

### Miscellaneous Tasks

- Exit without returning a project upon user abort
- Configure Github actions; Generate CHANGELOG.md

