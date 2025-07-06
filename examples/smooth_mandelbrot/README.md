# Rust Smooth Mandelbrot Program 

This program is an example of a simple pure graphics program using Sagebox in Rust. 
This example is also a good example of showing how Sagebox code can be added on top of 
existing code with just a few lines, without requiring changing any coding or data structure
inside of the code itself.  

This program draws a Smooth Mandelbrot, which smooths out the colors that are otherwise jagged in a plain Mandelbrot. 

> **Note:** In debug mode, the Mandelbrot calculation can take anywhere from 500ms to 2 seconds. In release mode, the Mabdelbrot calculation only takes a few ms, and the result appears instantly. 

Most of the program is just the Mandelbrot code itself, with just 5 lines of Sagebox Code:

1. Create the Window
2. Clear the window to black (which really isn't necessary -- see notes there)
3. One line to draw the pixel for each pixel in the array
4. One line to draw the text label on the top of the window in a large font
5. One line to bring up an 'Exit Button' to allow the user to press a buttom before the program goes back to the OS (since it just draws and exits). 

There are 4 other lines of code that use the Sagebox library to print to the Console Window, but are not graphics related.

- These lines of code use Sagebox::console_write() to field colors within "{}", such as "{green}" (or {g} for short), which will turn the text color to green until the end of the text line, or an empty "{}" is seen.
- At the end of each text line, Sagebox returns the console mode to the text color that was active before it printed the line.
- Background colors may also be set, such as "{bg=blue}"

## About this Example

This example is a great example of just having fun with graphics, either for learning purposes or just for fun. 

- The Mandelbrot code takes a small array of 17 colors and converts it to a 1024-size array to make the smooth colors.
- Then two log() functions are used to make the color smooth based on it's strength factor.
  - I'd explain it here, but I didn't write the code:
  - This code can be found in various places on the Internet. 
- Most of the code is the Mandelbrot code itself vs a few lines of Sagebox function calls
- Sagebox was adapted to display it in roughly 3 lines of code (plus some additives to make it nice, such as the top text label)
