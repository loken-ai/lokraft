# Run model commands

The Terminal takes model commands as typed text: list, load, unload, pull and delete a model, and ask how the connection stands.

## Before you start

<!-- site: widget checklist id=lokraft-terminal-prereq -->
- [ ] lokraft is connected, see [Connect to a server](connect.md).
<!-- site: end -->

## Smallest configuration

None.

## Run it

<!-- site: widget steps id=lokraft-terminal-steps -->
1. Open the Terminal with `Ctrl+5`.
2. Type a command after `$` and press `Enter`.
3. Recall an earlier command with `Up` and `Down`.
<!-- site: end -->

## Commands

| Command | Effect |
|---|---|
| `list` | lists the models of the server with their size, and their modality when it is not text |
| `loaded` | lists the loaded models |
| `load <model>` | loads a model |
| `unload <model>` | unloads a loaded model |
| `pull <model>` | pulls a model from the Ollama registry |
| `delete <model>` | deletes a model from the server's disk |
| `ps` | shows the server address, the connection, the counts of models and the chosen model |
| `clear` | clears the output |
| `help` | lists the commands |

`ps` also prints `Status: Not started (client mode)` under `Server Status`, since lokraft starts no server.

`list` and `loaded` read the list lokraft fetched last.
The refresh button of the top bar fetches it again.  
`pull` always pulls from Ollama.
A Hugging Face model is pulled from Models, see [Manage models](manage-models.md).

## Expected result

<!-- site: widget replay id=lokraft-terminal-replay -->
```
$ loaded
Loaded models:
  llama3.2:latest
$ unload llama3.2:latest
Model llama3.2:latest unloaded.
$ loaded
No models currently loaded.
```
<!-- site: end -->

A command that runs shows `RUNNING` until the server answers.
A failed command tints its entry in the error color.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `Unknown command: <word>. Type 'help' for available commands.` | The word is not a command. | Type `help`. |
| `Usage: load <model_name>` | The command needs a model name. | Add the name. |
| `Another operation is in progress. Please wait.` | A load, pull or delete runs already. | Wait for it to end. |
| `Model <name> is not loaded.` | `unload` named a model that is not loaded. | Check `loaded`. |
