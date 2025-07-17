

# About ImageView Functions

Sagebox ImageView functions (e.g. image_view(), image_view_before_after()) allow an easy, but powerful way to 
look at image data, which can be: 

  - Loaded or derived Bitmaps
  - Any image in Bitmap or Vec<> format
    Vector formats can be u8, f32, i32, (u8,u8,u8), (f32,f32,f32), (i32,i32,i32), or
    32-bit equivalents, such as (u8,u8,u8,u8), where the last component is an alpha channel.

The Image View functions are meant to be generalized function supporting in-program created content (e.g. a Vec<> of just about any type), or a loaded
(and possibly altereed) image.

## Powerful Functions in one line of code

It just takes one line of code to activate and use an Image View Window, which can do the following: 

- The Image View functions include the ablity to resize the image, zoom in and out, and move the image about in the window.
- A navigator (eka. Zoom Box) window can be called up within the window (or dispayed with a keyword in the image_view() call) to help 
  moving about in the window
- Multiple Image View windows may be opened at one time.
- WIndows are not required to be modal - the Sagebox internal system manages the windows independently and will close them when the user
  closes them or the program ends. 
- An object is returned.  It is not required to save the object, as the Image View windows survive object deletion.
- With the return object, more power is handed to the program to close, hide, update, or move the window.
- Captions and titles may be added to the images in different font styles.



## Sobel Edge Image Example (`sobel_image_view`)

This program shows a simple use of the image_view() function in Sagebox that allows
a bitmap or vector array (color or monochrome) to be viewed with just one line of code, 
or displayed and further managed with a returned object. 


## Sobel Edge Before and After Image Example (`sobel_before_after`)

This program shows a simple use of the image_view_before_after() function in Sagebox that allows
a bitmap image or vector array-based image (color or monochrome) before & after to be viewed with just one line of code, 
or displayed and further managed with a returned object. 

## Watch the Demo on Youtube

[One-Line Image View Functions — Github Demo | GUI Programming in Rust](https://youtu.be/yJyATaCLZ9U)

