
// --------------------------------------------------------------
// Sagebox Plug-in Widget Example - Dial Widget Library Interface 
// --------------------------------------------------------------
//
// This Dial Widget is an example of using plug-in widgets that anyone can write for Sagebox.
//
// This is the interface to the widget, which can be a .LIB or .DLL file, depending on the 'link-static' 
// feature set in the dependency in the calling cargo.toml file.
//
// The .LIB file is compiled as a separate library and then linked to and interface with this lib.rs file. 
//
// --------------------------------------
// About This Example - Plain Dial Widget
// --------------------------------------
//
// This example is very simple (almost too simple). It does the following
//
//      - Creates a main Window
//      - Creates a dial widget and places it in the window
//      - Creates a general one-line event-loop that displays any dial value changes to
//        the built-in Sagebox debug window
//      - If the window is closed during this process, the event loop exits and the program ends.
//
// ---------------------------------
// A Good example of General Widgets
// ---------------------------------
//
// While the Dial Widget was written as a specific thermostate-like tool, it's also a good example of 
// writing widgets for any use, due to its usefulness in using a radial dial.
//
// This widget, for example, can be adapted to a generic dial-based slider with a central display that
// can be used for general purposes, rather specifically for emulating a hardware-based thermostat
//
// ---------------------
// About the Dial Widget
// ---------------------
//
// The Dial Widget is an example of an emulated interface to a physical hardware control such as a
// wall thermostat controlling the temperature to a heater or some other device.
//
// The Dial Widget is an example of a more complex widget that has a graphic display and changes based
// on mouse movements (setting the temperature) as well as button clicks. 
//
// The Dial Widget does the following: 
//
//	- Allows the mouse to set the temperture in a radial mannter by moving the small knob in the dial/thermostat.
//  - Allows the temperature to be set in increments by pressing the '<' and '> buttons with the mouse.
//  - Development Debug Modes - Allows the user to enter a debug mode by right-clicking the mouse: 
//    - ** Document modes here **
//
// A more extensive Dial widget in a real-world scenario would be able to send signals directly to the hardware, as well as receive feedback 
// about actual temperature vs. the temperature set on the dial. 
//
// ----------------------------
// The Widget and using Sagebox
// ----------------------------
//
// This widget creates a small window for it's own display, and then places this window in the parent window
// passed to the widget when it was created.
//
// This widget uses Sagebox display functions to display the data and maintain the window it creates, as well
// changing display states. 
//
// Sagebox input functions using the mouse, mouse clicks (both left and right), as well as debug code is also used in this
// example.

use sagebox::*;

fn main() 
{
    // Create the window sized to fit the LCD Widget and the controls we're going to create later.
    // - We can also get the size of the window from the dial widget to help us decide on the parent window size.

    let win = Sagebox::new_window_s(kw::size((270,300)));

    // By default, windows cannot be closed by the user, and the 'closed_button_pressed()' function of the window must be called
    // to determine if the user tried to close it, to which the program can react accordingly. 
    //
    // using win.enable_close() allows the window to be closed by the user, which
    // allows Sagebox to start closing it down.
    //
    // It is used here to let get_event() (used below) to return false and exit the while loop automatically.
    // - Otherwise, we could just check with win.close_button_pressed() or simply change the event loop code to 
    //       while win.get_event() && !win.close_button_pressed()
    //
    // win.enable_close() is used here as an example of usage, and isn't specifically needed.

    win.enable_close();     // Allow the window to close, which allows Sagebox to set that flag internally by itself.

    let dial = dial_widget::new(&win,(25,60));      // Create a new dial widget at location (25,60)

    // Write a title message to the window
    //
    // {32} -- Sets a font size of 23 (Arial font)
    // Kw::center_x() -- Centers the message horizontally (which is why the location's X value is 0, as it is ignored)

    win.write_xy_s((0,20),"{23}Dial Widget Example",kw::center_x()); 

    // Enter the main event loop.  
    // - get_event() will return false if the user closes the window (since we set enable_close() above)

    while win.get_event()
    {
        // If the dial value changed (i.e. the user moved the knob or clicked the '<' or '>' button), display
        // the value to Sagebox's built-in debug window.
        // --> {{g}} sets the text color to green (double {{}} are used because we're using the format function)

        if dial.value_changed() { Sagebox::debug_writeln(format!("value = {{g}}{}",dial.get_value_i32())); }

    }
    Sagebox::exit_button();     // Since we were in an event loop waiting for the window to close, this function is not needed.
                                // But, it is always nice to get a visible indication the program intended to end, so a button to press
                                // lets the user know everything is as intended, rather than closing down the window and returning the OS 
                                // suddenly. 
}
