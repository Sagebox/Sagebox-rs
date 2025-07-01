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
//        requires a Ctrl-C interrupt; later versions use Sagebox controls for a smoother end-of-program
//

use std::*; 

use sagebox::*; 

static YELLOW : &str = "\x1b[1;33;20m";
static WHITE  : &str = "\x1b[1;37;20m";

fn main()
{
    let mut angle       = 0.0;  // Current Angle
    let     period      = 40.0; // Lines per-period
    let mut amplitude   = 20.0; // char-width
    let mut frequency   = 1.0;

    // Create the two sliders for Amplitude and Frequency
    // - The frequency slider is a 'floating point' slider that can support floating-point values

    let amp_slider      = Sagebox::dev_slider_s("Amplitude", kw::default(amplitude));
    let freq_slider     = Sagebox::dev_sliderf_s("Frequency", kw::default(frequency) + kw::range((0.1,5.0)));

    // Create the Stop and Pause Buttons
    // - kw::width(100) sets the width to 100 pixels, since these are short words and would otherwise create short buttons.
    
    let stop_button     = Sagebox::dev_button_s("Stop",kw::width(100));
    let pause_button    = Sagebox::dev_button_s("+Pause",kw::width(100));

    loop
    {

        amplitude = amp_slider.get_pos_f();     // Get slider values for amplitude and frequency
        frequency = freq_slider.get_pos_f();    // --> get_pos_f() can be used to return f32, even in non-floating point sliders

        // If either slider has moved, print out the current amplitude and frequency values
        // 
        // We can print these to a new window, or a window we create in the Dev Window (so it is nicely organized).
        // Since this represents debug information, they put out to the built-in Sagebox debug window, since it's easy to do. 
        // (we can't print these out to the console window, because it's too busy with our streaming output data respresnted
        // by the sine-wave).

        if amp_slider.moved() || freq_slider.moved()
            { Sagebox::debug_writeln(format!("amplitude: {{g}}{:.4}{{}}, frequency : {{y}}{:.4}",amplitude,frequency)); }

        let y = (amplitude + amplitude * f32::sin(angle)) as usize; // Get the sin() value (which we put out as spaces)

        println!("  y: {y:0>3} {YELLOW}{: <1$}****{WHITE}","",y);   // Print out 'y' spaces, then '****' for our sine wave

        angle += frequency * 3.14159 * 2.0 / period;        // Go to the next position based on the frequency

        thread::sleep(time::Duration::from_millis(25));     // Wait a little bit, otherwise we go way too fast

        // If the pause button is pressed, we have a few ways of handling this: 
        //
        //  1. The way it is done in the next line of code - wait for the user to press the button again.
        //      - We can change the text on this button with button.set_text("continue") and then change it back. 
        //  2. Use some other form of input, such as looking for a keyboard press to continue.
        //  3. Use a closure or call out to some other function for some criteria to continue. 
        //  4. Enter into a procedural event-loop so we can look at the 'pause' button as well as the 'stop' button
        //     and field any other events happening while the display is paused. 
        //  5. Have another thread react to the button press and set a boolean somewhere
        //  6. Use a signal set in the button so we can react to a boolean we might send to this function
        //     - this allows abstracting Sagebox and all GUI/graphic control components from called functions. 

        if pause_button.pressed() 
        { 
            println!("Press Pause Button again to continue."); 
            pause_button.wait_for_press(); 
        } 

        // If the stop button is pressed, exit the loop and exit back to the OS

        if stop_button.pressed() { println!("Stop Button Pressed.");  break; }

    }
}