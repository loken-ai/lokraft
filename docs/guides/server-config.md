# Edit the server configuration

Settings edits a loken `config.toml` in a form: the model directories, the address the server binds, the default model and a set of inference keys.

## Before you start

<!-- site: widget checklist id=lokraft-server-config-prereq -->
- [ ] lokraft runs on the machine of the server.
- [ ] A `config.toml` sits in the working directory of lokraft or beside its executable.
- [ ] The file has an `[inference]` table with a `model_id`, and each model directory is 1 path, not a list.
<!-- site: end -->

lokraft shows the server panels only when it reads such a file.
Without one, Settings stops after `APPEARANCE`.

## Smallest configuration

```toml
[server]
host = "127.0.0.1"
port = 11435

[inference]
model_id = "qwen3:0.6b"
```

## Run it

<!-- site: widget steps id=lokraft-server-config-steps -->
1. Start lokraft from the directory that holds `config.toml`.

   ```sh
   cd /path/to/loken
   /path/to/lokraft
   ```

2. Open Settings with `Ctrl+6` and scroll to `MODEL DIRECTORIES`.
3. Change the fields.
   `Browse` opens a folder picker for a model directory.
4. Press `Save config.toml`.
5. Restart the server.
<!-- site: end -->

## Expected result

lokraft writes `config.toml` in its working directory and reloads the model list.  
The keys the form does not show are kept as they were.
`Reload from disk` throws the unsaved edits away.

## What loken reads

| Panel | Field | Key | Read by loken 0.4.2 |
|---|---|---|---|
| `MODEL DIRECTORIES` | `OLLAMA MODELS` | `ollama_models_dir` | yes |
| `MODEL DIRECTORIES` | `HUGGINGFACE MODELS` | `huggingface_models_dir` | yes |
| `SERVER BIND` | `HOST`, `PORT` | `[server] host`, `port` | yes |
| `INFERENCE` | `MODEL ID` | `[inference] model_id` | yes |
| `INFERENCE` | `MAX TOKENS`, `CONTEXT LENGTH`, `TEMPERATURE`, `TOP P`, `TOP K`, `SEED` | `[inference] max_tokens`, `context_length`, `temperature`, `top_p`, `top_k`, `seed` | no |
| `GPU AND DEVICE` | `DEVICE INDEX`, `GPU MEMORY %`, `FORCE GPU LAYERS`, `CPU THREADS`, `Use quantized GPU` | `[inference] device_index`, `max_gpu_memory_fraction`, `force_gpu_layers`, `cpu_threads`, `use_quantized_gpu` | no |

loken 0.4.2 logs ``config.toml [inference] `<key>` is no longer read`` at start for each key it ignores, and decides those values from the model and the machine.
Every field and its range is in [Settings](../reference/settings.md#server-configuration).

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| No `MODEL DIRECTORIES` panel | No `config.toml` in the working directory or beside lokraft, or the file does not parse. | Start lokraft from the directory of the file. Check `[inference] model_id` and the model directories. |
| `Invalid server port (must be 0-65535)` | `PORT` is not a port number. | Type a number up to 65535. |
| `temperature out of range (must be 0.0..=2.0)` | A value is outside its range. | Fix the field the message names. |
| `Failed to save config.toml: ...` | The working directory is read-only, or the disk is full. | Start lokraft from a writable directory. |
