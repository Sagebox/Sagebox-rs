<h1 align="center">Sagebox for Rust</h1>

<p align="center">
   <img src="https://user-images.githubusercontent.com/70604831/174466253-c4310d66-c687-4864-9893-8f0f70dd4084.png">
</p>


# A High-Performance, Procedural GUI Designed for Rapid, Creative Development

Sagebox is a set of GUI tools that let you add windows, controls, and graphical output to your Rust programs using procedural, linear code without requiring any event-driven code. 

Built from scratch, Sagebox was designed to support cross-platform, high-performance GUI applications with no boilerplate whatsoever, and without the overhead of event-driven frameworks.

Sagebox is great for adding simple GUI elements to existing programs, building utilities, or experimenting with visual features.

Sagebox is also designed for education, hobbyist, and general creative, free-form development and rapid prototyping without the need to write a lot of interface code just to add a button, slider, or other control -- or to remove them.

## Scalable, Easy-to-Use Procedural GUI Tools

Sagebox is a procedural-first GUI toolkit that lets you build console-mode programs or full graphical applications using straightforward, linear code. 

Though rarely needed, for some programs, Sagebox can use event-driven components, blending seamleassly with procedural Sagebox code or scaling into a fully event-driven model. 

Tools and Controls in Sagebox start simple and can scale as you want to do more.  Additional options for more complexity can be added in as needed through the use of keywords in the function call.

Sagebox manages its own environment, so you don't have to keep track of any of the controls or widgets that are launched, unless you want to.

### Sagebox Stays Out of Your Way

- <small>Bullet List Item-
- <small>Bullet List Item-
- <small>Bullet List Item-
- <small>Bullet List Item-
- <small>Bullet List Item-

## Quick Examples (Sagebox in a Nutshell)

If you want some quick controls such as buttons, sliders, or some text widgets, etc., you typically use controls in two lines of code: one line to define the control, and one line to use it.

For example, if you want a slider and a button, all you need are these two lines:


```Rust
let box_slider = Sagebox::dev_slider("box size");
let my_button = Sagebox::dev_button("Press me");
```

![output-dev-slider-box-0](https://user-images.githubusercontent.com/70604831/174466571-6d968e7b-3e87-4cfa-8060-602137041084.png)

The above code uses the Dev Window controls (one of a few ways to create controls in Sagebox), which creates a slider labeled ***"box size"***, with a default range of 0-100 and default value of 100, followed by a button.

Sagebox puts these in a window for you, and will delete them later when the program ends or the window is dismissed.
When you want to use the controls, you can just call <i>**`box_slider.get_pos()`**</i> and <i>**`my_button.pressed()`**</i>.  You can also use <i>**`my_slider.moved()`**</i> to determine if the slider has been moved since the last time checked.


For example,

```rust
let box_size = box_slider.get_pos();
if my_button.pressed { println!("Button was pressed!"); }
```

You can set a custom range and default to the slider by using keywords and chaining them together when you create the slider:


```rust
// Create a slider with some keywords.  The _s form denotes scaleability, to keep the simple form simple.

let box_slider = Sagebox::dev_slider_s("Box Size",kw::range((10,500)) + kw::default(150));
```

![output-dev-slider-box-150](https://user-images.githubusercontent.com/70604831/174466616-fed9d593-d165-458f-9c55-84ba93524adf.png)


Now the slider has a range of 10-500, and a default of 150, as shown in the image above.  You can also use floating-point sliders.

With other slider functions, the slider can be told to fill a memory value (e.g. <i>`&mut i32`</i> or <i>`&mut f32`</i>) as it is moved, so there’s no need to call <i>**box_slider.get_pos()**</i>. This can abstract the GUI from routines that use the slider's real-time position without knowing about the slider or GUI.

Sliders, radio buttons, checkboxes, input boxes, list-boxes, text widgets, and other controls are all this easy to use, and can be scaled to more complexity.


With various widgets, you can call up color selectors, date pickers, formatted message boxes, image view & img before/after windows, and so-forth.

This really represents Sagebox in a nutshell -- all examples for Sagebox (even the larger, more comprehensive ones), use the above approach, just with more Sagebox tools and functions.


## Simple and Powerful
![output-cpp-basics-light](https://user-images.githubusercontent.com/70604831/174572814-6cc3092e-d171-420d-b3e7-a9f73d40992c.png)

By adding just a few more lines of code, more tools can be created. 

The above examples show using Sagebox functions as easy library calls, from basic controls (left), to graphics with and without controls (right) &ndash;
all with just simple function calls without event-driven or GUI programming, representing a small amount of the overall code that stays out of the way and doesn't require any programming or GUI environment. 

The middle image, for example, is a one-line function call:

```C++
Sagebox::image_view_before_after(&image1,&image2);
```

where elements can be added to extend its functionality, and the return class can be kept (or discarded) to control and manage the created window.
<br /><br />



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

