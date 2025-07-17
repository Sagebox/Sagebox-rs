# Rust Console Sine-Wave Example

This example reflects a common console-mode scenario: a program that processes large amounts of data and 
outputs to the terminal or console window.

During development, it’s often helpful to have live controls and visual feedback -- features that may be
removed or disabled in the final release via #cfg or if() statements.

Sagebox based-code can add graphical controls for development and debugging, without requiring any changes
to existing code.

One of the main points of this example is to show that Sagebox-based source code:

1. Can be added on top of existing code without changing the structure of the code or data it uses
2. Can be easily removed or excluded using conditional compilation (e.g., with #cfg flags).
3. Follows the "One Line to Define it. One Line to Use it." Sagebox principle for graphic-controls.

The three examples in this section progressively build on each other do a little more each time:

| Project Name | Description |
|--|--|
| `console_sine_wave_raw` |  The original ASCII output program with no usage Sagebox. Amplitude and period are fixed; user interaction could be added via keyboard (e.g., IJKL keys). |
| `console_sine_wave_simple` | Adds two sliders (Amplitude, Period) and a Stop button. The sliders dynamically affect the wave; the Stop Button exits the loop.  (As with all console-mdode programs, Control-C may also be pressed top stop the program) |
| `console_sine_wave_more` | Adds a Pause/Resume button.  Demonstrates  Console-Mode usage to create a simple button to stop or pause data output, or to signal some action in the middle of a loop. |

# Watch the Demo on Youtube

[Sine Wave - Drop-In GUI Controls for Rust Console Programs  (Github Demo)](https://youtu.be/A6atjDW5Jxg)

# About this Example

This example builds on a simple ASCII sine-wave generator — a small program that prints a moving sine wave to the console in a continuous loop.

The output simulates a data stream (e.g., sensor input or signal processing), and this example shows how Graphic sliders can control aspects like amplitude and period without introducing an event loop. Controls are checked inline during each iteration.
