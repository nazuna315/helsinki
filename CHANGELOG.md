# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Apply custom color styles to `config` subcommand help when profile is given without key
- Show correct binary name (`helsinki config`) in usage line of `config` subcommand help

## [0.1.1] - 2026-03-20

### Added
- Colored help output with custom clap styles
- GitHub Actions for CI (fmt, clippy, test, build), security audit, and publish

### Fixed
- Show help instead of error when `config` subcommand is missing required key argument

## [0.1.0] - 2026-03-19

### Added
- `config` subcommand to set and get profile values using git config keys
- `set` subcommand to apply a profile to the current repository via `git config --local`
- Interactive profile selection with `dialoguer` when `set` is run without arguments
- `list` subcommand to display all registered profiles
- `remove` subcommand to delete a profile
- `global` subcommand to set `user.useConfigOnly = true` globally
- Colored output for error and success messages
- Git installation and repository validation before executing git commands
- Cross-platform config path with `XDG_CONFIG_HOME` support
