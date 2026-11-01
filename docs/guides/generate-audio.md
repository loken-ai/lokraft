# Generate music, sound and speech

The Studio renders a music clip, a sound effect, a MIDI score or spoken text from a prompt, and plays the result in the window.

## Before you start

<!-- site: widget checklist id=lokraft-audio-prereq -->
- [ ] lokraft is connected to a loken server built with the `audio` family, and `midi` for MIDI, see [Connect to a server](connect.md).
- [ ] The server holds the model of the kind: ACE-Step for music, EzAudio or Stable Audio for sound effects, a MIDI model, Parler, Kyutai or Piper for speech.
- [ ] lokraft is built with audio output, see [Install](../start/install.md).
<!-- site: end -->

## Smallest configuration

None.
Each kind keeps its own settings, so moving from one kind to another loses nothing.

## Run it

<!-- site: widget tabs group=audio id=lokraft-audio-kind -->
### Music

1. Open the Studio with `Ctrl+2` and pick `Music`.
2. Type the prompt, such as `upbeat synthwave with driving bass, 128 bpm`.
3. Set `Duration` and `Tempo`, and `Lyrics` for a song.
4. Press `Generate`.

`seamless loop`, under `Loop`, cuts a whole number of bars and crossfades the tail into the head.
`force the whole duration`, under `Full length`, keeps the song from ending before the length asked.  
Picking a checkpoint sets its steps and CFG.

### SFX

1. Pick `SFX`.
2. Type the prompt, such as `heavy rain on a tin roof with distant thunder`.
3. Pick the `Model` and set `Duration`.
4. Press `Generate`.

A source clip under `Variation of`, on a model that offers it, makes a variation of that clip.
`seamless`, under `Loop`, makes a loop of whole bars, on a model that offers loops.

### MIDI

1. Pick `MIDI`.
2. Type the prompt, such as `a cheerful piano and strings waltz`.
3. Set `Max tokens`, the length budget of the score.
4. Press `Generate`.

The result is a `.mid` file to save.

### Speech

1. Pick `Speech`.
2. Type the text.
3. Pick the `Engine`: `Parler (EN)`, `Kyutai (EN/FR)` or `Piper`.
4. Describe the voice under `Style` for Parler, or name a voice under `Voice` for Kyutai and Piper.
5. Press `Generate`.

For Parler, the button beside the `Voice` list fetches the voices of the server.
<!-- site: end -->

## Expected result

A status line follows the server's steps.
`Stop` tells the server to stop.  
The clip plays in the window with a play and pause button, a stop button and a seek bar, and `Save` writes it to a file.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `No SFX model available` | The server lists no sound effect model. | Pull one, see [Manage models](manage-models.md). |
| The player reports `no audio output device` | The system has no audio output, or none is free. | Connect an output, or save the clip and play it elsewhere. |
| The player reports `built without the native-audio feature` | lokraft was built without audio output. | Build again with the default features. |
| `Generate` stays grey | The prompt is empty, or a render is running. | Type a prompt, or wait for the render. |
