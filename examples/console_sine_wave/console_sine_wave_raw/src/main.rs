// ------------------------------
// Rust Console Sine-Wave Example
// ------------------------------
//
// This example reflects a common console-mode scenario: a program that processes large amounts of data and 
// outputs to the terminal or console window.
//
// During development, it’s often helpful to have live controls and visual feedback -- features that may be
// removed or disabled in the final release via #cfg or if() statements.
//
// Sagebox based-code can add graphical controls for development and debugging, without requiring any changes
// to existing code.
// 
// One of the main points of this example is to show that Sagebox-based source code:
// 
// 	    1. Can be added on top of existing code without changing the structure of the code or data it uses
// 	    2. Can be easily removed or excluded using conditional compilation (e.g., with #cfg flags).
// 	    3. Follows the "One Line to Define it. One Line to Use it." Sagebox principle for graphic-controls.
// 
// The three examples in this section progressively build on each other do a little more each time:
// 
// 	    1. Sine_Wave_Raw        - A standard console-mode program with no Sagebox code.
// 	    2. Sine_Wave_Simple     - Adds sliders for amplitude and frequency, along with a “Stop” button.
// 	    3. Sine_Wave_More       - Extends the Simple version by displaying live values in the Sagebox debug 
//                                window and adding a “Pause” button.
//
// ------------------
// About this Example
// ------------------
// 
//      • The sine wave represents a stream of output data.
//      • Amplitude and frequency sliders serve as live input variables that shape the output.
//      • The program runs continuously until stopped. In the original version, stopping the program
//        requires a Ctrl-C interrupt; the other versions use a graphic button for a smoother end-of-program
//

use std::*; 

static YELLOW : &str = "\x1b[1;33;20m";
static WHITE  : &str = "\x1b[1;37;20m";

fn main()
{
    let mut angle       = 0.0;  // Current Angle
    let     period      = 40.0; // Lines per-period
    let     amplitude   = 20.0; // char-width
    let     frequency   = 1.0;

    loop
    {
        let y = (amplitude + amplitude * f32::sin(angle)) as usize; // Get the sin() value (which we put out as spaces)

        println!("  y: {y:0>3} {YELLOW}{: <1$}****{WHITE}","",y);   // Print out 'y' spaces, then '****' for our sine wave

        angle += frequency * 3.14159 * 2.0 / period;        // Go to the next position based on the frequency

        thread::sleep(time::Duration::from_millis(25));     // Wait a little bit, otherwise we go way too fast

    }
}