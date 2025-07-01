use sagebox::Bitmap;  

/// [sobel()] - create a monocrhome sobel edge image
///
///  - For demonstration purposes, the input is a Bitmap. 
///  - The bitmap can be converted to a Vec<u8>, Vec<i32> or Vec<f32> prior to 
/// calling this function, allowing the sobel() function to accept a generic array. 
/// - The code in this function is a generic sobel implementation
/// - This function returns a monochrome **Vec&lt;u8>** with the edge data (0-255 values)
///
///  note:
///
/// - This function does not calculate the outermost edges of the incoming data, just to keep it simple.
/// - Currently the input is a Vec<u8>, which contains data for an unaligned color bitmap, so each there are
///   3 entries for each R,G,B color
/// - In later examples, this will be converted to Vec<(u8,u8,u8)> to make it easier to use the iterator.
///
pub fn sobel(bitmap_in : &Bitmap) -> Vec<u8>
{
    if !bitmap_in.is_valid() { return vec![]; }

    let array_v : [[i32; 3]; 6] = [ [ -1,-1, 1 ],
                                    [ -1, 0, 2 ],
                                    [ -1, 1, 1 ],
                                    [  1,-1,-1 ],
                                    [  1, 0,-2 ],
                                    [  1, 1,-1 ] ];

    let  array_h :  [[i32; 3]; 6] = [ [ -1,-1, 1 ],
                                      [  0,-1, 2 ],
                                      [  1,-1, 1 ],
                                      [ -1, 1,-1 ],
                                      [  0, 1,-2 ],
                                      [  1, 1,-1 ] ];


    let mut bitmap : Vec<u8> = vec![0; (bitmap_in.width()*bitmap_in.height()*3) as usize];

    let width = bitmap_in.width();
    let height = bitmap_in.height();

    let mut bitmap_iter = bitmap.iter_mut().skip((width*3) as usize);   // Start at line 1, since we're not looking at outermost image edges.

    // Calculate the edges in two inner loops
    //  - note that it does not look at the edges to avoid 
    //    edge-condtional logic (since this is just an example)
    //  - This can be adapted easily, if we want to updgade it.

    for i in 1..(height-1) as usize
    {
        bitmap_iter.nth(2); // Move to the second pixel (3 places), since we're not checking outermost edges
     
        for j in 1..(width-1) as usize
        {   
            let mut gray_v = 0.0 as f32;
            let mut gray_h = 0.0 as f32; 

            // Set a macro to get the data we want so it's inlined for performance. 
            //
            // --> get_pixel() is being used here since we're looking at a bitmap.
            //     If the incoming data were a vector, we would look at it directly, or perhaps through an iterator.

            macro_rules! get_gray {  ($array:ident,$k:ident) => { ($array[$k][2]*bitmap_in.get_pixel(j as i32+ $array[$k][0],i as i32+$array[$k][1]).soft_gray()) as f32 } } 

            // Look at all edge conditions for each element int the horizontal and vertical arrays, respectively.

            for k in 0..6 
            {
                gray_v += get_gray!(array_v,k);
                gray_h += get_gray!(array_h,k); 
            }

                // Multiplied by 1.3 (and not normalized) to keep it a little brighter

                let mag = ((f32::sqrt(gray_v*gray_v+gray_h*gray_h))*1.3)/255.0;
                
                let mut pixel   = bitmap_in.get_pixel(j as i32,i as i32);   // Get center pixel (actual pixel we're looking at)
                pixel.red   = ((pixel.red as f32 * mag) as i32).min(255);   // Set colors - Cap overflow at 255
                pixel.green = ((pixel.green as f32 * mag) as i32).min(255);
                pixel.blue  = ((pixel.blue as f32 * mag) as i32).min(255);


                // Set pixels in our Vec<u8>, so we need to set 3 entries for each color

                *bitmap_iter.next().unwrap() = pixel.blue as u8;
                *bitmap_iter.next().unwrap() = pixel.green as u8;
                *bitmap_iter.next().unwrap() = pixel.red as u8;
        }
        bitmap_iter.nth(2); // Since we're not checking the edge, we need to move to the next line by going to one more pixel in this row.
                            // moves three places for R,G,B
    
    }
    bitmap
}
