<h1 align="center">Sagebox for Rust</h1>

<p align="center">
   <img src="https://user-images.githubusercontent.com/70604831/174466253-c4310d66-c687-4864-9893-8f0f70dd4084.png">
</p>


# A High-Performance, Procedural GUI Designed for Rapid, Creative Development

Sagebox is a set of GUI tools that let you add windows, controls, and graphical output to your Rust programs using procedural, linear code — without any boilerplate or the overhead of an event-driven framework. 

Built from scratch, Sagebox was designed as a comprehensive GUI to support everything from cross-platform, high-performance applications, fun-with-graphics programs, down to very simple console-mode-only applications using GUI controls for development or release.

Sagebox drops cleanly into existing code or new projects when rapidly prototyping or exploring creative ideas.

Sagebox is also designed for education, hobbyist, and general creative, free-form development and rapid prototyping without the need to write a lot of interface code just to add a button, slider, or other control -- or to remove them.

## Scalable, Easy-to-Use Procedural GUI Tools

Sagebox is a procedural-first GUI toolkit that lets you build console-mode programs or full graphical applications using straightforward, linear code. 

Though rarely needed, for some programs, Sagebox can use event-driven components, blending seamleassly with procedural Sagebox code or scaling into a fully event-driven model. 

Tools and Controls in Sagebox start simple and can scale as you want to do more.  Additional options for more complexity can be added in as needed through the use of keywords in the function call.

Sagebox manages its own environment, so you don't have to keep track of any of the controls or widgets that are launched, unless you want to.


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

```rust
Sagebox::image_view_before_after(&image1,&image2);
```

where elements can be added to extend its functionality, and the return class can be kept (or discarded) to control and manage the created window.
<br /><br />

# Getting Started

### Install with Cargo

```sh
    cargo add sagebox-rs
```

### With `Cargo.toml`

```sh
    [dependencies]
    sagebox = "0.1"
```
### Simple Program Example

```rust
use sagebox::*

fn main{
    let window = Sagebox::new_window();
    window.write("Hello World!");
    window.wait_close();               // Wait for user to close the window
}
```

#### For a more fun version with a 100pt font centered in the window, just use <i>`write_s()`</i> and add a couple keywords:

```rust
    window.write_s("Hello World", kw::font(100) + kw::center());
```

#### Or set the font size within the text:
```rust
    window.write_s("{100}Hello World",kw::center());
```

## Table of Contents
- [Why Sagebox? — Fun with Coding and Creative Programming](#why-sagebox--fun-with-coding-and-creative-programming)
- [Designed to stay out of the way of your code](#designed-to-stay-out-of-the-way-of-your-code)
- Beta Version
- Project Roadmap

## Why Sagebox? — Fun with Coding and Creative Programming

Like many programmers, I wanted to code creatively while I design. Sagebox was built as a powerful toolset to quickly add (and just as easily remove) controls, widgets, and other elements without the overhead of event-driven or complex GUI boilerplate. 

Sagebox originated from real-world industry work to produce prototypes and finished products rapidly (sometimes in just hours) without sacrificing code quality. It scales with your needs to expand into your overall program, or use it as a development tool that can compile out at runtime.


This approach can make programming more enjoyable by enabling more freeform, exploratory or ad-hoc-style coding, allowing you to concentrate more on the code you want to write rather than the interface code just to have a button, slider, color selector, or whatever else is needed.

 
## Designed to stay out of the way of your code

Sagebox can be added to existing Rust code without changing its structure or setup—and you don’t need to write a “Sagebox program” to start using it. It works with native Rust types (like `i32`, `f32`, `(i32,i32)`) and integrates cleanly alongside other libraries, including other GUIs.

  
## Using Sagebox in Console Mode

![output-console-mode](https://user-images.githubusercontent.com/70604831/174466676-d8cec449-a241-4402-9b7e-0e354a4d0777.png)

Sagebox can be used with GUI/Windows based programs or regular console mode programs.  In console mode, you can use Sagebox functions to help with the program
user-interface, such as bringing up entry boxes and other dialogs, as well as before & after windows, color selectors, etc.

Since these are called as simple functions, you can just put them in your code without changing your style or interfering with the rest of your code.

Console-based programs can use Sagebox for development even when the end-product doesn't have any UI or GUI components, or can use some of the GUI-based library
calls to help with user input and program flow while developing and debugging. 


# Fun with Graphics

![output-collage-graphics](https://user-images.githubusercontent.com/70604831/174466730-86c6f38a-e743-4f97-be99-8d84be64d39f.png)

Sagebox can also be used as a full GUI when you want.  Sagebox has a lot of graphics and other functions to allow building GUI-based applications.

You can place specific controls, create graphic buttons, as well as use many drawing graphics functions. 

Sagebox can also be used as a full GUI when you want, staying out of the way when you don't.

Sagebox has a lot of graphics and other functions to allow building GUI-based applications, the above collage showing some programs using the Sagebox graphics
functions.  For most of these programs,  the Sagebox usage is just a few lines of code, outputting the results of whatever the code is generating.

# Standalone and 3rd-Party Widgets

![output-collage-widgets](https://user-images.githubusercontent.com/70604831/174466845-3b17fbe1-85bc-43f8-9f38-6dd1e248f8af.png)

Sagebox has a lot of support for writing widgets, with many pre-made widgets coming soon now that the Alpha release is out. 

Widgets can be completely standalone and used on their own with just a call, and do not need a GUI interface.  Anyone can write a widget that can be
used as a standalone object for use with any program. 

The above examples are the Color Selector, Dial Example Widget, LCD Example Widget, and Spline Widget.

### Embedding Widgets into Windows

![output-emulation](https://user-images.githubusercontent.com/70604831/174466885-1ac37379-5cb0-4538-83c5-1cefeab58dea.png)

Widgets can be embedded seamlessly into a window to create a larger GUI-based interface with little code. 

The above is an example of using two widgets together to emulate or control an Arduino or other hardware.

When the dial is moved by the user, the LCD reflects the Dial value, which is also printed to the window using different colors to highlight the values.  The LCD widget is placed on a circuit board image, and the Dial Widget is placed on a stucco background to emulate a wall. 
A smaller child window is created to show the display, and two buttons are added to start/stop the emulation and quit the program.   
There is also a nice rounded title bar on top. 


# Fast Real-Time 3-D GPU Graphics

![output-collage-gpu](https://user-images.githubusercontent.com/70604831/174467047-dda08078-cf76-4d76-af24-7689271d5a56.png)

Soon to be released, Sagebox features fast, real-time 3-D GPU functions.  Shown above are some examples of real-time 60fps+, high-resolution graphics using the GPU.
These are taking roughly 30us of microprocessor time when rendering over 1 million pixels. 

To render 1 million changing pixels in real-time can also be done in just a few milliseconds with the multi-threading AVX functions written for
Sagebox (most of which are expected to be released into open-source). 

These functions will be released soon, with more coming in the next few months for creating programs with GPU-based real-time graphics. 

# High Performance Computing: Super-Fast AVX, Multi-Threading Functions

![output-collage-avx-both](https://user-images.githubusercontent.com/70604831/174681183-a4fd9c49-b98c-4247-8817-537682f5a5fa.png)

Sagebox and Sagebox was originally started as a platform to develop and explore a number of different projects, such as GPU-based projects, neural networking
and so-forth. 

With Sagelight Image Editor and other projects, a lot of multi-threading AVX/SIMD code was written for very fast processing. 

Now that Sagebox is released, these functions will follow shortly, mostly released as open source. 

Some examples are shown above, all multi-threaded AVX/SIMD functionality, such as the Gaussian/Sinc/Kernel Blur shown above, transferred from Sagelight
Image Editor and other source code, with more coming. 

The example to the right shows a still from a real-time, constantly re-generated texture with 1 million polygons that is actually created on the CPU and transferred
to the GPU twice as fast as sending it directly to the GPU due to the AVX it uses  -- it's actually a 2D image where it is much faster to process
the polygons, lighting, and reflections on the CPU than it is to have the GPU do it, thanks to AVX.  This adds a lot of power to creating quick, easy, and
generic functions with the GPU. 

Look for releases in the next few weeks after this initial Sagebox release.

  
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

