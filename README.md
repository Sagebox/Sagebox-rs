<h1 align="center">Sagebox for Rust</h1>

<p align="center">
   <img src="https://user-images.githubusercontent.com/70604831/174466253-c4310d66-c687-4864-9893-8f0f70dd4084.png">
</p>


# A Procedural GUI Designed for Rapid, Creative Development

Sagebox is a set of GUI tools that let you add windows, controls, and graphical output to your Rust programs, all without adding a lot of event-driven or GUI-specific boilerplate code just to have graphics and controls in your program. 

Sagebox is great for adding simple GUI elements to existing programs, building utilities, or experimenting with visual features. Most controls take just one or two lines: one to create it, one to use it. 

Sagebox is also designed for education, hobbyist, and general creative, free-form development and rapid prototyping without the need to write a lot of interface code just to add a button, slider, or other control -- or to remove them.

## A Procedural-First, Scalable GUI

Sagebox is a procedural-first GUI toolkit that lets you build console-mode programs or full graphical applications using straightforward, linear code.

It follows a “simple first, scalable later” approach: one line to define a control, one line to use it. Additional options for more complexity can be added in as needed through the use of keywords in the function call.

Let’s start with a basic <i>Range</i> slider:

```rust
let range_slider = Sagebox::dev_slider("Range");    // Create a default slider with range 0-100

... (later in the program)

let range_value = range_slider.get_pos();               // Get the slider's position value
```

This is a simple slider that sets a default range of 0-100, created using the Dev Window, one of the ways to create a graphics slider in console-based or more graphical applications.

```rust
let range_slider = Sagebox::dev_slider_s("Range", kw::range((200,300)));    // Create a slider with range 200-300

...

let range_value = range_slider.get_pos();               // Get value of the slider position
```

We can easily change the slider's range by adding a keyword.  The <i>_s</i> form (for "scalability") allows the addition of keywords while keeping the simpler form simple.  

With other methods, the slider can be told to fill a memory value (e.g. &mut i32 or &mut f32) as it is moved, so there’s no need to call <i>slider.get_pos()</i>.  This can abstract the GUI from routines that use the slider's real-time position without knowing about the slider or GUI. 

Multiple keywords can be combined: 

```rust
let win = Sagebox::new_window()    // Create a quick, default window
```

This creates a window of default size, background color, font, and other basic window properties.

```rust
let win = Sagebox::new_window_s(kw::title("Test Window") + kw::size((400,500)) + kw::font("Courier New, 12"))
```

This adds keywords to set the title, size, and font.

Event-driven components are available when needed, and can blend seamlessly with procedural code or scale into fully event-driven designs — depending on the needs of your program.

## Why Sagebox? — Fun with Coding and Creative Programming

Like many programmers, I wanted to code creatively while I design. Sagebox was built as a powerful toolset to quickly add (and just as easily remove) controls, widgets, and other elements without the overhead of event-driven or complex GUI boilerplate. 

Sagebox originated from real-world industry work to produce prototypes and finished products rapidly (sometimes in just hours) without sacrificing code quality. It scales with your needs to expand into your overall program, or use it as a development tool that can compile out at runtime.


This approach can make programming more enjoyable by enabling more freeform, exploratory or ad-hoc-style coding, allowing you to concentrate more on the code you want to write rather than the interface code just to have a button, slider, color selector, or whatever else is needed.

## Sagebox in a Nutshell — Scaleable Functions in Three Short Examples

Each example builds naturally on the last — from a simple window to real, usable tools.


<details>
    <summary>Click here to see example code and output</summary>
    
```rust
use sagebox::*; 

fn main() {
    let win = Sagebox::new_window();                   // Create a default-sized window
        let radius = 150;                              // Static value
        win.fill_circle((300,200),radius,"skyblue");
        win.wait_for_close();                          // Wait for the user to close the window
   }
```

Test text here. 

</details>

### More Text Heading

Some Text 
<details>
    <summary>Click here to see example code and output</summary>
    
```rust
use sagebox::*; 

fn main() {
    let win = Sagebox::new_window();                   // Create a default-sized window
    let radius_slider = Sagebox::dev_slider("Radius"); // Slider auto-placed in Dev Window

    // Now we're entering a loop where we can look at or simply ignore events like mouse clicks, button presses, etc.,
    // including our radius_slider moving or changing value.

    while win.wait_event() {                           // Exist when the user closes the window (or exit signal)
        win.cls();                                     // Clear the window Canvas. 
        let radius = radius_slider.get_pos();          // Set a dynamic radius set by slider value
        win.write("Hello, World!");                    // Default 12pt in top-left (styled/centered in next example)
        win.fill_circle((300,200),radius,"skyblue");
    }
}    

```

More Text here


  </details>
test text

### More Text Heading

Some Text

<details>
    <summary>Click here to see example code and output</summary>

```rust
use sagebox::*; 

fn main() {
    let win = Sagebox::new_window();                                            // Create a default-sized window
    let radius_slider = Sagebox::dev_slider_s("Radius",Kw::range((50,300)));    // Set user-defined slider range

    while win.wait_event() {                                                    // Our main loop
        win.cls();                                                              // Clear the window Canvas. 
        let radius = radius_slider.get_pos();                                   // Set a dynamic radius set by slider value
        win.write_s("Hello, World!",Kw::font(100) + Kw::center());              // Center it in the window and make it a 100pt font 
        win.fill_circle((300,200),radius,"skyblue");
    }
}    
  
```
More Text here


  </details>

  More Text here

  
## The Sagebox Philosophy

Sagebox was written with a philosophy guiding its design approach to keep it simple and scalable.

<details>
  <summary>What Drives Sagebox</summary>

  - **Procedural by design** — No event loops, callbacks, or framework ceremonies  
  - **Zero boilerplate** — Add GUI elements with single function calls  
  - **Rapid iteration** — Experiment freely without architectural overhead  
  - **Scales with your needs** — From quick prototypes to production-ready applications  
  - **Developer-first** — Built for how programmers actually think and work  

</details>

