# search-once

A command line application to search multiple websites at once.

## Installation

```console
$ cargo install search-once
```

If you use it on WSL (Windows Subsystem for Linux), you'll require the `wslview` command included in [wslu](https://wslutiliti.es/wslu/).

## Usage

```console
$ search-once --help
A tool to search multiple websites at once

Usage: search-once [OPTIONS] <QUERY>

Arguments:
  <QUERY>

Options:
  -c, --config <FILE>
  -h, --help           Print help
  -V, --version        Print version
```

## Configuration

You can customize search sites via a YAML configuration file with the following format:

```yaml
sites:
  - name: GitHub Rust repos
    url: https://github.com/search?q=language%3ARust+%s&type=repositories
  - name: Crates.io
    url: https://crates.io/search?q=%s
```

The `%s` placeholder in the URL will be replaced with your query.

The default configuration file will be automatically placed in the appropriate path based on your OS. You can find the path by running `search-once <QUERY>` without the `--config` option.

## Development

### Publishing a new version

1. Bump the version in `Cargo.toml`.
2. Create a git commit, push it, and ensure that the CI passes.
3. Create a git tag for the new version and push it, then the Crates.io workflow starts.
