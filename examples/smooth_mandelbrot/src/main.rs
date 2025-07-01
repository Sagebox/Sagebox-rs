
// ------------------------------
// Rust Smooth Mandelbrot Program 
// ------------------------------
// 
// This program is an example of a simple pure graphics program using Sagebox in Rust. 
// This example is also a good example of showing how Sagebox code can be added on top of 
// existing code with just a few lines, without requiring changing any coding or data structure
// inside of the code itself.  
//
// This program draws a Smooth Mandelbrot, which smooths out the colors that are otherwise jagged in a plain Mandelbrot. 
//
// Note --> In debug mode, the Mandelbrot calculation can take anywhere from 500ms to a second.
//          In release mode, the Mabdelbrot calculation only takes a few ms, and the result appears instantly. 
//
// Most of the program is just the Mandelbrot code itself, with just 5 lines of Sagebox Code:
//
// 		1. Create the Window
// 		2. Clear the window to black (which really isn't necessary -- see notes there)
// 		3. One line to draw the pixel for each pixel in the array
//      4. One line to draw the text label on the top of the window in a large font
// 		5. One line to bring up an 'Exit Button' to allow the user to press a buttom before the 
//   	   program goes back to the OS (since it just draws and exits). 
//
// There are 4 other lines of code that use the Sagebox library to print to the Console Window, but are not graphics related.
//
//    - These lines of code use Sagebox::console_write() to field colors within "{}", such as "{green}" (or {g} for short), which
//      will turn the text color to green until the end of the text line, or an empty "{}" is seen.
//    - At the end of each text line, Sagebox returns the console mode to the text color that was active before it printed the line.
//    - Background colors may also be set, such as "{bg=blue}"
// 
// ------------------
// About this Example
// ------------------
//
// This example is a great example of just having fun with graphics, either for learning purposes or just for fun. 
//
// - The Mandelbrot code takes a small array of 17 colors and converts it to a 1024-size array to make the smooth colors.
// - Then two log() functions are used to make the color smooth based on it's strength factor.
//   - I'd explain it here, but I didn't write the code:
//   - This code can be found in various places on the Internet. 
// 
// As mentioned, most of the code is the Mandelbrot code itself
//   
// - Sagebox was adapted to display it in roughly 3 lines of code (plus some additives to make it nice, such as the top text label)
//
extern crate num; 
use num::complex::Complex;

use sagebox::*;


// Basic 17-value color table
// Stretched out late with create_color_table();
//
fn create_small_color_table() -> Vec<(i32,i32,i32)>
{
	vec![ 	( 0  , 0  ,  0  ), ( 25 , 7  , 26  ), ( 9  , 1  , 47  ), ( 4  , 4  , 73  ),
			( 0  , 7  , 100 ), ( 12 , 44 , 138 ), ( 24 , 82 , 177 ), ( 57 , 125, 209 ),
			( 134, 181, 229 ), ( 211, 236, 248 ), ( 241, 233, 191 ), ( 248, 201, 95  ),
			( 255, 170, 0   ), ( 204, 128, 0   ), ( 153, 87 , 0   ), ( 106, 52 , 3   ), 		
			( 106, 52 , 3   ), 		]
}	

/// Create a smooth transition between the 16 colors above into a 1024-value table. 
///
pub fn create_big_color_table(color_table : &mut Vec<(i32,i32,i32)>,colors : &Vec<(i32,i32,i32)>)
{
	for i in 0..16
	{
		for j in 0..64
		{
			let percent = 16.0*j as f32 / 1024.0; 	// Get percent of current color (i) vs. next color (i+1)

			// get_color_value() - this macro calculates the color by percentage of the current and next color, creating
			//                     the smooth transition between colors.
			//
			// $i - is the current index, which also looks at i+1 to get the percentage between the two colors
			// $index for red, green, and blue, respectively, e.g. colors[i].1 = green pixel color.

			macro_rules! get_color_value { ($i:ident,$index:tt) => { (colors[$i].$index as f32 * (1.0-percent) +
																					colors[$i+1].$index as f32 * percent) as i32 } }

			// Put in the RGB color by calling get_color_value() for red, green, and blue (e.g. 0, 1 and 2)
			
			color_table[1024 * i / 16 + j] = (get_color_value!(i,0),get_color_value!(i,1),get_color_value!(i,2));
		}
	}
}


/// -----------------
/// Main Program Loop
/// -----------------
///
/// This is a simple procedural program that does everything in the main() function, rather
/// than creating a structure and implementation.
///
/// As a refactoring step, as the program grows, then it would be split into a Mandelbrot implementation
///
pub fn main()
{
	let colors = create_small_color_table();	// 16 colors

	let mut color_table = vec![(0,0,0); 1024]; // 1024-color color table

	create_big_color_table(&mut color_table,&colors);	// Create the smooth color 1024-sized table from 17 colors. 

	let win = Sagebox::new_window();	// Create and display a default window

	win.cls_s("black"); 	// Give the window a black background. Not necessary, but just to have it black when
							// calculating in debug mode (release mode is basically instant). 
							
	let range    = 3.7;		// Initial Range (i.e. "zoom" factor)
	let max_iter = 50;      // Max Mandelbrot Iterations
	
	let disp_size = win.get_window_size();

	let f_center  = (-0.6,0.0); 	// center of display area in Mandelbrot coordinates

	// math using raw (i32,i32) and (f32,f32) -- can be cleaned up to use struct-based math with 
	// various point types. e.g. Sagebox::Point<> or from any library or mod file. 

	let f_range   = (range,range*disp_size.y as f32/disp_size.x as f32);
	let fd 		  = (f_range.0/disp_size.x as f32,f_range.1/disp_size.y as f32); 
	let f_start   = (f_center.0-fd.0*disp_size.x as f32/2.0,f_center.1-fd.1*disp_size.y as f32/2.0); 

	// Print out some color text information to the console window. 

	Sagebox::console_writeln("{cyan}Starting Mandelbrot");
	Sagebox::console_writeln(format!("{{yellow}}--> Display Calculation Size = {{g}}{} x {}.",disp_size.x,disp_size.y));

	// Main Mandelbrot Loop

	for i in 0..disp_size.y
	{
		let fy = i as f32 * fd.1 + f_start.1;	// Get start Y based on our center f_center
		for j in 0..disp_size.x
		{
			let fx = j as f32 * fd.0 + f_start.0;	// Get start X based on center in f_center
	
			let mut iter  = 0; 
	
			let c = Complex::new(fx,fy);
			let mut z = c;
	
			// Mandelbrot main calcultion -- same as any other, i.e. z = z^2+c, is all we're doing.
			// **note: z.norm() is absolute value. Not called z.abs() in rust. 

			while z.norm() < 256.0 && iter < max_iter-1	 
			{
				z = z*z + c;
				iter += 1;
	
			}		
			let mut rgb_color = (0,0,0); 	// Start with black if we're over the max_iter

			// If we have a value, then make a smooth transition between colors
			// using log2() functions and the 1024-value color table we created

			if iter < max_iter-1
			{
				let f_log = f32::log2(f32::log2(z.norm_sqr())/2.0);
				let mut f_iter = iter as f32 + 1.0 - f_log;
				f_iter /= max_iter as f32 - 1.0; 
								
				// Get the index from the 1024-value color table

				let index = f32::min((1024-1) as f32*f_iter.sqrt(),1023 as f32);	
				rgb_color = color_table[index as usize];	// Output color based on iteration value

			}

			win.set_pixel_rgb(j,i,rgb_color);

		}
	}

	Sagebox::console_writeln("{c}Finished Calulating Smooth Mandelbrot!");

	// Put a label centered in the top of the screen in a 50-point font
	// - pad_y() brings the text down 20 pixels in the window so it is not pressed against the top too much. 

	win.write_s("{50}Rust Smooth Mandelbrot",kw::top_center() + kw::pad_y(20));
	
	Sagebox::exit_button();		// Display a button to let the program know the program is over and press an "OK" button.
								// This keeps the window from being closed as the program ends and returns the OS, since it
								// just displays the Mandelbrot and the program ends.
								//
								// In programs with interactivity, this isn't needed, since the user would choose when to end the program.


}

