
# Source Temporarily Offline

## Status of the Source for the Meta Balls Explorer Demo

The source code for the `meta_ball_explorer` project has been temporarily taken offline.

The reason for this is to replace the threading model from a temporary one to a worker-thread model,
making it a better example of larger, industry-style programs, as well as to showcase general multi-threading programs that need
higher levels of performance.

It happened like this:

- The original Meta Ball Explorer program started as just a Meta Ball program; a neat little project to show 3D-lighting on 2-D metaballs.
- It turned out very well, that it just kept growing into something much nicer and more comprehensive: The Meta Ball Explorer
- The `threading method` used was just a simple set of temporary threads, which worked fine, but not representative of a permanent solution.
- The decision was made to create a more sturdy `worker thread` model as a better example.

This process will delay the release of the source code by about two weeks.

In the meantime, check out the `realtime_marching_squares` demo code, as it is also multi-threading (and uses the same threading model that is being adjusted here, as it is more appropriate for the marching squares demo)