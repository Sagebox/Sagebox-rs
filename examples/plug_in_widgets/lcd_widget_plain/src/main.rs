
use sagebox::*; 

// --------------------
// About the LCD Widget
// --------------------
//
// This program is an example of using plug-in widgets that anyone can write for Sagebox.
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
// --------------------------------------------------
// This Example - Plain and Simple LCD Widget Example
// --------------------------------------------------
//
// This example simply launches an LCD Widget and counts to 1,000,000
// Since the set_value() call is updating on every call, it takes a few seconds to count through entirely.
// 
// - The program ends when the count is finished
// - If the main window is closed before counting is finished, the counting stops and the program exits.
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
// This example widget is fairly simple since it takes no mouse or other input.
//
// See the Dial Widget for an example widget that uses mouse input, button clicks, and is generally more extensive.
//
fn main() 
{
    // Create a new window that is a little bigger than the LCD window
    // - An addition to possibly make is to get the window size from the LCD widget before 
    //   the window creation, so we can just add around the returned size

    let win = Sagebox::new_window_s(kw::size((350,250)));

    win.cls_s("skybluelight,skybluedark");  // Clear window with a nice gradient

    // Write a title below the widget
    //
    // - {13,italic}     - Sets the font to Arial, 13, italic - but only for the segment following.  
    //                     The other text is in the base font (arial, 18)
    // - {lightblue*1.8} - Sets the text color to 'lightblue', the multiplies it by 1.8 to make it brighter
    // - kw::font(18)    - Sets the overall font to Arial,18
    // - kw::center_x()  - Centers the two-line text in the X-plane, keeping the Y location 
    //                     (which is why the X location in write_xy_s is 0, as it isn't used)

    win.write_xy_s((0,170),"Rust LCD Widget Emulation Example\n{13,italic}{lightblue*1.8}(close window to stop counting)",kw::font(18) + kw::center_x());

    let lcd = lcd_widget::new(&win,(20,20),0);  // Create our LCD widget, passing in our window reference, preferred location in the window, 
                                                // and initial value of 0.

    // Write 0-999999 out to the LCD display.  If the window is closed before it finishes, then it exits
    for i in 0..1000000
    {
        lcd.set_value(i);
        if win.close_button_pressed() { break; }    // Exit if the user pressed the close button on the window.
    }

    Sagebox::exit_button();     // Show a 'program is over' button in a window (tailored messages can used with exit_button_s())
                                // - We don't really need this since the program was running for a while with a display
                                //   (it didn't end abruptly), but it's always nice to know the program intended to end itself.
}
