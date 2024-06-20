# Changelog

All notable changes to this project will be documented in this file.

## [0.1.5] - 2024-06-20

### Bug Fixes

- Update rust crate clap to 4.4.8
- Update rust crate serde to 1.0.193
- Update rust crate clap to 4.4.9
- Update rust crate clap to 4.4.10
- Update rust crate clap to 4.4.11
- Update rust crate clap_complete to 4.4.5
- Update rust crate clap to 4.4.12
- Update rust crate serde_json to 1.0.109
- Update rust crate serde to 1.0.194
- Update rust crate serde_json to 1.0.110
- Update rust crate clap_complete to 4.4.6
- Update rust crate serde_json to 1.0.111
- Update rust crate clap to 4.4.13
- Update rust crate serde to 1.0.195
- Update rust crate clap to 4.4.14
- Update rust crate clap to 4.4.15
- Update rust crate clap to 4.4.16
- Update rust crate clap to 4.4.17
- Update rust crate clap_complete to 4.4.7
- Update rust crate clap to 4.4.18
- Update rust crate clap_complete to 4.4.8
- Update rust crate clap_complete to 4.4.9
- Update rust crate serde to 1.0.196
- Update rust crate serde_json to 1.0.112
- Update rust crate serde_json to 1.0.113
- Update rust crate clap_complete to 4.4.10
- Update rust crate clap to 4.5.0
- Update rust crate clap_complete to 4.5.0
- Update rust crate clap to 4.5.1
- Update rust crate clap_complete to 4.5.1
- Update rust crate serde to 1.0.197
- Update rust crate serde_json to 1.0.114
- Update rust crate log to 0.4.21
- Update rust crate simplelog to 0.12.2
- Update rust crate clap to 4.5.2
- Update rust crate rust-ini to 0.21.0
- Update rust crate clap to 4.5.3
- Update rust crate clap to 4.5.4
- Update rust crate serde_json to 1.0.115
- Update rust crate clap_complete to 4.5.2
- Update rust crate serde_json to 1.0.116
- Update rust crate serde to 1.0.198
- Update rust crate serde to 1.0.199
- Update rust crate serde to 1.0.200
- Update rust crate serde to v1.0.201
- Update rust crate serde_json to v1.0.117
- Update rust crate serde to v1.0.202
- Update rust crate serde to v1.0.203
- Update rust crate clap to v4.5.6
- Update rust crate clap_complete to v4.5.4
- Update rust crate clap_complete to v4.5.5
- Update rust crate clap to v4.5.7
- Update rust crate clap_complete to v4.5.6

### Miscellaneous Tasks

- Update actions/cache action to v4

## [0.1.4] - 2023-11-07

### Bug Fixes

- Update rust crate serde to 1.0.165
- Update rust crate serde to 1.0.166
- Update rust crate serde_json to 1.0.102
- Update rust crate serde to 1.0.171
- Update rust crate serde_json to 1.0.103
- Update rust crate clap to 4.3.14
- Update rust crate clap to 4.3.15
- Update rust crate clap to 4.3.16
- Update rust crate clap to 4.3.17
- Update rust crate serde to 1.0.173
- Update rust crate serde to 1.0.174
- Update rust crate clap to 4.3.19
- Update rust crate serde to 1.0.175
- Update rust crate serde to 1.0.176
- Update rust crate serde_json to 1.0.104
- Update rust crate serde to 1.0.178
- Update rust crate serde to 1.0.179
- Update rust crate serde to 1.0.180
- Update rust crate serde to 1.0.181
- Update rust crate serde to 1.0.182
- Update rust crate serde to 1.0.183
- Update rust crate clap to 4.3.21
- Update rust crate log to 0.4.20
- Update rust crate serde_json to 1.0.105
- Update rust crate clap to 4.3.22
- Update rust crate clap to 4.3.23
- Update rust crate serde to 1.0.185
- Update rust crate clap to 4.3.24
- Update rust crate serde to 1.0.186
- Update rust crate clap to 4.4.0
- Update rust crate clap_complete to 4.4.0
- Update rust crate serde to 1.0.187
- Update rust crate serde to 1.0.188
- Update rust crate clap to 4.4.1
- Update rust crate clap to 4.4.2
- Update rust crate clap_complete to 4.4.1
- Update rust crate serde_json to 1.0.106
- Update rust crate clap to 4.4.3
- Update rust crate serde_json to 1.0.107
- Update rust crate clap to 4.4.4
- Update rust crate clap to 4.4.5
- Update rust crate clap_complete to 4.4.2
- Update rust crate clap to 4.4.6
- Update rust crate clap_complete to 4.4.3
- Apply Clippy recommendation clippy::unwrap-or-default
- Update rust crate serde to 1.0.192
- Update rust crate rust-ini to 0.20.0
- Update rust crate clap to 4.4.7
- Update rust crate clap_complete to 4.4.4
- Update rust crate serde_json to 1.0.108

### Miscellaneous Tasks

- Update actions/checkout action to v4

## [0.1.3] - 2023-07-03

### Bug Fixes

- Update rust crate log to 0.4.19
- Update rust crate serde to 1.0.164
- Update rust crate serde_json to 1.0.99
- Update rust crate clap to 4.3.10
- Update rust crate clap_complete to 4.3.1
- Update rust crate rust-ini to 0.19.0

### Miscellaneous Tasks

- Update changelog

## [0.1.2] - 2023-03-07

### CI/CD

- Add Cargo tests

### Miscellaneous Tasks

- Allow providing multi string project ID, (eg: proj prod)
- Don't switch project if current == target project
- Upload Cargo.lock
- Bump version 0.1.2

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

