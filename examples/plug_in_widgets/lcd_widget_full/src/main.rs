
// -------------------------------
// Rust SageBox LCD Widget Example
// -------------------------------
//
// This program demonstrates a simple top-level emulation of an LCD display using plug-in Sagebox widgets 
// that anyone can write for Sagebox
// 
// The code is small in length, showing shows how to include and use a widget with a single line. 
// 
// While currently high-level, the emulation can be expanded to simulate lower-level functions
// such as LCD segments and output ports, and eventually interface directly with
// embedded hardware.
//
// ----------------------------------------------
// Using the Program - Fast Mode vs. default Mode
// ----------------------------------------------
//
// This program has two buttons that each count from 0 to 1,000,000, demonstrating different performance techniques in SageBox.
//
// Default Mode Button:
//
// Counts with continuous LCD updates. Shows basic usage such as updating button titles and handling input through signals.
//
// Fast Mode Button:
//
// Uses FastMode() to limit updates to every 10–20ms, greatly increasing performance by avoiding redundant redraws.
// Internally, UpdateReady() checks when to update, and LastUpdate() handles any final display output.
//
// The loop remains simple, while the LCD function handles efficient timing behind the scenes.
//
// --------------------
// About the LCD Widget
// --------------------
//
// The LCD Widget is a simple example of an emulated interface to an LCD. 
//
// As an example of an embedded display widget, it can
//
//  - Display an LCD modeled after an exising, specific hardware LCD 
//	- Take simple numbers to display to the LCD screenwidget
//  - Choose between to modes:  a regular LCD mode, and a Blue Led Mode
//
// A more extensive LCD widget in a real-world scenario would be able to work with different modes from lower-higher levels,
// for an emulation at all levels:
//
//  - Base mode to send simple numbers (the mode supported by this example)
//  - Lower-level modes to support setting individual LCD segments for each number
//  - Allow selection of exterior LCD digit modes, such as the '.' and other lcd points 
//  - Even low-level modes to support the actual hardware interface, talking in I2C or some other format to talk to the hardware directly.
//

use sagebox::*;
use lcd_widget::LcdMode;


fn main() 
{
    // Create the window sized to fit the LCD Widget and the controls we're going to create later.

     let win = Sagebox::new_window_s(kw::pos((100,100)) +  kw::size((320,320)) + kw::title("Rust Sagebox - Lcd Widget Example"));
    win.cls_s("skyblue,skybluedark");   // Clear window with a vertical gradient

    let lcd_pos = (10,10);
    let lcd = lcd_widget::new(&win,lcd_pos,0);          // Create the LCD Widget

    let button_count_title = "Count to 1,000,000";      // Set here because we re-used it when we change the button name and then back when 
                                                        // we're counting (see the event loop). 

    // Create a couple buttons.
    //
    // --> The (x,y) values in the buttons, text widgets and other controls are hard-coded 
    //     here to make the code simpler.  
    //
    //      functions such as get_size() for controls can help when using a starting (x,y) base, to
    //      make new controls relative to previous controls.
    //
    // -> center_x() centers the button horizontally in the window

    let button_count = win.new_button_s((20,210),button_count_title,kw::center_x() + kw::width(250));
    let button_fast = win.new_button_s((20,240),"Count to 1,000,000 (Fast Mode)",kw::center_x() + kw::width(250));

    let checkbox_blue = win.new_checkbox_s((0,270), "Blue Led Style",kw::center_x());

    win.write_xy_s((0,170),"Rust LCD Widget Emulation Example",kw::center_x());

    // Main event loop
    //
    // - Looks for button presses and such. 
    // - Until events are received, the thread is suspended and the program is not running in the foreground (using no cpu cycles)
    // - get_event() will return true until the user closes the window (if user-close is enable, which is not the default - set with win.enable_close())
    //   or a system sends a message that it is going to close it.
    //
    while win.get_event() && !win.close_button_pressed()
    {
        // If the 'Blue Led Style' checkbox was pressed, change the mode (or back if it was already checked)

        if checkbox_blue.pressed() { lcd.set_display_mode(if checkbox_blue.checked() { LcdMode::BlueLcd } else { LcdMode::PlainLcd }); }

        // If the 'count' button was pressed, count from 0 to 1,000,000
        // --> This mode refreshes the display on each pass, so takes longer because it has to 
        //     update the window 1,000,000 times.  This shows the display acting like a typical counting LCD display
        //
        // --> The text and color of the button is changed while it is counting, and then changed back when finished.
        //
        if button_count.pressed()
        {
            button_count.set_text("Press to stop counting");        // Set a new text for the button while counting
            button_count.set_style("red");                          // Make the button color red

            for i in 0..1000000
            {
                lcd.set_value(i);               
                if button_count.pressed() { break; }                // Discontinue of the button was pressed
            }
            button_count.set_text(button_count_title);              // Put the original button title back
            button_count.set_style("default");                      // ...and the original color
        }

        // If the 'count (fast mode)' button was pressed, count in 'fastmode'
        // --> Fastmode will only update the window every 60 fps or so, so it doesn't need to update
        //     on every call.  
        //
        //     This basically causes the count to happen more-or-less instantly, in just a few milliseconds
        //
        // --> The function UpdateLast() is called to ensure the update is current to the current count.
        //
        if button_fast.pressed()
        {
            lcd.set_fast_mode(true);                        // Set the LCD to only update every 10-20 ms. 
            for i in 0..1000000 { lcd.set_value(i); }       // Count to 1,000,000
            lcd.set_fast_mode(false);                       // Reset the 'fast mode' display status
            lcd.update_last();                              // Update the last value to the window to ensure it is displayed.
        }
    }
    Sagebox::exit_button();     // Since we were in an event loop waiting for the window to close, this function is not needed.
                                // But, it is always nice to get a visible indication the program intended to end, so a button to press
                                // lets the user know everything is as intended, rather than closing down the window and returning the OS 
                                // suddenly. 
}
