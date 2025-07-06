# Sagebox Library Examples

This directory contains the examples implementation of Sagebox for Rust.


| Example | Applicable Uses | Description (see below for more details on each example) |
|-----------------|---------------| ----------------------------------------------------------------------------------|
| `handle_events` | Learning Basics. | A 10-line example demonstrating one method of using events when using controls and drawing graphics. |
| `console_sine_wave`  | Console-Mode Programs with Graphics Controls. Easily Adding onto Existing Code. | Three short examples showing how to add GUI controls to a Console-Mode-only app in just a few lines (5-7 lines each). |
| `double_pendulum`  | Teaching, Hobbyist, Fun-with-Graphics. Medium-sized projects. Non-GPU realtime graphics.  | Three graphical Double Pendulum examples, from simple graphics to comprehensive controls and abilities.     |
| `image_view_sobel`  | One-Line Quick Functions. Industry, Research, Hobbyist. | Two examples using one-line Saegbox `ImageView` calls to display bitmap and image data in a GUI window.    |
| `marching_squares_realtime`  | Larger projects.  Industry, Teaching, Research. Non-GPU realtime graphics. | A comprehensive 1500+ line real-time 60fps demo showing Sagebox in large-scale applications. |
| `plug_in_widgets`  | Industry, Embedded, Personslized Widgets, Arduino. | Demonstrates user-created plug-in widgets to emulate embedded systems graphically. |
| `smooth_mandelbrot`  | Fun-with-Graphics, education. | A small, pure-graphics demo showing a smooth-colored Mandelbrot set with title text. |

<br />
<br />

# Using Examples

## Crates.io

From a crates.io download with Cargo, you can run an example by stating:

`cargo run --example <example_name>`

example:

`cargo run --example marching_squares_realtime` 

To list all examples: `cargo run --example`

## Github Repository examples 

For examples downloaded from Github in their own library, you can use the `Cargo 'run'` command from the top-level directory

`c:\rust\sagebox\examples\marching_squares_realtime> cargo run`

On some examples, using `--release` can work better, as some examples are intensive, realtime graphics programs. 

For example,

`c:\rust\sagebox\examples\marching_squares_realtime> cargo run --release`

Release compiles are the recommended mathod for `marching_squares_realtime` and `double_pendulum examples`

### Notes

- Not all examples are included when obtaining the crate from crates.io
  - plug_in_widgets is located only in the Github Repository
  - When running the `smooth_mandelbrot` example from a crates.io download, use --features mandelbrot
    - e.g. cargo run --example smooth_mandelbrot --features mandelbrot 
    - (This is because it uses the 'num' library)

<br />
<br />

# Example Descriptions

## `handle_events`

### Uses

- Learning basics of using Sagebox in simple programs

### Description

This small, 10-line program shows how to respond to user input from controls by using an event loop. 

There are many different ways to get input from controls (e.g. sliders, buttons) or user actions like moving the mouse. 

This program shows one such way, using a simple event loop that runs until the user dimisses the Window:

- A circle follows the mouse position while the mouse button is down.
- A slider is created so the radius may be changed.
- "Hello World" is drawn in 100pt font in the center of the window
- - An example of using keywords for more control. 
- Demonstrates real-time graphics, control interaction, and event handling in one small block of code.

## `console_sine_wave`

### Uses

- Adding discrete graphic controls to console-mode programs without changing their structure or code.
- GUI controls can be for release, or just for development and conditionally compiled-out. 

### Description

This example builds on a simple ASCII sine-wave generator — a small program that prints a moving sine wave to the console in a continuous loop.

The output simulates a data stream (e.g., sensor input or signal processing), and this example shows how Graphic sliders can control aspects like amplitude and period without introducing an event loop. Controls are checked inline during each iteration.

<details>
<summary>Click for Project Description (3 progressive projects)</summary>

There are three progressively enhanced sub-projects:

| Project Name | Description |
|--|--|
| `console_sine_wave_raw` |  The original ASCII output program with no usage Sagebox. Amplitude and period are fixed; user interaction could be added via keyboard (e.g., IJKL keys). |
| `console_sine_wave_simple` | Adds two sliders (Amplitude, Period) and a Stop button. The sliders dynamically affect the wave; the Stop Button exits the loop.  (As with all console-mdode programs, Control-C may also be pressed top stop the program) |
| `console_sine_wave_more` | Adds a Pause/Resume button.  Demonstrates  Console-Mode usage to create a simple button to stop or pause data output, or to signal some action in the middle of a loop. |

This example contrasts with `handle_events`, which uses an event loop to wait for input. 

Here, the original console loop remains intact. This approach checks graphic controls inline, preserving the original loop structure — useful in console-mode programs where no event loop is needed.

</details>

## `double_pendulum`

### Uses

- Medium-sized programs combining real-time graphics with graphic controls
- Education, research, or just general fun-with-graphics
- Example of non-GPU realtime graphics. 

### Description

This example draws a real-time double pendulum at 60 fps. It includes three versions that build on each other progressively.

The Double Pendulum Examples are progressive, building on the base program that:

- Displays a swinging double pendulum at 60 fps.
- Shows live values (acceleration, mass, speed) are written directly to the graphics window.
- Shows A fading trail shows the path of the lower pendulum.
- Shows timing in (ms) for each frame in the Debug Window
- Progressive variations allow for more control, such as placing each pendulum, altering live vales, and setting display elements.

<br />

> **Suggested use:** Run with `cargo run --release` for best performance.
This is a realtime graphics program that does not use the GPU.  It will run fine in debug mode with no slowness. However it's interesting to see the timing display in both modes (release mode will be about 2x faster)


<details>
<summary>Click for Projects Description (3 progressive projects)</summary>

### `double_pendulum_simple`

- No user interaction — just watch or close the window anytime.
- Slows down naturally and stops after a few minutes.
- Frame timing (ms) is shown in the Sagebox Debug Window.

### `double_pendulum_interactive`

- Adds mouse interaction.
- Drag either pendulum to reposition it.  Release it to start the pendulum moving again.
- Right-click and move the mouse to shift the display up or down
- Launches with an instruction displayed in the window that disappears on first mouse click

### `double_pendulum_full_controls`

- Adds graphic controls so you can change settings with sliders and other controls
- Uses a <i>**Quick Form Window**</i> — a larger window that brings controls and graphics together in one clean interface.
- Adjust mass, gravity, size, zoom, and more using sliders, input boxes, radio-buttons, and checkboxes. 
- Debug timing appears when <i>`Show Debug Info`</i> checkbox is checked, and is otherwise not displayed.

</details>


## `image_view_sobel`

### Uses

- Demonstrates powerful one-line functions for graphical display of bitmaps or calculated data.
- Applicable to industry, education, and research.

### Description 

- Launches a file dialog using the Sagebox `get_open_filename()` to select an image.
- Loads the image into memory. If the file doesn't exist or the user cancels, the program exits with a message.
- Applies a Sobel edge filter to the image.
- Displays the result using Sagebox’s Image View functions.

**Image View functions provide:**

- Resizable windows with zoom and optional navigator panels.
- Multiple images shown simultaneously, with the optional navigator switching between them.
- Modal display or background management by Sagebox (so your program can fire-and-forget and move on without waiting)
- One-line launch with optional retention of the returned object.
  - If the object is discarded, the window is still managed automatically.
    - The returned object can be dropped at any time.  Sagebox manages the Image View window itself. 
  - If retained, the program can interact with it directly.
  
<details>
<summary>Click for Projects Description (2 individual projects)</summary>

There are three progressively enhanced sub-projects:

| Project Name | Description |
|--|--|
| `sobel_image_view` |  Displays the Sobel edge map as a monochrome bitmap using `image_view()`. |
| `sobel_before_after` | Shows original and color Sobel edge map side-by-side using `image_view_before_after()`. |

</details>

## `marching_squares_realtime`

### Uses

- Medium to large-scale applications
- Industry, research, and advanced educational tools
- Real-time visualization, algorithm exploration, and interactive control design
- Example of non-GPU realtime graphics. 

### Description 

This program demonstrates Sagebox in a larger-scale, real-time application. At ~1800 lines with multiple modules, it builds a full-featured Marching Squares explorer running at 60 fps, using animated meta-balls to generate a grid of values (i.e. scalar fields), then calculating both raw and interpolated contours in real time.

It includes:

- Real-time Marching Squares visualization over dynamic data
- Side-by-side rendering of raw and interpolated outlines
- Zoomable display — draw a rectangle to open a second window with a magnified view
- Graphic controls to adjust meta-ball properties and rendering options

This example shows how Sagebox can be used in larger applications for industrial, educational, or research purposes — with real-time performance and procedural design.

> **Note:** For technical details, design structure, and widget implementation, see the individual **README.md** file in the `marching_squares_realtime` project  directory.

> **Suggested use:** Run with `cargo run --release` for best performance.
This is an advanced and aggressive realtime 60fps program.  Compiling in debug mode will be slower, but can run it ok.

<details>
<summary>Click for Project Description (1 large project)</summary>

| Project Name | Description |
|--|--|
| `marching_squares_realtime` |  Full example with GUI controls, real-time rendering, zooming, and meta-ball contour display. |

</details>

## `plug_in_widgets`

### Uses

- Shows how to build external, custom graphic controls and widgets to use with Sagebox
- Useful for embedded emulation, UI prototyping, and modular component design
- A good example for Industrial, educational, embedded applications, and hobbyists (e.g., Arduino)

This example set demonstrates how Sagebox supports user-created graphic controls and widgets — such as dials and LCD-style displays — that can be linked or compiled in as external components.

Though implemented using Sagebox, these controls are independent of the core Sagebox library. Once built, they can be used as native widgets in any Sagebox program. Each project in this directory highlights a different aspect of embedding or integrating plug-in controls and widgets.

> **Note:** For technical details, design structure, and widget implementation, see the individual **README.md** file in the `plug_in_widgets` project  directory.

<details>
<summary>Click for Projects Description (4 projects, 1 larger example)</summary>

| Project Name | Description |
|--|--|
| `dial_widget_plain` | A simple program displaying a standalone dial control styled like a thermostat, with live value display. |
| `lcd_widget_plain` | A basic animated LCD-style counter, counting from 0 to 1,000,000, modeled after an actual LCD component on the market. |
| `lcd_widget_full` | Adding on the last example, this adds toggle buttons, button-text changes, styling options (e.g. blue LED mode), and performance modes. |
| `embedded_emulation_widgets` | A larger example showing use as an embedded emulator or embedded control program.  Combines the dial and LCD into a unified window, simulating an embedded control system with live debug output and a custom graphical About screen.
 |

</details>

## `smooth_mandelbrot`

### Uses

- Fast and easy pixel-based graphics.
- Fun with Graphics, education.

This example draws a Mandelbrot set using a more advanced mathematical approach that eliminates the banded look (visible stripes or abrupt color steps) often seen in Mandelbrot graphic displays. The result is a smoother gradient with a more aesthetic look.

> **Suggested use:** Run with `cargo run --release` for best performance.
In debug mode, image generation may take a couple seconds; in release mode, it completes in ~11 ms.

<details>
<summary>Click for Project Description (1 project)</summary>

| Project Name | Description |
|--|--|
| `smooth_mandelbrot` | Draws a visually smooth Mandelbrot set using Sagebox pixel graphics functions.|

</details>