# Command line

```
lokraft [OPTIONS]
```

lokraft opens its window with no option.

## Options

<!-- site: widget table id=lokraft-cli-options -->
| Option | Value | Effect |
|---|---|---|
| `-s`, `--server` | an address, such as `http://192.0.2.10:11435` | replaces the address of the active connection for this run, and is never saved |
| `-m`, `--model` | a model name, such as `llama3.2:latest` | loads the model at start and makes it the chosen model |
| `-h`, `--help` | | prints the help and exits |
| `-V`, `--version` | | prints `lokraft` and its version, and exits |
<!-- site: end -->

`--server` takes the address as written.
The `Address` field of a connection in Settings adds `http://` to a bare `host:port`, the option does not.
Picking a connection in the top bar or in Settings ends the replacement.

Without `--model`, lokraft loads the model chosen in the last session, when there is one.  
A load unloads every other loaded model first.

## Environment

| Variable | Effect | Default |
|---|---|---|
| `RUST_LOG` | the log filter, in the syntax of `tracing-subscriber` | `lokraft=info,reqwest=info` |

```sh
RUST_LOG=lokraft=debug lokraft
```

The lines go to the Logs view and to standard error.

## Files

| File | Holds |
|---|---|
| `config.json` | the settings of lokraft, see [Configuration file](config-file.md) |
| `crash.log` | 1 report per crash, appended, beside `config.json` |
| `config.toml` | a loken configuration, read from the working directory or beside the executable, written to the working directory, see [Edit the server configuration](../guides/server-config.md) |

A crash also opens a dialog, `lokraft stopped unexpectedly`, that names `crash.log`.

## Window

A first run opens the window maximised.
lokraft then restores the size and the maximised state of the last session.  
The window never gets smaller than 600 by 400 points.
The Chat and Settings views narrow with the window, the Studio stacks its form above its result, and the connection list and the generation parameters scroll when the window is short.
