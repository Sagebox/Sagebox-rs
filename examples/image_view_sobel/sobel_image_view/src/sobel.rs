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


    let mut bitmap : Vec<u8> = vec![0; (bitmap_in.width()*bitmap_in.height()) as usize];

    let width = bitmap_in.width();
    let height = bitmap_in.height();
        
    let norm = f32::sqrt(2.0);      // Nomralization for a (a^2 + b^2) equation used below. 

    let mut bitmap_iter = bitmap.iter_mut().skip(width as usize);   // Start at line 1, since we're not looking at outermost image edges.

    // Calculate the edges in two inner loops
    //  - note that it does not look at the edges to avoid 
    //    edge-condtional logic (since this is just an example)
    //  - This can be adapted easily, if we want to updgade it.

    for i in 1..(height-1) as usize
    {
        bitmap_iter.next(); // Move to the second pixel, since we're not checking outermost edges
        for j in 1..(width-1) as usize
        {   
            let mut gray_v = 0.0 as f32;
            let mut gray_h = 0.0 as f32; 

            // Set a macro to get the data we want so it's inlined for performance. 
            //
            // --> get_pixel() is being used here since we're looking at a bitmap.
            //     If the incoming data were a vector, we would look at it directly, or perhaps through an iterator.

            macro_rules! get_gray {  ($array:ident,$k:ident) => { ($array[$k][2]*bitmap_in.get_pixel(j as i32+ $array[$k][0],i as i32+$array[$k][1]).soft_gray()) as f32 } } 

            // Look at all edge conditions for each element int he horizontal and vertical arrays, respectively.

            for k in 0..6 
            {
                gray_v += get_gray!(array_v,k);
                gray_h += get_gray!(array_h,k); 
            }
                let mag = (f32::sqrt(gray_v*gray_v+gray_h*gray_h)/norm).min(255.0) as u8;
                
                *bitmap_iter.next().unwrap() = mag; // Set the output value
        }
        bitmap_iter.next(); // Since we're not checking the edge, we need to move to the next line by going to one more pixel in this row.
    }
    bitmap
}
