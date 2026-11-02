# Changelog

All notable changes to lokraft. The format follows [Keep a Changelog](https://keepachangelog.com/1.1.0/).

## [Unreleased]

## [0.2.0] - 2026-11-02

### Added

- studio: cancel an image, MIDI, edit, split or transcription render before it ends.
- ui: confirm every copy with a toast.
- chat: edit the generation parameters from a panel, each sent only once set.
- connections: replace the API profiles with a list of connections, switched from the top bar.
- ui: show the section shortcuts in the sidebar tooltips.
- studio: start a render with Ctrl+Enter.

### Changed

- ui: redraw the interface flat, with a violet accent and IBM Plex.
- ui: ask before clearing the chat, the log and the earlier results, like for a delete.
- models: show the models as a table.
- studio: put the form on the left with Generate in view, and the result on the right.
- chat: write the conversation on a reading column, with the actions under the text.
- build: update egui to 0.37 and require Rust 1.95 or later.

### Fixed

- settings: fit the fields to a narrow window.
- chat: send the whole conversation to a text model.
- studio: show the video cost estimate.
- terminal: report an unload once the server has answered.
- models: count with the kind filter, pull quick installs from Ollama, explain Delete and Load.
- logs: describe the tab as the app's own log.
- chat: let Auto send without a model picked.
- terminal: name every command the same way in the header and help.
- chat: name Auto where the model is shown.

### Removed

- chat: remove the layer mode toggle, which the server ignored.

## [0.1.0] - 2026-10-24

### Added

- Chat: streaming, image attachments for vision models, and speech input.
- Studio: image, editing, audio, speech, video and transcription, each kind keeping its own parameters.
- Viewer: fullscreen and zoom, and a strip of earlier generations that survives a new run.
- Views: hardware and server, per-device topology, what is loaded where, and the server log.
- Audio: in-process output (native-audio, on by default), so the transport is the same on every platform.
- Video: decodes as it plays, keeping packets and producing pictures around the playhead.
- Studio: a cost estimate shown next to the button that starts a render.
- Theme: light and dark, muted text pinned against the WCAG AA contrast floor on both, by test.
- Chat: Up and Down at the input edges recall the previous and next prompt; Ctrl+Up and Ctrl+Down from anywhere.
- Models: the picker opens in a popup of its own.
- Studio: the status names which node renders.
- Docs: the chat's Auto mode is documented.
- CI: a workflow with a dependency audit.

### Changed

- Project: rename atelier to lokraft.
- Theme: one visual grammar, two skins on three surface depths, one accent, five type sizes.
- Theme: chat as rows in a well, Studio two columns, terminal a screen; skin colours pinned by tests.
- Docs: screenshots for the model list and the server log.
- Docs: examples use documentation addresses and the current project name.
- Docs: each screenshot sits with the section that explains it.

### Fixed

- Audio: a failed output is closed and reopened, instead of logging every period and never playing again.
- Build: clean build and lint without audio output.
- Audio: PCM samples are read as fixed-size chunks.
