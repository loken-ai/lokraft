# Server routes

Every request of lokraft goes to the address of the active connection, with the routes below appended.
The key of that connection goes with each one as `Authorization: Bearer <key>`.

lokraft reads no version of the server.
A feature whose route the server lacks fails with the status the server returns, often 404.

<!-- site: widget table id=lokraft-server-routes -->
| Feature | Route | loken 0.4.2 | Ollama |
|---|---|---|---|
| Model list | `GET /api/tags` | yes | yes |
| Loaded models | `GET /api/models/loaded` | yes | no, the list stays empty |
| LoRA adapters | `GET /v1/loras` | yes | no |
| Pull | `POST /api/pull` | yes | yes |
| Delete | `DELETE /api/delete` | yes | yes |
| Load, unload | `POST /api/generate`, empty prompt, `keep_alive` of `5m` or `0` | yes | yes |
| Load on port 11434 | `POST /api/chat`, empty system message | yes | yes |
| Chat with a chosen model | `POST /api/chat`, streamed | yes | yes |
| Enhance | `POST /api/chat` | yes | yes |
| Auto | `POST /conversation`, `smart_routing: true` | yes | no |
| Image | `POST /v1/images/generations`, streamed | yes, `image` build feature | no |
| Image Edit | `POST /v1/images/edits` | yes, `image` | no |
| Music, SFX | `POST /v1/audio/generations`, streamed | yes, `audio` | no |
| MIDI | `POST /v1/audio/generations` | yes, `midi` | no |
| Speech | `POST /v1/audio/speech`, streamed when the server can | yes, `audio` | no |
| Voices | `GET /v1/audio/voices` | yes, `audio` | no |
| Transcribe | `POST /v1/audio/transcriptions` or `/v1/audio/translations` | yes, `audio` | no |
| Separate | `POST /v1/audio/separate` | yes, `audio` | no |
| Video | `POST /v1/video/generations`, streamed | yes, `video` | no |
| Video estimate | `POST /api/video/plan` | yes | no |
| Cancel a render | `POST /v1/renders/{id}/cancel` | yes | no |
<!-- site: end -->

A model is loaded through `/api/chat` instead of `/api/generate` when the address holds `:11434`, the Ollama port.

## Timeouts

<!-- site: widget table id=lokraft-server-timeouts -->
| Request | lokraft waits up to |
|---|---|
| A streamed render, Image, Music, SFX, Speech or Video | 180 seconds without data, however long the render |
| Model list, loaded models | 20 seconds |
| Load | 600 seconds |
| Delete | 60 seconds |
| Pull from Models | 6 hours |
| Pull from the Terminal | 600 seconds |
| Chat with a chosen model | 6 hours |
| Enhance | 300 seconds |
| Auto | 900 seconds |
| Image Edit, MIDI | 900 seconds |
| Transcribe | 600 seconds |
| Separate | 6 hours |
| LoRA adapters | 15 seconds |
| Video estimate | 10 seconds |
| Voices | 5 seconds |
| Any other request | 30 seconds |
<!-- site: end -->

A request that waits too long fails with `request timed out`.
