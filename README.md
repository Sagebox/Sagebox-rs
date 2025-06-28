# Sagebox-rs - A Procedural GUI Designed for Rapid, Creative Development

Sagebox is a set of GUI tools that let you add windows, controls, and graphical output to your Rust programs, all without adding a lot of event-driven or GUI-specific boilerplate code just to have graphics and controls in your program. 

Sagebox is great for adding simple GUI elements to existing programs, building utilities, or experimenting with visual features. Most controls take just one or two lines: one to create it, one to use it. 

Sagebox is also designed for education, hobbyist, and general creative, free-form development and rapid prototyping without the need to write a lot of interface code just to add a button, slider, or other control -- or to remove them.

## Why Sagebox? — Fun with Coding and Creative Programming

Like many programmers, I wanted to code creatively while I design. Sagebox was built as a powerful toolset to quickly add (and just as easily remove) controls, widgets, and other elements without the overhead of event-driven or complex GUI boilerplate. 

Sagebox originated from real-world industry work to produce prototypes and finished products rapidly (sometimes in just hours) without sacrificing code quality. It scales with your needs to expand into your overall program, or use it as a development tool that can compile out at runtime.


This approach can make programming more enjoyable by enabling more freeform, exploratory or ad-hoc-style coding, allowing you to concentrate more on the code you want to write rather than the interface code just to have a button, slider, color selector, or whatever else is needed.

## Sagebox in a Nutshell — Three Short Examples
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

