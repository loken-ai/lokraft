# Settings

Settings, `Ctrl+6`, holds the connections, the theme and the form of a loken `config.toml`.
A change saves itself to `config.json`, except the server form, which waits for `Save config.toml`.

## Connections

The `CONNECTIONS` panel lists the servers lokraft can talk to.
The active one serves the chat, the Studio and the Models view.

<!-- site: widget table id=lokraft-settings-connection -->
| Control | Effect |
|---|---|
| lamp | on the active connection, in the color of its state, off when no server answers |
| `Active` | marks the connection in use |
| `Use` | makes that connection the active one |
| `Edit` | loads the connection into the form under the list |
| `Delete` | removes the connection, never the last one |
<!-- site: end -->

A row shows the name, the address and `key set` when the connection holds an API key.
A new address or key on the active connection reloads the model list.

The form under the list adds a connection, or edits the one picked with `Edit`.

| Field | Effect | Default |
|---|---|---|
| `Name` | the name of the connection, required and unique | `Server`, then `Server 2` and so on |
| `Address` | the address of every request. A bare `host:port` gets `http://`, and trailing slashes go. No space allowed | empty, the hint reads `http://localhost:11435` |
| `API key` | sent on every request as `Authorization: Bearer <key>`. Empty sends nothing | empty |

`Add connection` appends the connection.
While a connection is edited, `Save` replaces it and `Cancel` leaves the form.  
A refused form keeps what was typed and says why under the fields.

A first run holds 1 connection, `Local`, at `http://localhost:11435`, with no key.
The top bar switches the connection too, see [Connect to a server](../guides/connect.md).

## Appearance

| Field | Values | Default |
|---|---|---|
| `Theme` | `Dark`, `Light` | `Dark` |

The palette button of the top bar switches the theme too.

## Generation parameters

The `Parameters` button of the chat header opens the parameters of a chat request.
Each field starts empty, and an empty field is not sent, so the server applies the defaults of the model.

| Field | Request option | Accepts |
|---|---|---|
| `TEMPERATURE` | `temperature` | a number, 0 or more |
| `TOP P` | `top_p` | a number above 0 and up to 1 |
| `TOP K` | `top_k` | a whole number |
| `REPEAT PENALTY` | `repeat_penalty` | a number above 0 |
| `CONTEXT` | `num_ctx` | a whole number of tokens above 0 |
| `MAX TOKENS` | `num_predict` | a whole number, -1 for no limit |
| `SEED` | `seed` | a whole number, empty for a random seed |
| `STOP` | `stop` | a list split on commas |

A value that does not parse is named in red under the fields and leaves the parameter as it was.
`Reset all` clears every field.  
The button reads `Parameters (changed)` while at least 1 field is set.
Auto sends none of these.

## Server configuration

The panels below show only when lokraft reads a `config.toml` from its working directory or from beside its executable, see [Edit the server configuration](../guides/server-config.md).
A field shows the value of the file, or the value of the Shown column when the key is absent.

<!-- site: widget table id=lokraft-settings-server -->
| Panel | Field | Key | Accepts | Shown |
|---|---|---|---|---|
| `MODEL DIRECTORIES` | `OLLAMA MODELS` | `ollama_models_dir` | a path, empty for none | empty |
| `MODEL DIRECTORIES` | `HUGGINGFACE MODELS` | `huggingface_models_dir` | a path, empty for none | empty |
| `SERVER BIND` | `HOST` | `[server] host` | an address | `127.0.0.1` |
| `SERVER BIND` | `PORT` | `[server] port` | 0 to 65535 | 11435 |
| `INFERENCE` | `MODEL ID` | `[inference] model_id` | a model name | |
| `INFERENCE` | `MAX TOKENS` | `[inference] max_tokens` | an integer from 0 | 2048 |
| `INFERENCE` | `CONTEXT LENGTH` | `[inference] context_length` | an integer from 0 | 4096 |
| `INFERENCE` | `TEMPERATURE` | `[inference] temperature` | 0 to 2 | 0.7 |
| `INFERENCE` | `TOP P` | `[inference] top_p` | 0 to 1 | 0.9 |
| `INFERENCE` | `TOP K` | `[inference] top_k` | an integer from 0 | 50 |
| `INFERENCE` | `SEED` | `[inference] seed` | an integer from 0 | 42 |
| `GPU AND DEVICE` | `DEVICE INDEX` | `[inference] device_index` | an integer, empty for automatic | empty |
| `GPU AND DEVICE` | `GPU MEMORY %` | `[inference] max_gpu_memory_fraction` | 0 to 1 | 0.9 |
| `GPU AND DEVICE` | `FORCE GPU LAYERS` | `[inference] force_gpu_layers` | an integer, empty for automatic | empty |
| `GPU AND DEVICE` | `CPU THREADS` | `[inference] cpu_threads` | an integer, 0 for all cores | 0 |
| `GPU AND DEVICE` | `Use quantized GPU` | `[inference] use_quantized_gpu` | ticked or not | ticked |
<!-- site: end -->

`Browse` beside a model directory opens a folder picker.  
`Save config.toml` writes every field of the form, the empty ones left out.
The keys of the file that the form does not show are kept.  
[What loken reads](../guides/server-config.md#what-loken-reads) lists the keys loken 0.4.2 ignores.

## Actions

| Button | Effect |
|---|---|
| `Save config.toml` | checks the form and writes `config.toml` in the working directory |
| `Reload from disk` | reads `config.toml` again and drops the unsaved edits |
| `Reset GUI defaults` | after a confirmation, saves `config.json` as `config.json.before_reset.<timestamp>` and starts again from the defaults |

The 3 buttons sit in the `ACTIONS` panel, which shows only with a `config.toml`.
A reset drops the connections with their keys, the generation parameters, the chosen model, the theme and the window preferences.
It keeps the model files of the server.
