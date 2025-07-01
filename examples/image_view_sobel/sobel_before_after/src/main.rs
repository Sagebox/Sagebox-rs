
// -----------------------------------------------------
// ImageView - Sobel Edge Before and After Image Example
// -----------------------------------------------------
//
// This program shows a simple use of the image_view_before_after() function in Sagebox that allows
// a bitmap image or vector array-based image (color or monochrome) before & after to be viewed with just one line of code, 
// or displayed and further managed with a returned object. 
//
// -------------------------
// About ImageView Functions
// -------------------------
//
// Sagebox ImageView functions (e.g. image_view(), image_view_before_after()) allow an easy, but powerful way to 
// look at image data, which can be: 
//
//   - Loaded or derived Bitmaps
//   - Any image in Bitmap or Vec<> format
//     Vector formats can be u8, f32, i32, (u8,u8,u8), (f32,f32,f32), (i32,i32,i32), or
//     32-bit equivalents, such as (u8,u8,u8,u8), where the last component is an alpha channel.
//
// The Image View functions are meant to be generalized function supporting in-program created content (e.g. a Vec<> of just about any type), or a loaded
// (and possibly altereed) image.
//
// --------------------------------------
// Powerful Functions in one line of code
// --------------------------------------
//
// It just takes one line of code to activate and use an Image View Window, which can do the following: 
//
// - The Image View functions include the ablity to resize the image, zoom in and out, and move the image about in the window.
// - A navigator (eka. Zoom Box) window can be called up within the window (or dispayed with a keyword in the image_view() call) to help 
//   moving about in the window
// - Multiple Image View windows may be opened at one time.
// - WIndows are not required to be modal - the Sagebox internal system manages the windows independently and will close them when the user
//   closes them or the program ends. 
// - An object is returned.  It is not required to save the object, as the Image View windows survive object deletion.
// - With the return object, more power is handed to the program to close, hide, update, or move the window.
// - Captions and titles may be added to the images in different font styles.
//
// ---------------------------------------------------------------------------------
// This Example - Sobel Edge Image Displayed with Sagebox::image_view_before_after()
// ---------------------------------------------------------------------------------
//
// This example uses another Sagebox function to open the OS's image browser, to select an image to display. 
// 
// The image is then loaded which is passed to the sobel() function, which returns a color Vec<u8>
// The image is then displayed through the image_view_before_after() function.
//

use sagebox::*; 

fn main() 
{
    // Bring up the Open File Dialog. 
    // 
    // - This call limits the types to Jpeg, Bmp, and Png.  Without the string, all files are shown for selection
    // - get_open_filename() returns with None of the file canceled.  Otherwise it contains the String with the selected filename

    let bitmap_name = Sagebox::get_open_filename_s("*.jpg;*.bmp;*.png"); 
    let bitmap_file = bitmap_name.unwrap_or(String::new());

    // Read the image file.  If the file does not exist, then it just created an empty bitmap. 
    // - Passing in a potentially empty string or bad filename is done on purpose, is it just falls through with the empty bitmap, which
    //   can then be checked with bitmap.is_valid() or bitmap_is_empty().
    //
    let bitmap = Sagebox::read_image_file(bitmap_file.as_str());    // Will just fall through with an empty bitmap if the string is empty. 

    // If we didn't find the bitmap, skip this section and then let the user know about it
    //
    // --> We can just drop through here, all functions will just disregard the empty bitmap and fall-through
    //     (e.g. image_view() will just return with an empty image view object, which can be used passively, ignoring
    //     all functions, or checking the return object with ImageView::is_valid()
    //
    if bitmap.is_valid()
    {
        let sobel_vec = sobel::sobel(&bitmap);     // Create the edge image

        // Note: the sobel_vec() image is temporarily converted to a bitmap image, as
        //       the image_view() functions that accept a standard vector have not yet been put into the sagebox library interface just yet. 
        // 
        // In future examples, the vector may be passed to the image_view_vec() function directly, or perhaps 
        // via a trait with the existing image_view() function. 

        let bitmap2 = Bitmap::from_vec(bitmap.size(),&sobel_vec);   // Create a bitmap from the returned vector. 
                                                                    // This function returns an RGB bitmap. 

        // Note that we don't save the returned object in the image_view_before_after() call below.
        // We can use the returned object for many things, such as waiting for the window to close, and other functions.
        //
        // In this case, we set the Kw::wait_for_close() keyword, which waits for the user to close the window. 
        // (In which case we don't need the returned object for anything)
        //
        // As noted above, if we want to follow-up, we could assign the object so something like 'my_view' and check my_view.is_valid()
        // to make sure the image_view_before_after() call below accepted and displayed the bitmap, instead of returning directly due to an invalid bitmap. 

        // Display the image view window with the Original Image and Sobel Result
        //
        // - The set_caption() and label_font() calls set up a caption for the before and after images, but aren't required
        // - Kw::wait_for_close() waits for the user to close the window before returning.
        //   - otherwise the object returned can be used by the program to manage the window.

        Sagebox::image_view_before_after_s(&bitmap,&bitmap2,kw::set_caption("Original Image\nSobel Edge Detection Result") + kw::wait_for_close());
     
        Sagebox::exit_button();  // Let the user know we ended the program.
                                // We don't really need this since the user just closed the window manually, but it's a 
                                // nice-to-have.
    }
    else 
    { 
        // We can further refine this to determine if the cause was the image file was not found, or it was found and wasn't a
        // compatible image type -- we can do this by checking if bitmap_name.is_some(), and if true, then we found the file and it was
        // the wrong type of file. 
        //
        // For now, we combine it into one generalized error, but don't print it if the user cancelled getting the file altogether, by
        // checking the length of the bitmap_file to determine if the get_open_filename() function returned with a string or None

        if bitmap_file.len() > 0 { Sagebox::info_window(format!("Image Not Found.\n'{}'\n was not found or was not a valid image file.",bitmap_file)); }
        else { println!("Sobel Edge Detection canceled."); }
        
    }
   

}
mod sobel;