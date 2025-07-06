
# Handle Events -- Example Program to Show Event Handling in Sagebox

This program shows how to enter an event loop and check for events in Sagebox. 

The event loop — typically (but not always) driven by a call to wait_event() — is not strictly necessary to check for events or read control values.

The event loop is used when your program (or a thread) is waiting for something to happen and has nothing else to do until the user changes something like slider position, presses a button, changes text in an input box, moves the mouse, and a number of other things. 

If we just loop looking for things to change, this will use 100% of the processing time.  So `wait_event()` is called to stop the program execution until something happens. 

### For example,without an event loop

```rust
while true
{
    if win.mouse_clicked() { handle_mouse_click(); }
}
```

Continuously looping to check for changes will consume 100% of the CPU.

### With an event loop

```rust
while win.wait_event()
{
    if win.mouse_clicked() { handle_mouse_click(); }
}
```

This puts the program to sleep until an event occurs, then resumes execution so you can respond to it.

In this case, if `win.mouse_clicked()` returns true, then this was one of the events that
 occurred and `handle_mouse_click()` is called.

If `win.mouse_clicked()` returns true, then some other event happened, such as a mouse 
move, a button press, or some other event not being actively handled. 

Either way, the program goes back to sleep until the next event.  

## This Example

This example does the following

- Creates a slider in the dev Window, with a range of 50-1000, and a default value of 300 - Enters an Event Loop
- Draws a filled circle in the middle of the window
- If the mouse button is down, then it changes the center of the circle to the mouse position in the window
- If the mouse is moved (as a returned event), the position of the mouse is displayed in the Sagebox debug Window
- "Hello World" is written, centered in the window with a 100pt font. 


## Keyword Usage

This example provides a good introduction to using keywords.

- The slider can be created with just `dev_slider("Radius")`. 
  - The function `dev_slider_s()` is used to add keywords to set the range and default value.
- The function `write_s()` is similarly used with keywords to set the font size and to center the text in the window.

See comments in the `main()` function for more details. 
