# Configuration file

lokraft keeps its settings in `config.json`.
Settings writes the file at each change, and lokraft writes it again when the window closes.

## Location

<!-- site: widget tabs group=system id=lokraft-config-file-location -->
### Linux

`~/.config/lokraft/config.json`, or under `$XDG_CONFIG_HOME` when it is set.

### macOS

`~/Library/Application Support/com.loken.lokraft/config.json`

### Windows

`%APPDATA%\loken\lokraft\config\config.json`
<!-- site: end -->

`crash.log` sits in the same directory.

## Keys

<!-- site: widget table id=lokraft-config-file-keys -->
| Key | Holds | Default |
|---|---|---|
| `connections` | the servers, each with `name`, `url` and `api_key`, the key in clear text | `Local`, `http://localhost:11435`, no key |
| `active_connection` | the name of the connection in use | `"Local"` |
| `dark_theme` | the dark theme when true | `true` |
| `selected_model` | the chosen model, loaded at start | `null` |
| `selected_model_source` | `"ollama"` or `"huggingface"`, the store of the chosen model | `null` |
| `window_width`, `window_height` | the size of the window | `1000.0`, `750.0` |
| `window_maximized` | the window opens maximised when true | `true` |
| `generation` | the generation parameters that were changed, see [Settings](settings.md#generation-parameters) | all empty |
| `chat_smart_auto` | Auto in the chat when true | `false` |
| `sidebar_expanded` | the sidebar shows its labels when true | `true` |
| `media` | the Studio: the kind, the prompt, the seed and the settings of each kind | `null` |
<!-- site: end -->

A key missing from the file takes its default.
A file written by an older build therefore loads whole.

A file that holds no `connections` is read as an older file.
Its `server_url` and `api_key` become the connection `Default`, and its active profile becomes the `generation` parameters that differ from the defaults of its type.
lokraft then saves the file in the new form.

```json
{
  "connections": [
    { "name": "Local", "url": "http://localhost:11435", "api_key": null },
    { "name": "Workstation", "url": "http://192.0.2.10:11435", "api_key": null }
  ],
  "active_connection": "Workstation",
  "dark_theme": true,
  "selected_model": "llama3.2:latest"
}
```

The address given with `--server` is never written to the file.

## Backups

| File | Written when |
|---|---|
| `config.json.broken.<timestamp>` | `config.json` exists and does not parse. lokraft starts on the defaults and logs `config.json parse failed (...); broken file backed up to ...` |
| `config.json.before_reset.<timestamp>` | `Reset GUI defaults` is confirmed |

An empty `config.json` gets no backup, and lokraft logs `config.json was empty (likely a crashed write); using defaults`.  
Copy a backup over `config.json` to restore it.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `config.save failed: ...` in the log | The directory is read-only or the disk is full. | Free the disk, or fix the rights of the directory. |
| The settings came back to the defaults | `config.json` did not parse. | Find `config.json.broken.<timestamp>`, fix it and copy it back. |
