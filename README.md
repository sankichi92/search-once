# search-once

A command line application to search multiple websites at once.

## Installation

```console
$ cargo install search-once
```

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
