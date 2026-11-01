# Connect to a server

Point lokraft at a loken or Ollama server, on this machine or another one, with a key when the server asks for one.

## Before you start

<!-- site: widget checklist id=lokraft-connect-prereq -->
- [ ] The address and port of the server.
- [ ] A key from the server's `api_keys` when its `require_auth` is on.
- [ ] For a server on another machine: the server listens on an address that machine exposes, such as `0.0.0.0`.
<!-- site: end -->

## Smallest configuration

None for a loken server on this machine.
lokraft starts on the connection `Local`, which points at `http://localhost:11435`.

## Run it

<!-- site: widget tabs group=connect id=lokraft-connect-method -->
### At start

```sh
lokraft --server http://192.0.2.10:11435
```

`--server` takes the address as written, scheme included.
It replaces the address of the active connection for this run, and lokraft saves nothing of it.

### In Settings

1. Open Settings with `Ctrl+6`.
2. Under `CONNECTIONS`, type a name in `Name` and the address in `Address`.
   A bare `host:port` gets `http://` in front.
3. Fill `API key` when the server requires one.
   The key is sent on every request as `Authorization: Bearer <key>`.
   An empty field sends no key.
4. Press `Add connection`.
5. Press `Use` on the new row.

`Edit` on a row changes its name, its address or its key, and `Delete` removes it.
Settings keeps at least 1 connection.

### From the top bar

1. Click the connection name at the left of the top bar.
2. Pick another connection in the list.
   `Manage connections` opens Settings.
<!-- site: end -->

## Expected result

A change of address or key reloads the model list from the new server, with a `Connecting to <address>` notice.  
The lamp of the top bar then turns green, and the pointer on the connection name gives `CONNECTED`.
The pill at the right of the top bar counts the loaded models.

The lamp is amber while lokraft connects.
It is off while no server answers, and the pointer then gives `OFFLINE`.

## Against Ollama

Chat, Models and the Terminal work against Ollama.
The Studio and Auto do not, since Ollama serves none of their routes.  
lokraft loads a model on an address with port `11434` through a chat request, which is how Ollama loads one.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| The lamp is off and a toast says `Failed to list models: Request error: could not connect to the server (...)` | No server listens at that address, or a firewall blocks it. | Check the address and the port. Bind the server to an address the network reaches. |
| `Failed to list models: Request error: request timed out (...)` | The server did not answer the model list in time. | Check the server is not stuck, then press the refresh button of the top bar. |
| `... failed with status 401 Unauthorized: ...` | The server requires a key and got none, or a wrong one. | Edit the connection and fill `API key` with a key of the server. |
| `Enter the address of the server.` under the form | `Address` is empty. | Type the address. |
| `An address holds no space.` under the form | `Address` holds a space. | Remove the space. |
| `Another connection already has this name.` under the form | The name is taken. | Pick another name. |
