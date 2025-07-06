// ------------------------------------------------------------------
// Handle Events -- Example Program to Show Event Handling in Sagebox
// ------------------------------------------------------------------
//
// This program shows how to enter an event loop and check for events in Sagebox. 
// 
// The event loop, which calls a function called wait_event() is not strictly 
// necessary to check for events or read control values.
// 
// The event loop is used when your program (or a thread) is waiting for something 
// to happen and has nothing else to do until the user changes something like slider
// position, presses a button, changes text in an input box, moves the mouse, and a 
// number of other things. 
// 
// If we just loop looking for things to change, this will use 100% of the processing 
// time.  So we call wait_event() to stop the program execution until something happens. 
// 
// For example,
// 
//      while true
//      {
//          if win.mouse_clicked() { handle_mouse_click(); }
//      }
// 
// Continuously looping to check for changes will consume 100% of the CPU.
// 
// With an event_loop
// 
//      while win.wait_event()
//      {
//          if win.mouse_clicked() { handle_mouse_click(); }
//      }
// 
// This puts the program to sleep until an event occurs, then resumes execution so 
// you can respond to it. 
//
// In this case, if win.mouse_clicked() returns true, then this was one of the events that
//  occurred and handle_mouse_click() is called.
// 
// If win.mouse_clicked() returns true, then some other event happened, such as a mouse 
// move, a button press, or some other event not being actively handled. 
// 
// Either way, the program goes back to sleep until the next event.  
// 
// ------------
// This Example
// ------------
// 
// This example does the following
// 
//      - Creates a slider in the dev Window, with a range of 50-1000, and a default value of 300
//      - Enters an Event Loop
//      - Draws a filled circle in the middle of the window
//      - If the mouse button is down, then it changes the center of the circle to the mouse
//        position in the window
//      - If the mouse is moved (as a returned event), the position of the mouse is displayed in
//        the Sagebox debug Window
//      - "Hello World" is written, centered in the window with a 100pt font. 
// 
// -------------
// Keyword Usage
// -------------
// 
// This example provides a good introduction to using keywords.
// 
//      - The slider can be created with just dev_slider("Radius"). 
//        - The function dev_slider_s() is used to add keywords to set the range and 
//          default value.
//      - The function write_s() is similarly used with keywords to set the font size and to 
//        center the text in the window.
// 
// See comments in the main() function for more details. 

use sagebox::*; 

fn main() 
{
    let win = Sagebox::new_window();    // Create window of default size, backgound color, font, etc.

    // Create a slider. --> We can simply use dev_slider("Radius") for a default slider, but 
    //                      Here we set the range and the starting value using keywords. 

    let radius_slider = Sagebox::dev_slider_s("Radius",kw::range((50,1000)) + kw::default(300)); 
   
    let mut cicle_pos = win.get_window_center();    // Get the starting circle position at the center
                                                    // of the window.
    // -------------------
    // Enter an event loop
    // -------------------
    //
    // There are many ways to look at control values or respond to events.
    //
    // One way (below) is to call wait_event() which shuts down the thread
    // Until some event happens (mouse movement, button click, etc.)
    //
    // wait_event() returns true until the user closes the window, or Sagebox
    // receives a system event that is closing down the program.
    //
    // ** note: For most things in the event-loop, processing of controls and mouse functions 
    //          don't look for the event. 
    //
    // There is only one event-related function, mouse_moved(), which returns true when the
    // mouse was moved since the last time it was checked. 
    //
    // This shows that many Graphic Control functions can be used without an event-loop. 
    // 
    // In this case, we don't exactly need an event loop.  It would run and look the same, except for the following:
    //
    // 1. Without an event loop, the program would run in a loop and use 100% of the CPU time needlessly
    // 2. With an event loop, since we only need to react when the user moves the mouse or the slider,
    //    the inner loop is only alive for a few milliseconds at a time, using the CPU very little.
    //    --> In this sense, the other functions implictly use the event-loop to stop the program only
    //        displaying the circle and writing "Hello World" when some event happened, without
    //        caring what the event actually was. 

    while win.wait_event(){
        win.cls();                              // Get a blank window canvas (clears to last clear background)

        let mouse_pos = win.get_mouse_pos();    // Get the current mouse position 

        // Only set the circle center if the mouse-button is down. 

        if win.mouse_button_down() { cicle_pos = mouse_pos; } 
        
        // Look for a mouse_moved() event.  This only returns true when the mouse has moved since the last time
        // we checked.  
        //
        // If the mouse has moved, then print the value to the Sagebox debug window.

        if win.mouse_moved() { 
            Sagebox::debug_writeln(
                    format!("Mouse Move Pos = {{g}}({}.{})",mouse_pos.x,mouse_pos.y)); 
        }

        // Draw the circle 

        let radius = radius_slider.get_pos();           // We can put this in the call. This is here for clarity. 
        win.fill_circle(cicle_pos, radius, "skyblue");

        // write "Hello World in the center" of the screen in a 100-point font
        // --> with just write(), "Hello World" would be written in the current font
        //     (which is usually 12pt) at the current window's write position in the upper-left.
        //
        // note: We can also use "{100}Hello World" to set the font, replacing kw::font(100)
        //
        win.write_s("Hello World",kw::font(100) + kw::center());
    }
}   
