# Render a video

The Studio renders a short clip from a prompt, or from a start frame and a prompt, and plays it in the window as it decodes.

## Before you start

<!-- site: widget checklist id=lokraft-video-prereq -->
- [ ] lokraft is connected to a loken server built with the `video` family, see [Connect to a server](connect.md).
- [ ] The server holds a Wan checkpoint.
  An image to video checkpoint, marked `i2v`, continues a start frame.
<!-- site: end -->

## Smallest configuration

None.
Every control starts on its default, see [Studio kinds](../reference/studio.md#video).

## Run it

<!-- site: widget steps id=lokraft-video-steps -->
1. Open the Studio with `Ctrl+2` and pick `Video`.
2. Type the prompt.
   A prompt of several lines renders 1 scene per line, cut together into 1 video.
3. Pick the `Model`, the `Duration` and the `Size`.
4. Choose a picture under `Start frame` for an image to video checkpoint.
5. Pick `MP4` or `GIF` under `Format`.
6. Press `Generate`.
<!-- site: end -->

## Expected result

A status line follows the denoising steps, then the decoding, and names the node that renders in a cluster.  
`Stop` tells the server to stop, and both the denoising and the decoding stop at their next step.

The clip plays in the window.
A click opens it fullscreen, where `Space` plays and pauses and the arrow keys step 1 frame.
The play button beside the file name decodes and plays it in the window.  
`Save` writes the file.

lokraft turns the duration into a frame count, rounded to the counts the model takes.

## The time estimate

lokraft asks the server for the denoising time of the settings on screen, with `POST /api/video/plan`.
The answer shows next to `Generate` as `~<time> of denoising`.  
Loading the checkpoint and decoding the frames come on top of it.

Against a server without that route, no estimate shows.

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| No video model in the list | The server holds no Wan checkpoint, or was built without `video`. | Pull a checkpoint, or use a server built with `video`. |
| The start frame changes nothing | The checkpoint is text to video. | Pick an `i2v` checkpoint. |
| Flicker or artefacts with `Auto` | The sampler that `Auto` picked does not suit the render. | Pick `UniPC` or `Heun` under `Sampler`. |
| `<file> is <n> MB; the limit is <max> MB.` | The start frame passes the upload cap. | Export it at a lower quality. |
