# Install

lokraft builds from source into 1 binary, `lokraft`.  
lokraft publishes no prebuilt archive and no package.

## Before you start

<!-- site: widget checklist id=lokraft-install-prereq -->
- [ ] A Rust toolchain with `cargo`, version 1.95 or later.
- [ ] The repository checked out from <https://github.com/loken-ai/lokraft>.
- [ ] On Linux, the system packages of your platform below.
- [ ] A loken server that lokraft can reach over HTTP, on this machine or another one.
<!-- site: end -->

## Build

<!-- site: widget tabs group=system id=lokraft-install-system -->
### Linux

```sh
sudo apt-get install -y libasound2-dev libxkbcommon-dev libwayland-dev
cargo build --release
```

`libasound2-dev` holds the ALSA headers that the audio output links against.
The other 2 packages are the ones the continuous integration installs for the window libraries.  
The window opens under Wayland or X11.

### macOS

```sh
cargo build --release
```

The audio output uses CoreAudio and needs no system package.

### Windows

```sh
cargo build --release
```

The audio output uses WASAPI and needs no system package.
A release build opens no console window beside the application.
<!-- site: end -->

The binary is `target/release/lokraft`, `lokraft.exe` on Windows.

## Build without audio output

```sh
cargo build --release --no-default-features
```

The build without audio output needs no ALSA headers on Linux.
The audio player then reports `built without the native-audio feature` instead of playing.

## Verify

<!-- site: widget replay id=lokraft-install-replay -->
```
$ target/release/lokraft --version
lokraft 0.1.0
$ target/release/lokraft --help
Lokraft - chat and media generation against a local inference server

Usage: lokraft [OPTIONS]

Options:
  -m, --model <MODEL>    Model to load on startup
  -s, --server <SERVER>  Server URL to connect to (default: localhost:11435)
  -h, --help             Print help
  -V, --version          Print version
```
<!-- site: end -->

`lokraft` with no option opens the window.
The lamp of the top bar turns green once the server answers.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| The build fails in `alsa-sys` | The ALSA headers are missing on Linux. | Install `libasound2-dev`, or build without audio output. |
| The window opens and the lamp of the top bar stays off | No server answers at the address of the active connection. | See [First run](first-run.md#troubleshooting). |
