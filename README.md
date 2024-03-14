# UT_Grade_Parser

[![crates.io](https://img.shields.io/crates/v/ut_grade_parser)](https://crates.io/crates/ut_grade_parser)

A grade distribution parser for the University of Texas at Austin

`ut_grade_parser` offers the most complete, up to date, and blazingly-fast course grade distribution data for the University of Texas at Austin.

## Features

-   Fetch and download grade distributions
-   Parse CSV files
-   Create a sqlite3 database

### Automation and Cross-platform Support

`ut_grade_parser` provides seamless automation capabilities, allowing you to fully automate the process of fetching, parsing, and storing grade distribution data. Moreover, it offers broad cross-platform compatibility, ensuring accessibility across various operating systems and architectures, including:

-   Apple Silicon macOS
-   Intel macOS
-   x64 Windows
-   x64 Linux

## Installation

### Cargo

```sh
cargo install ut_grade_parser
```

### From Source

To build and install from source, first checkout the tag or branch you want to install, then run

```sh
cargo install --path .
```

This will build and install `ut_grade_parser` in your `~/.cargo/bin`. Make sure that `~/.cargo/bin` is in your `$PATH` variable.

You can also download and install prebuilt binaries in [Releases](https://github.com/doprz/UT_Grade_Parser/releases)

## Usage

```
A grade distribution parser for the University of Texas at Austin

Usage: ut_grade_parser [OPTIONS] <COMMAND>

Commands:
  download  Fetch and download grade distributions
  parse     Parse CSV files
  database  Create a sqlite3 database
  all       Run all commands
  help      Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...  Turn debugging information on
  -h, --help      Print help
  -V, --version   Print version
```

## License

`UT_Grade_Parser`, `ut_grade_parser` is dual-licensed under the terms of both the MIT License and the Apache License 2.0

SPDX-License-Identifier: MIT OR Apache-2.0
