# Sagebox 2D-3D Meta Balls Explorer Demo

This program is a non-GPU (classical graphics) example that calculates a Meta Ball array and
then creates a real-time 60fps 3-D (in 2-D space) animated image with reflective and diffused 
lighting..  With each frame the Meta Ball array is recalculated, as it moves about the window.

This program is meant as an example of using Sagebox in Rust, showing how to use it and showing more
of a real-world example, rather than a simple demo.  

> **Note:** This program uses a lot of real-time graphics routines.  use `cargo run --release` to run in release mode.
Defaulting to debug mode may not reach 60fps, as this is a very intensive, multi-threading non-GPU graphics program.
     
## About this program (Sagebox 2D-3D Meta Balls Explorer Demo)

This program is written as a demo for using Sagebox to write more complete, medium-to-larger
scale programs, something you might write personally, at a company, for education, or for general release. 
 
While other Sagebox videos and tutorials focus on one or two specific features, this program
shows using Sagebox tools in multiple roles as a program expands and become more complex, 
whether for personal use, at a company, educational, etc..
 
This program is a good example of how to use Sagebox to quickly write complete, high-level 
application programs with GUI controls (and sometimes graphics, as in this program) –  
starting in a loosely structured, somewhat ad-hoc approach, which evolves 
into a more organized structure as program elements get more clearly defined.

The main focus of this program (the Meta Ball Shape and 3-D representation) took a few
hours – one or two to get the Meta Ball array roaming about the screen in an amorphous
blob, and another hour or two to add the 3-D lighting and other effects.

The rest followed along as details like the sliders, checkboxes, and
other items were added to make a larger program with a lot more exploration options.

## 2D Classical Graphics vs. GPU

This program is written without using the GPU, showing that we can create amazing things with
basic graphics, without needing to rely on the GPU. This can make programming more accessible
and fun, allowing us to develop graphics-based projects that can be much easier to write. It also
gives us the chance to create real-time prototypes before moving on to the GPU itself.

## Potential GPU version

Moving to the GPU can be a fun project.  For example, if this program were transformed into a 
screensaver or add more complexity, it would be moved to the GPU (Sagebox has many upcoming functions 
for this, too).

GPU programming is a little more complex and requires a different approach, and not everyone 
may have experience with it.  That’s why this example uses classic, procedural graphics, 
as it’s much more accessible and easier to program, and shows how much you can really do quite
a lot with traditional graphics before moving onto the GPU.

## Level Intended

This program is made for anyone who enjoys programming and wants to see how Sagebox functions 
work together. It also shows how simple circles can be used to create the Meta Ball shape, and 
then use that calculated Shape/Array to use the Marching Squares algorithm to create a pretty
accurate outline of a full-sized image with just a small array.

The great thing about  Sagebox is that it allows you to create these types of programs quickly
while keeping things straightforward. You can use it as a library without changing the way you 
normally code. This also makes it easy to add Sagebox to your program well after it is started 
and already working.  

## Coding Style Used in this Program

Code written for both large and small companies typically assumes future maintainers will need to
understand and extend it. While there’s always room for improvement, this example includes comments
in the style of production-quality code — with additional annotations throughout to highlight how 
Sagebox is used in practice.

## How to use this source code

Please feel free to look at the program try it out, take code snippets from it and try it with 
Sagebox for other things.  

## Comments in the code

There are quite a few comments in the code, many of which are added to explain how Sagebox works. 

The other comments are the kind you'd find in any program that would be shared with a team or check
into a company project. They are meant to make the code easy to understand and work with, 
whether in a company environment or just exploring the code.
