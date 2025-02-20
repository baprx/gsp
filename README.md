# gsp

A CLI tool to easily switch between Google gcloud projects inspired by the late [gsp](https://github.com/palm93/gsp).

This is more a pretext to play with Rust than anything else but I use this tool a lot and I will continue to maintain it.

## Usage

```bash
❯ gsp help
Simple CLI to switch between gcloud projects.


Usage: gsp [OPTIONS] [PROJECT] [COMMAND]

Commands:
  current               Print the project which currently used
  list                  List the available projects
  refresh               Refresh the list of available projects
  generate-completions  Generate shell completions
  help                  Print this message or the help of the given subcommand(s)

Arguments:
  [PROJECT]  Part or entire project ID

Options:
      --log-level <LOG_LEVEL>  Set the log level [default: INFO] [possible values: TRACE, DEBUG, INFO, WARN, ERROR]
      --refresh                Force a cache refresh then run the requested command
  -h, --help                   Print help
  -V, --version                Print version
```

## Cache

A cache file is created in order to avoid refreshing the project list every time. The file is stored at `~/.cache/gsp/projects.json`

The cache contains the list of available projects and can be refeshed by using the `gsp refresh` command or by adding the `--refresh` argument to any other command:

```bash
❯ gsp list --refresh
[INFO] The cache was successfully refreshed.
+----------------------------------+----------------+--------------------------------+
| Project ID                       | Project number | Project name                   |
+==================================+================+================================+
| dummy-example-project            | 902838561285   | Example Project                |
+----------------------------------+----------------+--------------------------------+
```

## Generate completions

```bash
source <(gsp generate-completions --shell zsh)
```
