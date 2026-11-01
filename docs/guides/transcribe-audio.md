# Transcribe or separate audio

The Studio turns an audio file into text, translated to English on request, or splits a song into its vocal and instrumental stems.

## Before you start

<!-- site: widget checklist id=lokraft-transcribe-prereq -->
- [ ] lokraft is connected to a loken server built with the `audio` family, see [Connect to a server](connect.md).
- [ ] The server holds a Whisper or Voxtral model for a transcript, or Mel-Band RoFormer for stems.
- [ ] An audio file in a common format.
  The server converts it.
<!-- site: end -->

## Smallest configuration

None.

## Run it

<!-- site: widget tabs group=listen id=lokraft-transcribe-kind -->
### Transcribe

1. Open the Studio with `Ctrl+2` and pick `Transcribe`.
2. Press `Choose audio` under `Audio file`.
3. Type the model in `Model`, or leave it empty for the model of the server, and pick the language under `Language`.
   `Detect` lets the model find it.
4. Tick `to English` under `Translate` for a translation instead of a transcript.
5. Press `Generate`.

### Separate

1. Pick `Separate`.
2. Press `Choose audio` under `Track`.
3. Pick `Both stems`, `Vocals only` or `Instrumental only` under `Stems`.
4. Press `Generate`.
<!-- site: end -->

## Expected result

A transcript shows as text, with `Copy text` to put it on the clipboard.  
Stems come back as clips to play and save.
The 2 stems add back to the original exactly.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `Generate` stays grey | No file is chosen. | Choose the file. |
| A transcript in the wrong language, or drifting mid-file | Detection misheard a short clip or an accent. | Name the language under `Language`. |
| `<file> is <n> MB; the limit is <max> MB.` | The file passes the upload cap. | Cut the file, or export it at a lower quality. |
