# Contributing

## Build and check

```sh
cargo build --release
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo doc --no-deps
cargo check --no-default-features --all-targets
cargo deny check
```

The continuous integration runs these on Linux, with `RUSTFLAGS=-D warnings` and `RUSTDOCFLAGS=-D warnings`.
It also checks the build with Rust 1.95.0, the minimum supported version.  
A last job refuses a `path` dependency in `Cargo.toml`, so the crate builds on its own.

Linux needs these packages for the build and the tests:

```sh
sudo apt-get install -y libxkbcommon-dev libwayland-dev libasound2-dev
```

## Features

| Feature | Default | Effect |
|---|---|---|
| `native-audio` | on | in-process audio output through `cpal`. Needs the ALSA headers on Linux |

Without `native-audio`, the player reports `built without the native-audio feature`.
The build without it is checked by the integration.

## Screenshots

The 4 pictures of `docs/img` are drawn by the test suite, from fixed state in `src/screenshots.rs`.
They show the current layout, and no real address, path or prompt.

```sh
cargo test --release screenshots -- --ignored
```

The run writes the dark renders to `docs/img/` and the light ones to `target/light/`.
It needs a graphics adapter, so the integration skips it.

## Layout

| Path | Holds |
|---|---|
| `src/main.rs` | the entry point, the crash reporter, the window options |
| `src/app.rs` | the application: the command line, the views, the requests and their results |
| `src/state.rs` | the state of each view, the Studio kinds and their parameters |
| `src/config.rs` | `config.json`: the settings, the connections, the load and the backups |
| `src/config_editor.rs` | the loken `config.toml` form: read, check, write |
| `src/api/client.rs` | the HTTP client: every route, the key, the timeouts |
| `src/api/types.rs` | the request and answer types |
| `src/chat_tab.rs` | the Chat view |
| `src/media_tab.rs` | the Studio view |
| `src/ui/models.rs` | the Models view |
| `src/ui/settings.rs`, `src/settings.rs`, `src/connection.rs` | the Settings view, the connection form and the connection type |
| `src/generation.rs` | the generation parameters of a chat request |
| `src/cli_tab.rs` | the Terminal view and its commands |
| `src/server_log_tab.rs`, `src/log_buffer.rs` | the Logs view and the buffer of log lines |
| `src/ui/layout.rs` | the top bar and the sidebar |
| `src/ui/widgets.rs`, `src/ui/surface.rs`, `src/theme.rs` | the shared widgets, the painted surfaces, the 2 palettes |
| `src/modality.rs` | the modality of a model from its name, the chat limits |
| `src/audio_engine.rs`, `src/audio_playback.rs` | the in-process audio output and its transport |
| `src/video_engine.rs` | the in-process H.264 decoder and player |
| `src/image_viewer.rs` | the fullscreen picture viewer |
| `src/dialog.rs` | the file dialogs on a worker thread |
| `src/screenshots.rs` | the screenshot tests |
| `docs/` | the user documentation, built into the site |

## Where it is careful

- **Playback is in-process.**
  lokraft decodes the samples and hands them to the operating system's audio device.
  Pause, seek and position then mean the same on every platform.
- **Video decodes as it plays.**
  The player keeps the packets and decodes only the pictures around the playhead.
- **The estimate comes before the render.**
  The Video kind asks the server what the settings will cost before `Generate` is pressed.
  lokraft asks `POST /api/video/plan`.
- **Contrast is a test.**
  The tests pin the muted text above the WCAG AA ratio, 4.5 to 1, on the background and the panels, in both themes.
- **An older `config.json` loads whole.**
  Every persisted struct takes `serde(default)`, and a test loads a file written by an older build.

## Documentation

The pages of `docs/` follow the rules of the family's documentation: no source path in a user page, every value read from the code.
[dkt](https://github.com/cbonte/dkt) checks their links and their widgets:

```sh
dkt lint --enable site-annotation docs
```
