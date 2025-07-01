//! ------------------------
//!/ Sagebox Libary Interface
//! ------------------------
//!
//! ** This files is currently in a prototype/beta stage. 
//! ** It is a small part of Sagebox meant to show Sagebox working in Rust. 
//! ** More will be added soon, and the the mod will be split into
//! ** different files. 
//! 
//! The interface itself is 20% code and 80% documentation of each function.
//!
//! Please feel free to comment on the structure of this file.  It's important that it keeps idomatic with Rust, so 
//! let me know if something is not right or can be better!
//!
//! e-mail me at rob@sagebox.org, or post on the Github project page.

#![allow(improper_ctypes)]	// So "not FFI-safe messages" do not appear

//use ext_func::sage_rust_window_get_gdi;
pub use colors::{pan_color,pan_color_a,sage_color,sage_color_a,RgbColor,RgbColorA };
pub use keywords::*;
pub use point::{Point};


#[repr(C)]
struct VecResult 
{
    data: *mut u8,
    len: usize,
}
struct VecResultBool
{
    data: *mut u8,
    len: usize,
	found : bool,
}
#[repr(C)]
struct CIntPair
{
	x : i32,
	y : i32,
}
// Returned with information as to whether the function was successful / pair found
struct CIntPairBool
{
	x : i32,
	y : i32,
	found : bool,
}
#[repr(C)]
pub struct SageDLLInit
{
	version : i32,
	sagebox : i64,
	paswindow : i64,
	davinci : i64,
}
#[repr(C)]
struct ExtRgbColor { red : i32, green : i32, blue : i32 }

pub static KW_NONE : kw = kw{pointer: 0};	
	
	pub trait SageStringFuncType<StringType>		
	{
		/// [**console_write()** - Write out to the console window.  This is the same as print! or println!() except that colors and formatting may be used: 
		/// 
		/// - Use <i>**{color}**</i> to start a color and <i>**{}**</i> to end the color. Example: <i>**console_write("This is {red}in the color red{}")**</i>
		/// - Multiple colors can be used. Example: <i>**console_writeln("This {c}is in cyan{} and this {r}is in the color red")**</i>
		/// - Use <i>**{x=&lt;number>}**</i> to set a column (does not use closing <i>**{}**</i>): Example: <i>**console_writeln("This {x=40}is at column 40")**</i>
		/// - Use <i>**{bg=&lt;color>}**</i> to set the background color: Example: <i>**console_writeln("This {bg=r}background{} is in red")**</i>
		/// - Use <i>**{lbg=&lt;color>}**</i> at the begining of the line to make the entire line the background color:
		/// 
		/// **Function Parameter Forms:**
		///
		///		console_write(title : &str)    
		///		console_write(title : &String) 
		///		console_write(title : String)  
		///		console_writeln(title : &str)    // console_writeln() adds a new line at the end of the text.
		///		console_writeln(title : &String) 
		///		console_writeln(title : String)  
		///
		///  **Other embedded text commands:**
		/// 
		///   - **{u}** -> Underline
		///   - **{r}** -> Reverse (reverses color and background)
		///   - **{vl}, {vr}, {ht}, {hb}** : Vertical left line, vertical left line, horizontal top line, horizontal bottom line (These can be used to make a box)
		//
		///   - Available Colors: <i>Black, White, Gray, red, green, yellow, blue, cyan, purple/magenta,
		///                     darkred, darkgreen, darkyellow, darkblue, darkcyan, darkpurple/darkmagenta</i>
		/// 
		///   - Abbreviation for Colors: <i>blk (black), w, gry (gray), r, g (green), y, b, c, p, m (magenta), db, dr, dy, db, dc, dp, dm (dark magenta)</i>
		///
		///   - [**Important Note:**]  When using background color, underlines, boxes, etc., Windows can change the rest of the line to the background color if the window size is changed.
		///You can end the line with <i>**{_}**</i> to end the block. <i>**"{bg=b}This is in blue{_}"**</i> vs. <i>**"{bg=b}This is in blue{}"**</i>.  This will prevent this issue with Windows. 
		/// 
		/// **Examples:** 
		///
		///		Sagebox::console_write("This {r}Text is in red{} and {b}this text is in blue")   // the first {} ends the red. The last {} is not needed
		///     Sagebox::console_write("This text {w}{bg=b}is in a blue background with bright white text.") 
		/// 
		fn console_write(msg : StringType) -> bool;

		/// [**console_write()** - Write out to the console window.  This is the same as print! or println!() except that colors and formatting may be used: 
		/// 
		/// - Use <i>**{color}**</i> to start a color and <i>**{}**</i> to end the color. Example: <i>**console_write("This is {red}in the color red{}")**</i>
		/// - Multiple colors can be used. Example: <i>**console_writeln("This {c}is in cyan{} and this {r}is in the color red")**</i>
		/// - Use <i>**{x=&lt;number>}**</i> to set a column (does not use closing <i>**{}**</i>): Example: <i>**console_writeln("This {x=40}is at column 40")**</i>
		/// - Use <i>**{bg=&lt;color>}**</i> to set the background color: Example: <i>**console_writeln("This {bg=r}background{} is in red")**</i>
		/// - Use <i>**{lbg=&lt;color>}**</i> at the begining of the line to make the entire line the background color:
		/// 
		/// **Function Parameter Forms:**
		///
		///		console_write(title : &str)    
		///		console_write(title : &String) 
		///		console_write(title : String)  
		///		console_writeln(title : &str)    // console_writeln() adds a new line at the end of the text.
		///		console_writeln(title : &String) 
		///		console_writeln(title : String)  
		///
		///  **Other embedded text commands:**
		/// 
		///   - **{u}** -> Underline
		///   - **{r}** -> Reverse (reverses color and background)
		///   - **{vl}, {vr}, {ht}, {hb}** : Vertical left line, vertical left line, horizontal top line, horizontal bottom line (These can be used to make a box)
		//
		///   - Available Colors: <i>Black, White, Gray, red, green, yellow, blue, cyan, purple/magenta,
		///                     darkred, darkgreen, darkyellow, darkblue, darkcyan, darkpurple/darkmagenta</i>
		/// 
		///   - Abbreviation for Colors: <i>blk (black), w, gry (gray), r, g (green), y, b, c, p, m (magenta), db, dr, dy, db, dc, dp, dm (dark magenta)</i>
		///
		///   - [**Important Note:**]  When using background color, underlines, boxes, etc., Windows can change the rest of the line to the background color if the window size is changed.
		///You can end the line with <i>**{_}**</i> to end the block. <i>**"{bg=b}This is in blue{_}"**</i> vs. <i>**"{bg=b}This is in blue{}"**</i>.  This will prevent this issue with Windows. 
		/// 
		/// **Examples:** 
		///
		///		Sagebox::console_write("This {r}Text is in red{} and {b}this text is in blue")   // the first {} ends the red. The last {} is not needed
		///     Sagebox::console_write("This text {w}{bg=b}is in a blue background with bright white text.") 
		/// 
		fn console_writeln(msg : StringType) -> bool;

		/// [**debug_write**] - Write a message out to the sagebox debug window. The SageboxDebug window is in the Sagebox Process Window.
		/// 
		/// - When writing to the debug window, the debug window will come up automatically the first time it is written to.
		/// - The debug window can be manually hidden (it will not come up automatically after that)
		/// - The Sagebox debug window is a good place to put debug information so it won't clutter up the console window.
		/// - Each line has a line number and you can scroll through the debug output. 
		/// - To hide and show the debug/Sagebox process window, move the mouse to the upper-right of the monitor and hold for 1/4 of a second.
		///
		/// **Function Parameter Forms:**
		///
		///		debug_write(title : &str)    
		///		debug_write(title : &String) 
		///		debug_write(title : String)  
		///		debug_writeln(title : &str)    // debug_writeln() adds a new line at the end of the text.
		///		debug_writeln(title : &String) 
		///		debug_writeln(title : String)  
		/// 
		/// As with the console output functions, you can use colors to set the color of the output:
		/// 
		/// - Use <i>**{color}**</i> to start a color and <i>**{}**</i> to end the color. 
		///  Example: <i>**Sagebox::debug_writeln("This is {red}in the color red{}")**</i>
		/// - You can use the first lett of the color, and do not need the closing <i>**{} if its at the end of the line:
		///         Example: <i>**Sagebox::debug_writeln("This is {r}in the color red")**</i>
		/// - Multiple colors can be used. Example: <i>**Sagebox::debug_writeln("This {c}is in cyan{} and this {r}is in the color red")**</i>
		/// - **{x=&lt;number>}** to set a column (does not use closing {}): Example: <i>**Sagebox::debug_writeln("This {x=40}is at column 40")**</i>
		/// - **{bg=&lt;color>"}** to set the background color: Example: <i>**Sagebox::debug_writeln("This {bg=r}background{} is in red")**</i>
		/// - **{lbg=&lt;color>}** at the begining of the line to make the entire line the background color: 
		///         Example: <i>**Sagebox::debug_writeln("{lbg=blue}This entire line is in a blue background")**</i>
		/// - **{bold}** or **{bld}** for bold
		/// - **{italic}** or **{i}** for italic
		/// - **{bolditalic}** or **{bi}** for bold and italic
		/// - **{div}** for dividing line (i.e. <i>**debug_write("{div}\n")**</i> 
		/// 
		/// - Available Colors: <i>Black, White, Gray, red, green, yellow, blue, cyan, purple/magenta</i>
		/// - Abbreviation for Colors: <i>w (white), r, g, y, b</i>
		///
		fn debug_write(msg : StringType) -> bool;

		/// [**debug_write**] - Write a message out to the sagebox debug window. The SageboxDebug window is in the Sagebox Process Window.
		/// 
		/// - When writing to the debug window, the debug window will come up automatically the first time it is written to.
		/// - The debug window can be manually hidden (it will not come up automatically after that)
		/// - The Sagebox debug window is a good place to put debug information so it won't clutter up the console window.
		/// - Each line has a line number and you can scroll through the debug output. 
		/// - To hide and show the debug/Sagebox process window, move the mouse to the upper-right of the monitor and hold for 1/4 of a second.
		///
		/// **Function Parameter Forms:**
		///
		///		debug_write(title : &str)    
		///		debug_write(title : &String) 
		///		debug_write(title : String)  
		///		debug_writeln(title : &str)    // debug_writeln() adds a new line at the end of the text.
		///		debug_writeln(title : &String) 
		///		debug_writeln(title : String)  
		/// 
		/// As with the console output functions, you can use colors to set the color of the output:
		/// 
		/// - Use <i>**{color}**</i> to start a color and <i>**{}**</i> to end the color. 
		///  Example: <i>**Sagebox::debug_writeln("This is {red}in the color red{}")**</i>
		/// - You can use the first lett of the color, and do not need the closing <i>**{} if its at the end of the line:
		///         Example: <i>**Sagebox::debug_writeln("This is {r}in the color red")**</i>
		/// - Multiple colors can be used. Example: <i>**Sagebox::debug_writeln("This {c}is in cyan{} and this {r}is in the color red")**</i>
		/// - **{x=&lt;number>}** to set a column (does not use closing {}): Example: <i>**Sagebox::debug_writeln("This {x=40}is at column 40")**</i>
		/// - **{bg=&lt;color>"}** to set the background color: Example: <i>**Sagebox::debug_writeln("This {bg=r}background{} is in red")**</i>
		/// - **{lbg=&lt;color>}** at the begining of the line to make the entire line the background color: 
		///         Example: <i>**Sagebox::debug_writeln("{lbg=blue}This entire line is in a blue background")**</i>
		/// - **{bold}** or **{bld}** for bold
		/// - **{italic}** or **{i}** for italic
		/// - **{bolditalic}** or **{bi}** for bold and italic
		/// - **{div}** for dividing line (i.e. <i>**debug_write("{div}\n")**</i> 
		/// 
		/// - Available Colors: <i>**Black, White, Gray, red, green, yellow, blue, cyan, purple/magenta**</i>
		/// - Abbreviation for Colors: <i>**w (white), r, g, y, b, p**</i>
		///
		fn debug_writeln(msg : StringType) -> bool;

		/// [**info_window()**] - Displays a window formatted with the given text and an "OK" button.
		/// 
		/// - The <i>info_window()</> function is used to present general information in a window box. 
		///
		/// **Function Parameter Forms:**
		///
		///		info_window(title : &str)    
		///		info_window(title : &String)
		///		info_window(title : String) 
		/// 	info_window(title : &str,keyw : kw)    	// When using keywords to control or structure the info window.
		///		info_window(title : &String,keyw : kw)
		///		info_window(title : String,keyw : kw) 
		/// 
		/// **Parameters**
		///
		/// - **msg** -- text to put into the window. 
		/// 
		/// **About Dialog Box Text**
		/// 
		/// The text for the dialog box can contain multiple lines.  The first line will be displayed in a larger font, with subsequent lines displayed in a smaller font. 
		/// If the first line in the text starts with a <i>**'+'**</i> the text line becomes the text in the title bar window and is not placed in the dialog box itself. 
		/// 
		/// ### Keyword descriptions TBD. 
		///
		/// Examples:
		/// 
		/// 		Sagebox::info_window("Press OK to continue")
		/// 		Sagebox::info_window("+Information Window\nPress OK to continue");
		/// 		Sagebox::info_window("Finished Processing\nPress OK to continue");
		/// 		Sagebox::info_window("+My Process\nFinished Processing\nPress OK to continue");
		///
		fn info_window(msg : StringType);

		/// [**info_window()**] - Displays a window formatted with the given text and an "OK" button.
		/// 
		/// - The <i>info_window()</> function is used to present general information in a window box. 
		///
		/// **Function Parameter Forms:**
		///
		///		info_window(title : &str)    
		///		info_window(title : &String)
		///		info_window(title : String) 
		/// 	info_window(title : &str,keyw : kw)    	// When using keywords to control or structure the info window.
		///		info_window(title : &String,keyw : kw)
		///		info_window(title : String,keyw : kw) 
		/// 
		/// **Parameters**
		///
		/// - **msg** -- text to put into the window. 
		/// 
		/// **About Dialog Box Text**
		/// 
		/// The text for the dialog box can contain multiple lines.  The first line will be displayed in a larger font, with subsequent lines displayed in a smaller font. 
		/// If the first line in the text starts with a <i>**'+'**</i> the text line becomes the text in the title bar window and is not placed in the dialog box itself. 
		/// 
		/// ### Keyword descriptions TBD. 
		///
		/// Examples:
		/// 
		/// 		Sagebox::info_window("Press OK to continue")
		/// 		Sagebox::info_window("+Information Window\nPress OK to continue");
		/// 		Sagebox::info_window("Finished Processing\nPress OK to continue");
		/// 		Sagebox::info_window("+My Process\nFinished Processing\nPress OK to continue");
		///
		fn info_window_s(msg : StringType,keyw : kw);
	}

	pub trait SageDrawFuncWrite<StringType>
	{
		/// [**write()**] and [**write_xy()**] - Write text out to the window.
		///
		/// The `write()` function writes text to the window and can be used with many options.
		///
		/// #### Forms:
		///
		/// 		write(&str);                             // Basic write() function call
		/// 		write_s(&str, keyword: kw);              // Basic write() with keywords
		/// 		write_xy(location,&str);                 // Write text in a specific location
		/// 		write_xy_s(location,&str, keyword: kw);  // Write text in a specif location (+ keywords)
		///
		/// #### Parameters:
		///
		/// - **location** - With **`write_xy`**, a location specifies where to write the text.
		///   - when using justification keywords, i.e. <i>just_center_x</>, the X or Y value may be 0 if the justification will overrride that value.
		///
		/// ### Basic Examples:
		///
		/// 		window.write("Hello World");**
		///			window.write(&format!("*My Variable is: {MyVariable}*").as_str());**
		///
		/// #### Sagebox keywords can be included. Some available options include:
		///
		/// - **Font** — Set the font to be used for the text.
		/// - **Center, CenterX, CenterY** — Center the text in various ways  
		///   (e.g. `CenterX` centers along the X-axis).
		/// - **TextColor, fgColor** — Set the text (foreground) color.
		/// - **bgColor** — Set the background color.
		/// - **Angle** — Set the rotation angle of the text.
		/// - **Opacity** — Set the opacity level of the text.
		/// - **Pos** — Set the position in the window  
		///   e.g. `Write("Hello World", kw::pos(x, y))`
		///
		/// #### Controls can be embedded in the text to change colors and font sizes.
		///
		/// example:
		///
		/// 		window.write("This *{r}word{}* is in the color red");
		///
		/// - Note the `{}` to close.
		/// - With Rust formatted strings, use double braces:  
		///   e.g. `"MyValue {{r}}{myvalue}{{}} is in red"`
		/// - More than one control can be used, such as:  
		/// **win.write("This is in *{r}Red{}* and this is in *{b}Blue*");**
		/// - You do not need the closing `{}` if it’s at the end of the line.
		///
		/// ### Some Curly-Brace Embedded Controls
		///
		/// - **{<color>}** — A named color like `{red}` or `{r}`, `{blue}`, `{skyblue}`, etc.  
		///   Example: *"this {blue}word{} is in blue"*  
		///   - Abbreviations like `{y}` for `{yellow}` are also supported.
		///
		/// - **{font size}** — Example:  
		///   *"This is in the normal font, and {30}this is in a 30-point font"*
		///
		/// - **{font name}** — Example:  
		///   *"This is in the normal font, and {Courier New,20}This is in a 20-point Courier New font"*
		///
		/// - **{x = <value>}** — Set X position on the line  
		///   Example: *"This value{x=100}starts at position 100 pixels from the left."*  
		///   - Useful for aligning text when fonts have varied character widths.
		///
		/// **Note:** When using `kw::angle()` or `kw::opacity()`, embedded curly-brace options (e.g. `{font}`, `{color}`) are disabled.  
		/// Instead, use keyword options:  
		/// e.g. `write("This is a big font", kw::font(50))`
		///
		/// ### Examples:
		///
		/// 		MyWindow.write_s("Hello World", kw::font(40) + kw::center())  // Writes a large "Hello World" centered in the window.
		///			MyWindow.write_s("Hello World", kw::fg_color("red"))          // Writes "Hello World" in red.
		///			MyWindow.write("{r}Hello World")                              // Also writes "Hello World" in red.
		///			MyWindow.write_s("Hello World", kw::font(50))                 // Writes "Hello World" in a 50-point font.
		///			MyWindow.write("{50}Hello World")                             // Also writes "Hello World" in a 50-point font.
		///
		fn write_s(&self,msg : StringType,keyw : kw ) -> bool;

	/// [**write()**] and [**write_xy()**] - Write text out to the window.
		///
		/// The `write()` function writes text to the window and can be used with many options.
		///
		/// #### Forms:
		///
		/// 		write(&str);                             // Basic write() function call
		/// 		write_s(&str, keyword: kw);              // Basic write() with keywords
		/// 		write_xy(location,&str);                 // Write text in a specific location
		/// 		write_xy_s(location,&str, keyword: kw);  // Write text in a specif location (+ keywords)
		///
		/// #### Parameters:
		///
		/// - **location** - With **`write_xy`**, a location specifies where to write the text.
		///   - when using justification keywords, i.e. <i>just_center_x</>, the X or Y value may be 0 if the justification will overrride that value.
		///
		/// ### Basic Examples:
		///
		/// 		window.write("Hello World");**
		///			window.write(&format!("*My Variable is: {MyVariable}*").as_str());**
		///
		/// #### Sagebox keywords can be included. Some available options include:
		///
		/// - **Font** — Set the font to be used for the text.
		/// - **Center, CenterX, CenterY** — Center the text in various ways  
		///   (e.g. `CenterX` centers along the X-axis).
		/// - **TextColor, fgColor** — Set the text (foreground) color.
		/// - **bgColor** — Set the background color.
		/// - **Angle** — Set the rotation angle of the text.
		/// - **Opacity** — Set the opacity level of the text.
		/// - **Pos** — Set the position in the window  
		///   e.g. `Write("Hello World", kw::pos(x, y))`
		///
		/// #### Controls can be embedded in the text to change colors and font sizes.
		///
		/// example:
		///
		/// 		window.write("This *{r}word{}* is in the color red");
		///
		/// - Note the `{}` to close.
		/// - With Rust formatted strings, use double braces:  
		///   e.g. `"MyValue {{r}}{myvalue}{{}} is in red"`
		/// - More than one control can be used, such as:  
		/// **win.write("This is in *{r}Red{}* and this is in *{b}Blue*");**
		/// - You do not need the closing `{}` if it’s at the end of the line.
		///
		/// ### Some Curly-Brace Embedded Controls
		///
		/// - **{<color>}** — A named color like `{red}` or `{r}`, `{blue}`, `{skyblue}`, etc.  
		///   Example: *"this {blue}word{} is in blue"*  
		///   - Abbreviations like `{y}` for `{yellow}` are also supported.
		///
		/// - **{font size}** — Example:  
		///   *"This is in the normal font, and {30}this is in a 30-point font"*
		///
		/// - **{font name}** — Example:  
		///   *"This is in the normal font, and {Courier New,20}This is in a 20-point Courier New font"*
		///
		/// - **{x = <value>}** — Set X position on the line  
		///   Example: *"This value{x=100}starts at position 100 pixels from the left."*  
		///   - Useful for aligning text when fonts have varied character widths.
		///
		/// **Note:** When using `kw::angle()` or `kw::opacity()`, embedded curly-brace options (e.g. `{font}`, `{color}`) are disabled.  
		/// Instead, use keyword options:  
		/// e.g. `write("This is a big font", kw::font(50))`
		///
		/// ### Examples:
		///
		/// 		MyWindow.write_s("Hello World", kw::font(40) + kw::center())  // Writes a large "Hello World" centered in the window.
		///			MyWindow.write_s("Hello World", kw::fg_color("red"))          // Writes "Hello World" in red.
		///			MyWindow.write("{r}Hello World")                              // Also writes "Hello World" in red.
		///			MyWindow.write_s("Hello World", kw::font(50))                 // Writes "Hello World" in a 50-point font.
		///			MyWindow.write("{50}Hello World")                             // Also writes "Hello World" in a 50-point font.
		///
		fn write(&self,msg : StringType) -> bool;

		/// [**write()**] and [**write_xy()**] - Write text out to the window.
		///
		/// The `write()` function writes text to the window and can be used with many options.
		///
		/// #### Forms:
		///
		/// 		write(&str);                             // Basic write() function call
		/// 		write_s(&str, keyword: kw);              // Basic write() with keywords
		/// 		write_xy(location,&str);                 // Write text in a specific location
		/// 		write_xy_s(location,&str, keyword: kw);  // Write text in a specif location (+ keywords)
		///
		/// #### Parameters:
		///
		/// - **location** - With **`write_xy`**, a location specifies where to write the text.
		///   - when using justification keywords, i.e. <i>just_center_x</>, the X or Y value may be 0 if the justification will overrride that value.
		///
		/// ### Basic Examples:
		///
		/// 		window.write("Hello World");**
		///			window.write(&format!("*My Variable is: {MyVariable}*").as_str());**
		///
		/// #### Sagebox keywords can be included. Some available options include:
		///
		/// - **Font** — Set the font to be used for the text.
		/// - **Center, CenterX, CenterY** — Center the text in various ways  
		///   (e.g. `CenterX` centers along the X-axis).
		/// - **TextColor, fgColor** — Set the text (foreground) color.
		/// - **bgColor** — Set the background color.
		/// - **Angle** — Set the rotation angle of the text.
		/// - **Opacity** — Set the opacity level of the text.
		/// - **Pos** — Set the position in the window  
		///   e.g. `Write("Hello World", kw::pos(x, y))`
		///
		/// #### Controls can be embedded in the text to change colors and font sizes.
		///
		/// example:
		///
		/// 		window.write("This *{r}word{}* is in the color red");
		///
		/// - Note the `{}` to close.
		/// - With Rust formatted strings, use double braces:  
		///   e.g. `"MyValue {{r}}{myvalue}{{}} is in red"`
		/// - More than one control can be used, such as:  
		/// **win.write("This is in *{r}Red{}* and this is in *{b}Blue*");**
		/// - You do not need the closing `{}` if it’s at the end of the line.
		///
		/// ### Some Curly-Brace Embedded Controls
		///
		/// - **{<color>}** — A named color like `{red}` or `{r}`, `{blue}`, `{skyblue}`, etc.  
		///   Example: *"this {blue}word{} is in blue"*  
		///   - Abbreviations like `{y}` for `{yellow}` are also supported.
		///
		/// - **{font size}** — Example:  
		///   *"This is in the normal font, and {30}this is in a 30-point font"*
		///
		/// - **{font name}** — Example:  
		///   *"This is in the normal font, and {Courier New,20}This is in a 20-point Courier New font"*
		///
		/// - **{x = <value>}** — Set X position on the line  
		///   Example: *"This value{x=100}starts at position 100 pixels from the left."*  
		///   - Useful for aligning text when fonts have varied character widths.
		///
		/// **Note:** When using `kw::angle()` or `kw::opacity()`, embedded curly-brace options (e.g. `{font}`, `{color}`) are disabled.  
		/// Instead, use keyword options:  
		/// e.g. `write("This is a big font", kw::font(50))`
		///
		/// ### Examples:
		///
		/// 		MyWindow.write_s("Hello World", kw::font(40) + kw::center())  // Writes a large "Hello World" centered in the window.
		///			MyWindow.write_s("Hello World", kw::fg_color("red"))          // Writes "Hello World" in red.
		///			MyWindow.write("{r}Hello World")                              // Also writes "Hello World" in red.
		///			MyWindow.write_s("Hello World", kw::font(50))                 // Writes "Hello World" in a 50-point font.
		///			MyWindow.write("{50}Hello World")                             // Also writes "Hello World" in a 50-point font.
		///
		fn writeln(&self,msg : StringType) -> bool;

		/// [**write()**] and [**write_xy()**] - Write text out to the window.
		///
		/// The `write()` function writes text to the window and can be used with many options.
		///
		/// #### Forms:
		///
		/// 		write(&str);                             // Basic write() function call
		/// 		write_s(&str, keyword: kw);              // Basic write() with keywords
		/// 		write_xy(location,&str);                 // Write text in a specific location
		/// 		write_xy_s(location,&str, keyword: kw);  // Write text in a specif location (+ keywords)
		///
		/// #### Parameters:
		///
		/// - **location** - With **`write_xy`**, a location specifies where to write the text.
		///   - when using justification keywords, i.e. <i>just_center_x</>, the X or Y value may be 0 if the justification will overrride that value.
		///
		/// ### Basic Examples:
		///
		/// 		window.write("Hello World");**
		///			window.write(&format!("*My Variable is: {MyVariable}*").as_str());**
		///
		/// #### Sagebox keywords can be included. Some available options include:
		///
		/// - **Font** — Set the font to be used for the text.
		/// - **Center, CenterX, CenterY** — Center the text in various ways  
		///   (e.g. `CenterX` centers along the X-axis).
		/// - **TextColor, fgColor** — Set the text (foreground) color.
		/// - **bgColor** — Set the background color.
		/// - **Angle** — Set the rotation angle of the text.
		/// - **Opacity** — Set the opacity level of the text.
		/// - **Pos** — Set the position in the window  
		///   e.g. `Write("Hello World", kw::pos(x, y))`
		///
		/// #### Controls can be embedded in the text to change colors and font sizes.
		///
		/// example:
		///
		/// 		window.write("This *{r}word{}* is in the color red");
		///
		/// - Note the `{}` to close.
		/// - With Rust formatted strings, use double braces:  
		///   e.g. `"MyValue {{r}}{myvalue}{{}} is in red"`
		/// - More than one control can be used, such as:  
		/// **win.write("This is in *{r}Red{}* and this is in *{b}Blue*");**
		/// - You do not need the closing `{}` if it’s at the end of the line.
		///
		/// ### Some Curly-Brace Embedded Controls
		///
		/// - **{<color>}** — A named color like `{red}` or `{r}`, `{blue}`, `{skyblue}`, etc.  
		///   Example: *"this {blue}word{} is in blue"*  
		///   - Abbreviations like `{y}` for `{yellow}` are also supported.
		///
		/// - **{font size}** — Example:  
		///   *"This is in the normal font, and {30}this is in a 30-point font"*
		///
		/// - **{font name}** — Example:  
		///   *"This is in the normal font, and {Courier New,20}This is in a 20-point Courier New font"*
		///
		/// - **{x = <value>}** — Set X position on the line  
		///   Example: *"This value{x=100}starts at position 100 pixels from the left."*  
		///   - Useful for aligning text when fonts have varied character widths.
		///
		/// **Note:** When using `kw::angle()` or `kw::opacity()`, embedded curly-brace options (e.g. `{font}`, `{color}`) are disabled.  
		/// Instead, use keyword options:  
		/// e.g. `write("This is a big font", kw::font(50))`
		///
		/// ### Examples:
		///
		/// 		MyWindow.write_s("Hello World", kw::font(40) + kw::center())  // Writes a large "Hello World" centered in the window.
		///			MyWindow.write_s("Hello World", kw::fg_color("red"))          // Writes "Hello World" in red.
		///			MyWindow.write("{r}Hello World")                              // Also writes "Hello World" in red.
		///			MyWindow.write_s("Hello World", kw::font(50))                 // Writes "Hello World" in a 50-point font.
		///			MyWindow.write("{50}Hello World")                             // Also writes "Hello World" in a 50-point font.
		///
		fn writeln_s(&self,msg : StringType,keyw : kw ) -> bool;

		/// [**write()**] and [**write_xy()**] - Write text out to the window.
		///
		/// The `write()` function writes text to the window and can be used with many options.
		///
		/// #### Forms:
		///
		/// 		write(&str);                             // Basic write() function call
		/// 		write_s(&str, keyword: kw);              // Basic write() with keywords
		/// 		write_xy(location,&str);                 // Write text in a specific location
		/// 		write_xy_s(location,&str, keyword: kw);  // Write text in a specif location (+ keywords)
		///
		/// #### Parameters:
		///
		/// - **location** - With **`write_xy`**, a location specifies where to write the text.
		///   - when using justification keywords, i.e. <i>just_center_x</>, the X or Y value may be 0 if the justification will overrride that value.
		///
		/// ### Basic Examples:
		///
		/// 		window.write("Hello World");**
		///			window.write(&format!("*My Variable is: {MyVariable}*").as_str());**
		///
		/// #### Sagebox keywords can be included. Some available options include:
		///
		/// - **Font** — Set the font to be used for the text.
		/// - **Center, CenterX, CenterY** — Center the text in various ways  
		///   (e.g. `CenterX` centers along the X-axis).
		/// - **TextColor, fgColor** — Set the text (foreground) color.
		/// - **bgColor** — Set the background color.
		/// - **Angle** — Set the rotation angle of the text.
		/// - **Opacity** — Set the opacity level of the text.
		/// - **Pos** — Set the position in the window  
		///   e.g. `Write("Hello World", kw::pos(x, y))`
		///
		/// #### Controls can be embedded in the text to change colors and font sizes.
		///
		/// example:
		///
		/// 		window.write("This *{r}word{}* is in the color red");
		///
		/// - Note the `{}` to close.
		/// - With Rust formatted strings, use double braces:  
		///   e.g. `"MyValue {{r}}{myvalue}{{}} is in red"`
		/// - More than one control can be used, such as:  
		/// **win.write("This is in *{r}Red{}* and this is in *{b}Blue*");**
		/// - You do not need the closing `{}` if it’s at the end of the line.
		///
		/// ### Some Curly-Brace Embedded Controls
		///
		/// - **{<color>}** — A named color like `{red}` or `{r}`, `{blue}`, `{skyblue}`, etc.  
		///   Example: *"this {blue}word{} is in blue"*  
		///   - Abbreviations like `{y}` for `{yellow}` are also supported.
		///
		/// - **{font size}** — Example:  
		///   *"This is in the normal font, and {30}this is in a 30-point font"*
		///
		/// - **{font name}** — Example:  
		///   *"This is in the normal font, and {Courier New,20}This is in a 20-point Courier New font"*
		///
		/// - **{x = <value>}** — Set X position on the line  
		///   Example: *"This value{x=100}starts at position 100 pixels from the left."*  
		///   - Useful for aligning text when fonts have varied character widths.
		///
		/// **Note:** When using `kw::angle()` or `kw::opacity()`, embedded curly-brace options (e.g. `{font}`, `{color}`) are disabled.  
		/// Instead, use keyword options:  
		/// e.g. `write("This is a big font", kw::font(50))`
		///
		/// ### Examples:
		///
		/// 		MyWindow.write_s("Hello World", kw::font(40) + kw::center())  // Writes a large "Hello World" centered in the window.
		///			MyWindow.write_s("Hello World", kw::fg_color("red"))          // Writes "Hello World" in red.
		///			MyWindow.write("{r}Hello World")                              // Also writes "Hello World" in red.
		///			MyWindow.write_s("Hello World", kw::font(50))                 // Writes "Hello World" in a 50-point font.
		///			MyWindow.write("{50}Hello World")                             // Also writes "Hello World" in a 50-point font.
		///
		fn write_xy(&self,location : (i32,i32), msg : StringType) -> bool; 

		/// [**write()**] and [**write_xy()**] - Write text out to the window.
		///
		/// The `write()` function writes text to the window and can be used with many options.
		///
		/// #### Forms:
		///
		/// 		write(&str);                             // Basic write() function call
		/// 		write_s(&str, keyword: kw);              // Basic write() with keywords
		/// 		write_xy(location,&str);                 // Write text in a specific location
		/// 		write_xy_s(location,&str, keyword: kw);  // Write text in a specif location (+ keywords)
		///
		/// #### Parameters:
		///
		/// - **location** - With **`write_xy`**, a location specifies where to write the text.
		///   - when using justification keywords, i.e. <i>just_center_x</>, the X or Y value may be 0 if the justification will overrride that value.
		///
		/// ### Basic Examples:
		///
		/// 		window.write("Hello World");**
		///			window.write(&format!("*My Variable is: {MyVariable}*").as_str());**
		///
		/// #### Sagebox keywords can be included. Some available options include:
		///
		/// - **Font** — Set the font to be used for the text.
		/// - **Center, CenterX, CenterY** — Center the text in various ways  
		///   (e.g. `CenterX` centers along the X-axis).
		/// - **TextColor, fgColor** — Set the text (foreground) color.
		/// - **bgColor** — Set the background color.
		/// - **Angle** — Set the rotation angle of the text.
		/// - **Opacity** — Set the opacity level of the text.
		/// - **Pos** — Set the position in the window  
		///   e.g. `Write("Hello World", kw::pos(x, y))`
		///
		/// #### Controls can be embedded in the text to change colors and font sizes.
		///
		/// example:
		///
		/// 		window.write("This *{r}word{}* is in the color red");
		///
		/// - Note the `{}` to close.
		/// - With Rust formatted strings, use double braces:  
		///   e.g. `"MyValue {{r}}{myvalue}{{}} is in red"`
		/// - More than one control can be used, such as:  
		/// **win.write("This is in *{r}Red{}* and this is in *{b}Blue*");**
		/// - You do not need the closing `{}` if it’s at the end of the line.
		///
		/// ### Some Curly-Brace Embedded Controls
		///
		/// - **{<color>}** — A named color like `{red}` or `{r}`, `{blue}`, `{skyblue}`, etc.  
		///   Example: *"this {blue}word{} is in blue"*  
		///   - Abbreviations like `{y}` for `{yellow}` are also supported.
		///
		/// - **{font size}** — Example:  
		///   *"This is in the normal font, and {30}this is in a 30-point font"*
		///
		/// - **{font name}** — Example:  
		///   *"This is in the normal font, and {Courier New,20}This is in a 20-point Courier New font"*
		///
		/// - **{x = <value>}** — Set X position on the line  
		///   Example: *"This value{x=100}starts at position 100 pixels from the left."*  
		///   - Useful for aligning text when fonts have varied character widths.
		///
		/// **Note:** When using `kw::angle()` or `kw::opacity()`, embedded curly-brace options (e.g. `{font}`, `{color}`) are disabled.  
		/// Instead, use keyword options:  
		/// e.g. `write("This is a big font", kw::font(50))`
		///
		/// ### Examples:
		///
		/// 		MyWindow.write_s("Hello World", kw::font(40) + kw::center())  // Writes a large "Hello World" centered in the window.
		///			MyWindow.write_s("Hello World", kw::fg_color("red"))          // Writes "Hello World" in red.
		///			MyWindow.write("{r}Hello World")                              // Also writes "Hello World" in red.
		///			MyWindow.write_s("Hello World", kw::font(50))                 // Writes "Hello World" in a 50-point font.
		///			MyWindow.write("{50}Hello World")                             // Also writes "Hello World" in a 50-point font.
		///
		fn write_xy_s(&self,location : (i32,i32), msg : StringType,keyw : kw) -> bool; 

	}

	pub trait SageDrawFuncSetPixelRgb<ColorType>
	{
		/// Draws a single RGB pixel in the window at point specified by x,y
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - draw_pixel_rgb(x,y,(i32,i32,i32));   // Set pixel from (i32,i32,i32) rgb value
		///     - draw_pixel_rgb(x,y,RgbColor);        // Set pixel from Sagebox RgbColor rgb value
		///
		/// note:
		/// - string-based colors (e.g. "yellow") are not supported for <i>draw_pixel()</i> to ensure efficiency 
		///(string-parsing at the pixel level is very slow compared to compiled-in rgb values)
		///
		/// - **`draw_pixel()`** and **`set_pixel()`** are the same function
		fn draw_pixel_rgb(&self,x : i32, y: i32,color : ColorType) -> bool;

		/// Draws a single RGB pixel in the window at point specified by x,y
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - draw_pixel_rgb(x,y,(i32,i32,i32));   // Set pixel from (i32,i32,i32) rgb value
		///     - draw_pixel_rgb(x,y,RgbColor);        // Set pixel from Sagebox RgbColor rgb value
		///
		/// note:
		/// - string-based colors (e.g. "yellow") are not supported for <i>draw_pixel()</i> to ensure efficiency 
		///(string-parsing at the pixel level is very slow compared to compiled-in rgb values)
		///
		/// - **`draw_pixel()`** and **`set_pixel()`** are the same function
		fn set_pixel_rgb(&self,x : i32, y: i32,color : ColorType) -> bool;

	}


	pub trait SageNewQuickForm<StringType>
	{
		/// [**quick_form**] - Creates a "QuickForm" window consisting of a Main Window, Application Window (i.e. Canvas), and a Dev Controls window for controls. 
		/// 
		/// - A QuickForm Window puts a Dev Window and General Window together in a unified window to organize using controls and using graphics in the main window. 
		/// - This function returns a QuickForm object that can be used to access and control the created windows.
		/// - The functions Window::new_window() Window::New_dev_window() are effectively called with each window blended in the upper-main window. 
		///
		/// Function Parameter Forms: 
		/// 
		///			quick_form(settings : &str)                 // &str input 
		///			quick_form_s(settings : &str,keyw : kw )    // &str When using keywords
		///			quick_form(settings : &String)              // &String input 
		///			quick_form_s(settings : &String,keyw : kw ) // &String When using keywords
		///
		/// - Note: for a default quick_form, specifiy an empty string, e.g. <i>**Sagebox::quick_form("")**</i>
		///
		/// **Parameters:**
		///
		/// - **settings** - Control string to set various attributes of the Quick Form Window (and/or Dev Window). 
		///                   - These are somewhat TBD, but a partial list: 
		///                     - - **devsize=<i>&lt;value>**</i> - Sets the width of the dev window on the left.
		/// 					- - **small, smaller, wide, wider, widest** - sets the width of the Dev Window on the left accordingly vs. the default width.
		///						- - **hidden** - Hides window (same as <i>kw::hidden()</i> keyword)
		///						- - **realtime** - States the window will be used for real-time graphics (same as kw::realtime() keyword)
		///						- - **font** - Sets base window font (same as <i>kw::font()</i> keyword)
		///						- - **title** - Sets the title of the window, e.g. <i>title="This is the window title"</i>
		///						- - **label** - puts a label in the upper-right corner of the interior window, e.g. <i>label="this is the label"</i>
		///                     - - **GrayBlack,GrayBlue,GrayGray,BlueBlack,BlueGray,BlueBlue,BlackBlack,BlackGray,BlackBlue,Black,Gray,Blue** - various preset color schemes for the Window and Dev Window, the first color as the Dev Window, and the second as the main window area.
		///						- - **&lt;others TBD>**
		///
		/// **Windows Created** 
		/// 
		/// - **main** -  The main window is the top-level window that hold the two (or more) innerwindows, label window, etc. 
		/// - **win** - This is the main 'blank' or 'canvas' window that acts as a normal window.  Items can be drawn in this window, and other window can be embedded within
		///                     - this window (i.e. child_window()).  Any regular window function is available in this window -- it's simply a regular window embedded into the Main window
		/// - **dev**  -- This is a Dev Controls Window that is embedded in the main window, typically to the left of the 'win' window.   With this window, you can easily add controls
		///                      - such as buttons, sliders, small window, list boxes, text widgets, etc. -- all with one line of code. 
		///
		/// **Some Keywords available with quick_form() (partial list):**
		/// 
		/// - **label**      -- Sets a label that will appear in the Main Window above the application window. i.e. (Label="this is my program")
		/// - **resize_ok**  -- Allows the Main window to be resized by dragging the borders.  It can also be maximized. Otherwise, the window cannot be resized by the user.
		/// - **realtime**   -- Sets the main Quick form canvas window as a Realtime window intended to be used for real-time graphics. 
		///              - see <i>kw::realtime()</i> for more information
		/// - **hidden**     - Tells the QuickForm to keep the window hidden when it is created.  The program then must call <i>win.show()</i> to show the window.  
		///                     -The default is to show the window on creation.
		/// - **no_padding**  - for "filled" (default) window type, this removes the few pixels of space between the Win and Dev windows to merge them together with no break.
		///                     - This can be useful when changing the Win background color to a color other than the Main window background color.
		/// - **no_auto_update** - This will cause the Application Window (i.e. Win) to not update.  The program must then use the win.update() function to update any graphics sent to the window.    
		/// - **&lt;more options>**    - See QuickForm documentation for more options available.
		/// 
		/// **Examples:** 
		///      
		/// 	let qf = Sagebox::quick_form("");                   // Default Quick Form Window with no settings
		///
		/// 		let my_button = qf.dev.new_button("Press Me")   // Add a button to the Dev window in the Quick Form Window
		/// 		qf.win.write("Hello World");                      // Write a text message to the interior window in the Quick Form Window
		///
		///     let qf = Sagebox::quick_form_s("",kw::label("This is the label in the window"));  // Sets a label in the upper-right corner if the interior window
		///     let qf = Sagebox::quick_form("label='This is the label in the window'"");         // Same as above, using the text settings input instead.
		///     let qf = Sagebox::quick_form("resizeok,grayblue,title='This is the window title',hidden); 	// Using the text control input instead of keywords
		///
		fn quick_form_s(settings : StringType,keyw : kw ) -> QuickForm;

		/// [**quick_form**] - Creates a "QuickForm" window consisting of a Main Window, Application Window (i.e. Canvas), and a Dev Controls window for controls. 
		/// 
		/// - A QuickForm Window puts a Dev Window and General Window together in a unified window to organize using controls and using graphics in the main window. 
		/// - This function returns a QuickForm object that can be used to access and control the created windows.
		/// - The functions Window::new_window() Window::New_dev_window() are effectively called with each window blended in the upper-main window. 
		///
		/// Function Parameter Forms: 
		/// 
		///			quick_form(settings : &str)                 // &str input 
		///			quick_form_s(settings : &str,keyw : kw )    // &str When using keywords
		///			quick_form(settings : &String)              // &String input 
		///			quick_form_s(settings : &String,keyw : kw ) // &String When using keywords
		///
		/// - Note: for a default quick_form, specifiy an empty string, e.g. <i>**Sagebox::quick_form("")**</i>
		///
		/// **Parameters:**
		///
		/// - **settings** - Control string to set various attributes of the Quick Form Window (and/or Dev Window). 
		///                   - These are somewhat TBD, but a partial list: 
		///                     - - **devsize=<i>&lt;value>**</i> - Sets the width of the dev window on the left.
		/// 					- - **small, smaller, wide, wider, widest** - sets the width of the Dev Window on the left accordingly vs. the default width.
		///						- - **hidden** - Hides window (same as <i>kw::hidden()</i> keyword)
		///						- - **realtime** - States the window will be used for real-time graphics (same as kw::realtime() keyword)
		///						- - **font** - Sets base window font (same as <i>kw::font()</i> keyword)
		///						- - **title** - Sets the title of the window, e.g. <i>title="This is the window title"</i>
		///						- - **label** - puts a label in the upper-right corner of the interior window, e.g. <i>label="this is the label"</i>
		///                     - - **GrayBlack,GrayBlue,GrayGray,BlueBlack,BlueGray,BlueBlue,BlackBlack,BlackGray,BlackBlue,Black,Gray,Blue** - various preset color schemes for the Window and Dev Window, the first color as the Dev Window, and the second as the main window area.
		///						- - **&lt;others TBD>**
		///
		/// **Windows Created** 
		/// 
		/// - **main** -  The main window is the top-level window that hold the two (or more) innerwindows, label window, etc. 
		/// - **win** - This is the main 'blank' or 'canvas' window that acts as a normal window.  Items can be drawn in this window, and other window can be embedded within
		///                     - this window (i.e. child_window()).  Any regular window function is available in this window -- it's simply a regular window embedded into the Main window
		/// - **dev**  -- This is a Dev Controls Window that is embedded in the main window, typically to the left of the 'win' window.   With this window, you can easily add controls
		///                      - such as buttons, sliders, small window, list boxes, text widgets, etc. -- all with one line of code. 
		///
		/// **Some Keywords available with quick_form() (partial list):**
		/// 
		/// - **label**      -- Sets a label that will appear in the Main Window above the application window. i.e. (Label="this is my program")
		/// - **resize_ok**  -- Allows the Main window to be resized by dragging the borders.  It can also be maximized. Otherwise, the window cannot be resized by the user.
		/// - **realtime**   -- Sets the main Quick form canvas window as a Realtime window intended to be used for real-time graphics. 
		///              - see <i>kw::realtime()</i> for more information
		/// - **hidden**     - Tells the QuickForm to keep the window hidden when it is created.  The program then must call <i>win.show()</i> to show the window.  
		///                     -The default is to show the window on creation.
		/// - **no_padding**  - for "filled" (default) window type, this removes the few pixels of space between the Win and Dev windows to merge them together with no break.
		///                     - This can be useful when changing the Win background color to a color other than the Main window background color.
		/// - **no_auto_update** - This will cause the Application Window (i.e. Win) to not update.  The program must then use the win.update() function to update any graphics sent to the window.    
		/// - **&lt;more options>**    - See QuickForm documentation for more options available.
		/// 
		/// **Examples:** 
		///      
		/// 	let qf = Sagebox::quick_form("");                   // Default Quick Form Window with no settings
		///
		/// 		let my_button = qf.dev.new_button("Press Me")   // Add a button to the Dev window in the Quick Form Window
		/// 		qf.win.write("Hello World");                      // Write a text message to the interior window in the Quick Form Window
		///
		///     let qf = Sagebox::quick_form_s("",kw::label("This is the label in the window"));  // Sets a label in the upper-right corner if the interior window
		///     let qf = Sagebox::quick_form("label='This is the label in the window'"");         // Same as above, using the text settings input instead.
		///     let qf = Sagebox::quick_form("resizeok,grayblue,title='This is the window title',hidden); 	// Using the text control input instead of keywords
		///
		fn quick_form(settings : StringType) -> QuickForm;
	}
	

	pub trait SageDrawFuncCls<ColorType>
	{
        /// [**cls**()] - Clears the window to a blank canvas, based on previous color or new color when using cls_s() or cls_grad()
        ///
		/// Function Parameter Forms: 
		/// 
		///			cls()                         // Clear the window to the last cls() color
		///			cls_s(color : RgbColor)       // Clear with an RgbColor, variable, or preset such as pan_color::ForestGreen or sage_color::Blue
		///			cls_s(color : &str)           // Clear with a string slice, e.g. cls("green")
		///			cls_s(color : (i32,i32,i32))  // Clear with an (i32,i32,i32) Rgb Value, e.g. cls((0,255,0))
		///			cls_s(color : &String)        // Clear with a string e.g. cls(my_string)
		///
		/// **Notes**
		///
		/// - using <i>**cls()**</i> with no parameters clears the window to the last known <i>cls() color</i>, which can be a solid color (e.g. black or blue), 
		/// a bitmap, a vertical gradient, a radial gradient, depending on the last cls() that used any parameters. 
		/// - using <i>**cls_s()**</i> with parameter (e.g. <i>**cls_s("green")**</i>) sets the new color for <i>cls()</i> when used with no parameters.
		/// - A vertical gradient can be used when using &str or &String, e.g. <i>**cls("black,blue")**</i>
		/// - See <i>**cls_grad()**</i> to clear the window with a vertical gradient , e.g. <i>**cls_grad_s(pan_color::black,pan_color::blue)**</i>, 
		///when using <i>RgbColor</i> or <i>(i32,i32,i32)</i> Rgb values
		/// - See <i>**cls_radial()**</i> to clear the window with a radial gradient eminating from the center of the window, e.g. <i>**cls_radial("black,darkblue")**</i>
		///
        /// **Examples:**
        ///
        ///	win.cls()                     // Clear the window with the current background color of the window
        ///	win.cls_s("red")              // Clear the window with the color red
        ///	win.cls_s(sage_color::Red)    // Clears the color with red, also
        ///	win.cls_s(pan_color::beige)   // Clears the window with PanColor.ForestGreen
        ///	win.cls_s(my_color)           // Clears the window with a defined my_color, which may be an RgbColor, &str, &String, or (i32,i32,i32)
        ///	win.cls_s((0,255,255)         // Clears the window with a defined "MyColor", which may be an RgbColor, &str, &String, or (i32,i32,i32)
        ///	win.cls_s("black,blue")       // Clears the window with a gradient from black to blue
        ///
		fn cls_s(&self,color : ColorType) -> bool;	


   		/// [**cls_grad**()] - Clears the window to a blank canvas with a vertical gradient (first color at the top of the window, to the second color at the bottom of the window)
        ///
		/// Function Parameter Forms: 
		/// 
		///			cls_grad()                     					
		///			cls_grad(color1 : RgbColor,color1 : RgbColor)   			
		///			cls_grad(color1 : (i32,i32,i32),color2 : (i32,i32.i32))  
		///			cls_grad(color1 : &str,color2 : &str)  
		///			cls_grad(color1 : &String, color2 : &String) 
		///
		/// **Notes**
		///
		/// - In the case of strings, such as <i>**cls_grad("black","blue")**</i>, <i>**ls_s("black,blue")**</i> can be used instead.
		/// - After using <i>**cls_grad()**</i> to create a vertical gradient, the next <i>cls()</i> with no parameters will clear the image to the same gradient.
		/// - See <i>**cls()**</i> to clear the window to a single color
		/// - See <i>**cls_radial()**</i> to clear the window with a radial gradient instead of a vertical gradient.
		///
        /// **Examples:**
        ///
        ///		win.cls_grad(pan_color::black,pan_color::blue)      // Clear the window with vertical gradient from black to blue
        ///		win.cls_grad("black","blue")                        // Clear the window with vertical gradient from black to blue
        ///		win.cls("black,blue")                               // Same as the above, with cls() using a single string instead of two.
		///		win.cls()                                           // Clear the window with the current background color or gradient of the window
        ///
		fn cls_grad(&self,color : ColorType,color2 : ColorType) -> bool;

		/// [**set_fg_color**] - Sets the foreground/text color of the text written to the window.
        /// 
        /// - The function <i>set_**fg_color()**</i> affects subsequent text written to the window with the <i>**write()**</i> or other window write functions.
        /// - Note: <i>**set_fg_color()**</i> and <i>**set_text_color()**</i> are the same function.
		///
		/// **Function Parameter Forms:** 
		/// 
		///			set_fg_color(color : RgbColor)       // Set the text color with an RgbColor, variable, or preset 
		///                                              // ..such as pan_color::ForestGreen or sage_color::Blue
		///			set_fg_color(color : &str)           // Set the text color with a string slice, e.g. set_fg_color("green")
		///			set_fg_color(color : (i32,i32,i32))  // Set the text color with an (i32,i32,i32) RGB value, e.g. set_fg_color((0,255,0))
		///			set_fg_color(color : &String)        // Set the text color with a string e.g. set_fg_color(my_string)
		///  
        /// **Examples:**
        /// 
        /// 		win.set_fg_color("red")                    // Sets the window's text color to the color red
        /// 		win.set_fg_color(sage_color::Red)          // Sets the window's text color to red, also
        /// 		win.set_fg_color(pan_color::ForestGreen)   // Sets the window's text color to PanColor.ForestGreen
        /// 		win.set_fg_color(my_color)                 // Sets the window's text color to a defined my_color
		///
		fn set_fg_color(&self,color : ColorType);

		/// [**set_bg_color**] - Sets the background color of the text written to the window.
        /// 
        /// - The function <i>**set_set_bg_color()**</i> affects subsequent text written to the window with the <i>**write()**</i> or other window write functions.
        /// - When <i>**set_set_bg_color()**</i> is used, the next non-parameter <i>**cls()**</i> function call will clear the window to the new background color, unless
		/// a <i>**cls_bitmap()**</i> has been set.
		///
		/// **Function Parameter Forms:** 
		/// 
		///			set_bg_color(color : RgbColor)       // Set the background color with an RgbColor, variable, or preset 
		///                                              // ..such as pan_color::ForestGreen or sage_color::Blue
		///			set_bg_color(color : &str)           // Set the background color with a string slice, e.g. set_fg_color("green")
		///			set_bg_color(color : (i32,i32,i32))  // Set the background color with an (i32,i32,i32) RGB value, e.g. set_fg_color((0,255,0))
		///			set_bg_color(color : &String)        // Set the background color with a string e.g. set_fg_color(my_string)
		///  
        /// **Examples:**
        /// 
        /// 		win.set_bg_color("red")                    // Sets the window's background color to the color red
        /// 		win.set_bg_color(sage_color::Red)          // Sets the window's background color to red, also
        /// 		win.set_bg_color(pan_color::ForestGreen)   // Sets the window's background color to PanColor.ForestGreen
        /// 		win.set_bg_color(my_color)                 // Sets the window's background color to a defined my_color
		///
		fn set_bg_color(&self,color : ColorType);
	}
	pub trait SageDrawFuncClsRadial<ColorType> 
	{ 
   		/// [**cls_radial**()] - Clears the window to a blank canvas with a radial gradient (first color in the center of the window, to the second color at edges of the window)
        ///
		/// Function Parameter Forms: 
		///                    					
		///			cls_radial(color1 : RgbColor,color1 : RgbColor)   			
		///			cls_radial(color1 : (i32,i32,i32),color2 : (i32,i32.i32))  
		///			cls_radial_str(color_string : &str)  
		///
		/// **Notes**
		///
		/// - After using <i>**cls_radial()**</i> to create a vertical gradient, the next <i>cls()</i> with no parameters will clear the image to the same gradient.
		/// - See <i>**cls()**</i> to clear the window to a single color
		/// - See <i>**cls_grad()**</i> to clear the window with a vertical gradient instead of a radial gradient.
		///
        /// **Examples:**
        ///
        ///		win.cls_radial(pan_color::blue,pan_color::black)      // Clear the window with radial gradient from blue (center) to black (edges)
        ///		win.cls_radial_str("blue,black")                      // Same as the above, using string slice with two colors embedded
		///		win.cls()                                             // Clear the window with the current background color or gradient of the window
        ///
		fn cls_radial(&self,color : ColorType,color2 : ColorType) -> bool;	
	}
	pub trait SageDrawFuncTypeTriangle<T,ColorType>
	{
		/// [**draw_triangle()**] - Draws an open/wireframe triangle on the screen with axis points at p1, p2, and p3 
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			draw_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : RgbColor)
		///			draw_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : RgbColor)
		///			draw_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : RgbColor)
		///			draw_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : RgbColor)
		///			draw_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : &str)
		///			draw_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : &str)
		///			draw_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : &str)
		///			draw_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : &str)
		///			
		///         draw_triangle(...) -- Use draw_triangle() to use keywords, e.g. draw_triangle(p1,p2,p3,"green",kw::pen_size(10), etc.
		///
		/// - See <i>**fill_triangle()**</i> to draw a filled trriangle vs. an open/wireframe triangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		/// - use <i>**draw_polygon()**</i> to use an array with the three triangle points vs. using separate values for each 3 points.
		///
		/// **Parameters**
		/// 
		/// - **p1,p2,p3** - Location of the 3 axes of the triangle
		/// - **color** - Color of the borders/lines of the triangle
		/// 
		/// **Keywords can be included.  Some various keywords are as follows:**
		/// 
		/// - **pen_size** - Pen size for the border color if specified.  If the pen_size is not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Examples:**
		/// 
		///		let p1 = (300,300)	// Create three points for the triangle
		///		let p2 = (200,500)
		///		let p3 = (400,500)
		/// 
		///		win.draw_triangle(p1,p2,p2,pan_color::Yellow)               // Draw a triangle in Yellow
		///		win.draw_triangle(p1,p2,p2,"Yellow")                        // Same as above, using a text color
		///		win.draw_triangle_s(p1,p2,p3,"yellow",kw::pen_size(6))      // Draw a yellow triangle with a thickness of 6 pixels
		///	
		fn draw_triangle_s(&self,p1  : T , p2 : T, p3 : T,color : ColorType,keyw : kw) -> bool;

		/// [**draw_triangle()**] - Draws an open/wireframe triangle on the screen with axis points at p1, p2, and p3 
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			draw_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : RgbColor)
		///			draw_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : RgbColor)
		///			draw_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : RgbColor)
		///			draw_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : RgbColor)
		///			draw_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : &str)
		///			draw_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : &str)
		///			draw_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : &str)
		///			draw_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : &str)
		///			
		///         draw_triangle(...) -- Use draw_triangle() to use keywords, e.g. draw_triangle(p1,p2,p3,"green",kw::pen_size(10), etc.
		///
		/// - See <i>**fill_triangle()**</i> to draw a filled trriangle vs. an open/wireframe triangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		/// - use <i>**draw_polygon()**</i> to use an array with the three triangle points vs. using separate values for each 3 points.
		///
		/// **Parameters**
		/// 
		/// - **p1,p2,p3** - Location of the 3 axes of the triangle
		/// - **color** - Color of the borders/lines of the triangle
		/// 
		/// **Keywords can be included.  Some various keywords are as follows:**
		/// 
		/// - **pen_size** - Pen size for the border color if specified.  If the pen_size is not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Examples:**
		/// 
		///		let p1 = (300,300)	// Create three points for the triangle
		///		let p2 = (200,500)
		///		let p3 = (400,500)
		/// 
		///		win.draw_triangle(p1,p2,p2,pan_color::Yellow)               // Draw a triangle in Yellow
		///		win.draw_triangle(p1,p2,p2,"Yellow")                        // Same as above, using a text color
		///		win.draw_triangle_s(p1,p2,p3,"yellow",kw::pen_size(6))      // Draw a yellow triangle with a thickness of 6 pixels
		///	
		fn draw_triangle(&self,p1  : T , p2 : T, p3 : T,color : ColorType) -> bool;

		/// [**fill_triangle()**] - Draws a filled triangle on the screen with axis points at **`p1`**, **`p2`**, and **`p3`** 
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			fill_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : RgbColor)
		///			fill_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : RgbColor)
		///			fill_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : RgbColor)
		///			fill_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : RgbColor)
		///			fill_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : &str)
		///			fill_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : &str)
		///			fill_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : &str)
		///			fill_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : &str)
		///			
		///         fill_triangle_s(...) -- Use fill_triangle_s() to use keywords, e.g. fill_triangle_s(p1,p2,p3,"green",kw::pen_size(10), etc.
		///
		/// - See <i>**draw_triangle()**</i> to draw an open/wireframe Triangle vs. a filled Triangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		/// - use <i>**fill_polygon()**</i> to use an array with the three triangle points vs. using separate values for each 3 points.
		///
		/// **Parameters**
		/// 
		/// - **`p1,p2,p3`** - Location of the 3 axes of the triangle
		/// - **`color`** - Color of the interior of the triangle
		///
		/// **Keywords can be included.  Some various keywords are as follows:**
		/// 
		/// - **[`pen_color`]** - Color if the outside edge of the triangle (based on the current Pen Size).  When omitted, the entire triangle is drawn the color
		///of the inside_color.  
		///                 - See: <i>**pen_size()**</i> (below) to set the thickness of the border when using pen_color.
		/// - **[`pen_size`]** - Pen size for the border color if specified.  If the pen_size is not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Notes:**
		///
		/// - if either <i>**pen_size()**</i> or <i>**pen_color()**</i> is used as a keyword, a border is drawn around the filled triangle at the current pen size, or the supplied <i>**kw::pen_size()**</i> keyword.
		///    - when neither <i>**pen_size()**</i> or <i>**pen_color()**</i> is used as a keyword, no border is drawn.
		/// - if <i>**pen_color()**</i> is used as a keyword, but <i>**pen_size()**</i> is not, the border is drawn using the current graphics pen_size, which may 
		///be set with <i>**win.set_pen_size()**</i> (defaults to 1)
		///
		/// **Examples:**
		/// 
		///	let p1 = (300,300)	// Create three points for the triangle
		///	let p2 = (200,500)
		///	let p3 = (400,500)
		/// 
		///	win.fill_triangle(p1,p2,p2,pan_color::Yellow)               // Draw a filled triangle in Yellow
		///	win.fill_triangle(p1,p2,p2,"Yellow")                        // Same as above, using a text color
		///	win.fill_triangle_s(p1,p2,p3,"yellow",kw::pen_color("red")) // Draw triangle with a red border, using the current pen_size
		///	win.fill_triangle_s(p1,p2,p3,"yellow",kw::pen_size(6))      // Draw triangle with a default (white) border, with a thickness of 6 pixels
		///	
		fn fill_triangle_s(&self,p1  : T , p2 : T, p3 : T,color : ColorType,keyw : kw) -> bool;

		/// [**fill_triangle()**] - Draws a filled triangle on the screen with axis points at p1, p2, and p3 
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			fill_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : RgbColor)
		///			fill_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : RgbColor)
		///			fill_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : RgbColor)
		///			fill_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : RgbColor)
		///			fill_triangle(&self,p1  : (f32,f32)  , p2 : (f32,f32) , p3 : (f32,f32) , color : &str)
		///			fill_triangle(&self,p1  : (i32,i32)  , p2 : (i32,i32) , p3 : (i32,i32) , color : &str)
		///			fill_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>, color : &str)
		///			fill_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>, color : &str)
		///			
		///         fill_triangle_s(...) -- Use fill_triangle_s() to use keywords, e.g. fill_triangle_s(p1,p2,p3,"green",kw::pen_size(10), etc.
		///
		/// - See <i>**draw_triangle()**</i> to draw an open/wireframe Triangle vs. a filled Triangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		/// - use <i>**fill_polygon()**</i> to use an array with the three triangle points vs. using separate values for each 3 points.
		///
		/// **Parameters**
		/// 
		/// - **p1,p2,p3** - Location of the 3 axes of the triangle
		/// - **color** - Color of the interior of the triangle
		/// 
		/// **Keywords can be included.  Some various keywords are as follows:**
		/// 
		/// - **pen_color** - Color if the outside edge of the triangle (based on the current Pen Size).  When omitted, the entire triangle is drawn the color
		///of the inside_color.  
		///                 - See: <i>**pen_size()**</i> (below) to set the thickness of the border when using pen_color.
		/// - **pen_size** - Pen size for the border color if specified.  If the pen_size is not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Notes:**
		///
		/// - if either <i>**pen_size()**</i> or <i>**pen_color()**</i> is used as a keyword, a border is drawn around the filled triangle at the current pen size, or the supplied <i>**kw::pen_size()**</i> keyword.
		///    - when neither <i>**pen_size()**</i> or <i>**pen_color()**</i> is used as a keyword, no border is drawn.
		/// - if <i>**pen_color()**</i> is used as a keyword, but <i>**pen_size()**</i> is not, the border is drawn using the current graphics pen_size, which may 
		///be set with <i>**win.set_pen_size()**</i> (defaults to 1)
		///
		/// **Examples:**
		/// 
		///	let p1 = (300,300)	// Create three points for the triangle
		///	let p2 = (200,500)
		///	let p3 = (400,500)
		/// 
		///	win.fill_triangle(p1,p2,p2,pan_color::Yellow)               // Draw a filled triangle in Yellow
		///	win.fill_triangle(p1,p2,p2,"Yellow")                        // Same as above, using a text color
		///	win.fill_triangle_s(p1,p2,p3,"yellow",kw::pen_color("red")) // Draw triangle with a red border, using the current pen_size
		///	win.fill_triangle_s(p1,p2,p3,"yellow",kw::pen_size(6))      // Draw triangle with a default (white) border, with a thickness of 6 pixels
		///	
		fn fill_triangle(&self,p1  : T , p2 : T, p3 : T,color : ColorType) -> bool;
	}
	pub trait SageDrawFuncTypes<PairType,RadiusType,ColorType>
	{	
		/// [**fill_circle()**] - Draws a filled circle on the screen with center at `center` and radius `r`
		/// 
		/// - Valid types for center point are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			fill_circle(&self, center : (f32,f32) , radius : f32 , color : RgbColor)
		///			fill_circle(&self, center : (i32,i32) , radius : i32 , color : RgbColor)
		///			fill_circle(&self, center : Point<f32>, radius : f32 , color : RgbColor)
		///			fill_circle(&self, center : Point<i32>, radius : i32 , color : RgbColor)
		///			fill_circle(&self, center : (f32,f32) , radius : f32 , color : &str)
		///			fill_circle(&self, center : (i32,i32) , radius : i32 , color : &str)
		///			fill_circle(&self, center : Point<f32>, radius : f32 , color : &str)
		///			fill_circle(&self, center : Point<i32>, radius : i32 , color : &str)
		///			
		///         fill_circle_s(...) -- Use fill_circle_s() to use keywords, e.g. fill_circle_s(center, radius, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**draw_circle()**</i> to draw an open/wireframe circle instead of a filled circle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **center** - The center point of the circle
		/// - **radius** - Radius of the circle
		/// - **color** - Color of the interior of the circle
		/// 
		/// **Keywords can be included. Some various keywords are as follows:**
		/// 
		/// - **pen_color** - Color of the circle's edge (based on the current Pen Size). When omitted, the entire circle is drawn in the fill color.  
		///                 - See: <i>**pen_size()**</i> (below) to set the thickness of the border when using pen_color.
		/// - **pen_size** - Pen size for the border color if specified. If not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Notes:**
		///
		/// - if either <i>**pen_size()**</i> or <i>**pen_color()**</i> is used as a keyword, a border is drawn around the filled circle at the current pen size, or the supplied <i>**kw::pen_size()**</i>.
		///    - when neither <i>**pen_size()**</i> nor <i>**pen_color()**</i> is used as a keyword, no border is drawn.
		/// - if <i>**pen_color()**</i> is used as a keyword but <i>**pen_size()**</i> is not, the border is drawn using the current graphics pen_size, which may 
		///      be set with <i>**win.set_pen_size()**</i> (defaults to 1)
		///
		/// **Examples:**
		/// 
		///		let center = (300,300);	
		///		let radius = 100;
		/// 
		///		win.fill_circle(center, radius, pan_color::Yellow)                // Draw a filled circle in Yellow
		///		win.fill_circle(center, radius, "Yellow")                         // Same as above, using a text color
		///		win.fill_circle_s(center, radius, "yellow", kw::pen_color("red")) // Draw circle with a red border, using the current pen_size
		///		win.fill_circle_s(center, radius, "yellow", kw::pen_size(6))      // Draw circle with a default (white) border, with a thickness of 6 pixels
		///	
		fn fill_circle(&self,center : PairType, radius : RadiusType,color : ColorType) -> bool;

		/// [**fill_circle()**] - Draws a filled circle on the screen with center at `center` and radius `r`
		/// 
		/// - Valid types for center point are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			fill_circle(&self, center : (f32,f32) , radius : f32 , color : RgbColor)
		///			fill_circle(&self, center : (i32,i32) , radius : i32 , color : RgbColor)
		///			fill_circle(&self, center : Point<f32>, radius : f32 , color : RgbColor)
		///			fill_circle(&self, center : Point<i32>, radius : i32 , color : RgbColor)
		///			fill_circle(&self, center : (f32,f32) , radius : f32 , color : &str)
		///			fill_circle(&self, center : (i32,i32) , radius : i32 , color : &str)
		///			fill_circle(&self, center : Point<f32>, radius : f32 , color : &str)
		///			fill_circle(&self, center : Point<i32>, radius : i32 , color : &str)
		///			
		///         fill_circle_s(...) -- Use fill_circle_s() to use keywords, e.g. fill_circle_s(center, radius, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**draw_circle()**</i> to draw an open/wireframe circle instead of a filled circle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **center** - The center point of the circle
		/// - **radius** - Radius of the circle
		/// - **color** - Color of the interior of the circle
		/// 
		/// **Keywords can be included. Some various keywords are as follows:**
		/// 
		/// - **pen_color** - Color of the circle's edge (based on the current Pen Size). When omitted, the entire circle is drawn in the fill color.  
		///                 - See: <i>**pen_size()**</i> (below) to set the thickness of the border when using pen_color.
		/// - **pen_size** - Pen size for the border color if specified. If not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Notes:**
		///
		/// - if either <i>**pen_size()**</i> or <i>**pen_color()**</i> is used as a keyword, a border is drawn around the filled circle at the current pen size, or the supplied <i>**kw::pen_size()**</i>.
		///    - when neither <i>**pen_size()**</i> nor <i>**pen_color()**</i> is used as a keyword, no border is drawn.
		/// - if <i>**pen_color()**</i> is used as a keyword but <i>**pen_size()**</i> is not, the border is drawn using the current graphics pen_size, which may 
		///      be set with <i>**win.set_pen_size()**</i> (defaults to 1)
		///
		/// **Examples:**
		/// 
		///		let center = (300,300);	
		///		let radius = 100;
		/// 
		///		win.fill_circle(center, radius, pan_color::Yellow)                // Draw a filled circle in Yellow
		///		win.fill_circle(center, radius, "Yellow")                         // Same as above, using a text color
		///		win.fill_circle_s(center, radius, "yellow", kw::pen_color("red")) // Draw circle with a red border, using the current pen_size
		///		win.fill_circle_s(center, radius, "yellow", kw::pen_size(6))      // Draw circle with a default (white) border, with a thickness of 6 pixels
		///
		fn fill_circle_s(&self,center : PairType, radius : RadiusType,color : ColorType,keyw : kw) -> bool;

		/// [**draw_circle()**] - Draws an open (wireframe) circle on the screen with center at `center` and radius `r`
		/// 
		/// - Valid types for center point are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			draw_circle(&self, center : (f32,f32) , radius : f32 , color : RgbColor)
		///			draw_circle(&self, center : (i32,i32) , radius : i32 , color : RgbColor)
		///			draw_circle(&self, center : Point<f32>, radius : f32 , color : RgbColor)
		///			draw_circle(&self, center : Point<i32>, radius : i32 , color : RgbColor)
		///			draw_circle(&self, center : (f32,f32) , radius : f32 , color : &str)
		///			draw_circle(&self, center : (i32,i32) , radius : i32 , color : &str)
		///			draw_circle(&self, center : Point<f32>, radius : f32 , color : &str)
		///			draw_circle(&self, center : Point<i32>, radius : i32 , color : &str)
		///			
		///         draw_circle_s(...) -- Use draw_circle_s() to use keywords, e.g. draw_circle_s(center, radius, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**fill_circle()**</i> to draw an filled circle instead of an open/wireframe circle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **center** - The center point of the circle
		/// - **radius** - Radius of the circle
		/// - **color** - Color of the border of the circle
		/// 
		/// **Keywords can be included. Some various keywords are as follows:**
		/// 
		/// - **pen_size** - Pen size for the border color if specified. If not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Examples:**
		/// 
		///		let center = (300,300);	
		///		let radius = 100;
		/// 
		///		win.draw_circle(center, radius, pan_color::Yellow)                // Draw an open circle with a Yellow border
		///		win.draw_circle(center, radius, "Yellow")                         // Same as above, using a text color
		///		win.draw_circle_s(center, radius, "yellow", kw::pen_size(6))      // Draws a Yellow circle with a thickness of 6 pixels
		///
		fn draw_circle(&self,center : PairType, radius : RadiusType,color : ColorType) -> bool;

		/// [**draw_circle()**] - Draws an open (wireframe) circle on the screen with center at `center` and radius `r`
		/// 
		/// - Valid types for center point are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			draw_circle(&self, center : (f32,f32) , radius : f32 , color : RgbColor)
		///			draw_circle(&self, center : (i32,i32) , radius : i32 , color : RgbColor)
		///			draw_circle(&self, center : Point<f32>, radius : f32 , color : RgbColor)
		///			draw_circle(&self, center : Point<i32>, radius : i32 , color : RgbColor)
		///			draw_circle(&self, center : (f32,f32) , radius : f32 , color : &str)
		///			draw_circle(&self, center : (i32,i32) , radius : i32 , color : &str)
		///			draw_circle(&self, center : Point<f32>, radius : f32 , color : &str)
		///			draw_circle(&self, center : Point<i32>, radius : i32 , color : &str)
		///			
		///         draw_circle_s(...) -- Use draw_circle_s() to use keywords, e.g. draw_circle_s(center, radius, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**fill_circle()**</i> to draw an filled circle instead of an open/wireframe circle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **center** - The center point of the circle
		/// - **radius** - Radius of the circle
		/// - **color** - Color of the border of the circle
		/// 
		/// **Keywords can be included. Some various keywords are as follows:**
		/// 
		/// - **pen_size** - Pen size for the border color if specified. If not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Examples:**
		/// 
		///		let center = (300,300);	
		///		let radius = 100;
		/// 
		///		win.draw_circle(center, radius, pan_color::Yellow)                // Draw an open circle with a Yellow border
		///		win.draw_circle(center, radius, "Yellow")                         // Same as above, using a text color
		///		win.draw_circle_s(center, radius, "yellow", kw::pen_size(6))      // Draws a Yellow circle with a thickness of 6 pixels
		///
		fn draw_circle_s(&self,center : PairType, radius : RadiusType,color : ColorType,keyw : kw) -> bool;


		/// [**draw_line()**] - Draws a line on the screen from point `p1` to point `p2`.
		///
		/// - Valid types for points are *`(f32, f32)`, `(i32, i32)`, `Point<i32>`, `Point<f32>`*
		/// - Valid types for colors are *`RgbColor`* or *`&str`*.  
		///   - Colors in the form *(i32, i32, i32)* can be used via `RgbColor::fromi32()`, e.g. `RgbColor::fromi32((0, 255, 0))`
		///
		/// **Function Parameter Forms:** 
		///
		///         draw_line(&self, p1: (f32, f32),   p2: (f32, f32),   color: RgbColor)
		///         draw_line(&self, p1: (i32, i32),   p2: (i32, i32),   color: RgbColor)
		///         draw_line(&self, p1: Point<f32>,   p2: Point<f32>,   color: RgbColor)
		///         draw_line(&self, p1: Point<i32>,   p2: Point<i32>,   color: RgbColor)
		///         draw_line(&self, p1: (f32, f32),   p2: (f32, f32),   color: &str)
		///         draw_line(&self, p1: (i32, i32),   p2: (i32, i32),   color: &str)
		///         draw_line(&self, p1: Point<f32>,   p2: Point<f32>,   color: &str)
		///         draw_line(&self, p1: Point<i32>,   p2: Point<i32>,   color: &str)
		///
		///         draw_line_s(...) -- Use `draw_line_s()` with additional keyword options,  
		///         e.g. `draw_line_s(p1, p2, "green", kw::pen_size(10), ...)`
		///
		/// - See *`draw_lines()`* to draw multiple connected lines from a point array.
		/// - This drawing function respects current opacity, transformations, and anti-aliasing settings  
		///   (unless smoothing is disabled with `set_smoothing_mode()`).
		/// - Also see <i>`draw_line_to()`</i> to draw a line from the current saved point to the next point.
		///
		/// **Parameters**
		///
		/// - **p1, p2** — Start and end points of the line.
		/// - **color** — Color of the line.
		///
		/// **Keywords can be included. Some available keywords:**
		///
		/// - **pen_size** — Thickness of the line in pixels.  
		///   If not specified, the current default pen size is used.
		/// - **<others TBD>** — Additional keyword options are available but not public in the current version.
		///
		/// **Examples:**
		///
		///     let p1 = (100, 100);
		///     let p2 = (300, 300);
		///
		///     win.draw_line(p1, p2, pan_color::Green);             // Draw a green line
		///     win.draw_line(p1, p2, "Green");                      // Same as above using a string color
		///     win.draw_line_s(p1, p2, "green", kw::pen_size(4));   // Draw a green line with 4-pixel thickness	
		///	
		fn draw_line(&self, point1 : PairType, point2 : PairType, color : ColorType) -> bool;

		/// [**draw_line()**] - Draws a line on the screen from point `p1` to point `p2`.
		///
		/// - Valid types for points are *`(f32, f32)`, `(i32, i32)`, `Point<i32>`, `Point<f32>`*
		/// - Valid types for colors are *`RgbColor`* or *`&str`*.  
		///   - Colors in the form *(i32, i32, i32)* can be used via `RgbColor::fromi32()`, e.g. `RgbColor::fromi32((0, 255, 0))`
		///
		/// **Function Parameter Forms:** 
		///
		///         draw_line(&self, p1: (f32, f32),   p2: (f32, f32),   color: RgbColor)
		///         draw_line(&self, p1: (i32, i32),   p2: (i32, i32),   color: RgbColor)
		///         draw_line(&self, p1: Point<f32>,   p2: Point<f32>,   color: RgbColor)
		///         draw_line(&self, p1: Point<i32>,   p2: Point<i32>,   color: RgbColor)
		///         draw_line(&self, p1: (f32, f32),   p2: (f32, f32),   color: &str)
		///         draw_line(&self, p1: (i32, i32),   p2: (i32, i32),   color: &str)
		///         draw_line(&self, p1: Point<f32>,   p2: Point<f32>,   color: &str)
		///         draw_line(&self, p1: Point<i32>,   p2: Point<i32>,   color: &str)
		///
		///         draw_line_s(...) -- Use `draw_line_s()` with additional keyword options,  
		///         e.g. `draw_line_s(p1, p2, "green", kw::pen_size(10), ...)`
		///
		/// - See *`draw_lines()`* to draw multiple connected lines from a point array.
		/// - This drawing function respects current opacity, transformations, and anti-aliasing settings  
		///   (unless smoothing is disabled with `set_smoothing_mode()`).
		/// - Also see <i>`draw_line_to()`</i> to draw a line from the current saved point to the next point.
		///
		/// **Parameters**
		///
		/// - **p1, p2** — Start and end points of the line.
		/// - **color** — Color of the line.
		///
		/// **Keywords can be included. Some available keywords:**
		///
		/// - **pen_size** — Thickness of the line in pixels.  
		///   If not specified, the current default pen size is used.
		/// - **<others TBD>** — Additional keyword options are available but not public in the current version.
		///
		/// **Examples:**
		///
		///     let p1 = (100, 100);
		///     let p2 = (300, 300);
		///
		///     win.draw_line(p1, p2, pan_color::Green);             // Draw a green line
		///     win.draw_line(p1, p2, "Green");                      // Same as above using a string color
		///     win.draw_line_s(p1, p2, "green", kw::pen_size(4));   // Draw a green line with 4-pixel thickness	
		///	
		fn draw_line_s(&self, point1 : PairType, point2 : PairType , color : ColorType,keyw : kw) -> bool;

		/// [**draw_line_to()**] - Draws a line from the last known point to the new point.
		///
		/// - Valid types for points are *`(f32, f32)`, `(i32, i32)`, `Point<i32>`, `Point<f32>`*
		/// - Valid types for colors are *`RgbColor`* or *`&str`*.  
		///   - Colors in the form *(i32, i32, i32)* can be used via `RgbColor::fromi32()`, e.g. `RgbColor::fromi32((0, 255, 0))`
		///
		/// **Functions**
		///
		/// 	draw_line_to()           // Draw line to next point
		/// 	draw_line_to_s()         // Draw line to next point (+ keywords)
		/// 	draw_line_to_ex()        // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only
		/// 	draw_line_to_ex_s()      // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only (+keywords)
		///
		/// **Function Parameter Forms:** 
		///
		///         draw_line_to(&self, p1: (f32, f32), color: RgbColor)
		///         draw_line_to(&self, p1: (i32, i32), color: RgbColor)
		///         draw_line_to(&self, p1: Point<f32>, color: RgbColor)
		///         draw_line_to(&self, p1: Point<i32>, color: RgbColor)
		///         draw_line_to(&self, p1: (f32, f32), color: &str)
		///         draw_line_to(&self, p1: (i32, i32), color: &str)
		///         draw_line_to(&self, p1: Point<f32>, color: &str)
		///         draw_line_to(&self, p1: Point<i32>, color: &str)
		///
		/// **Parameters**
		/// 
		/// - **p1** — point to draw line to from current set point
		/// - **is_first** — when using <i>`draw_line_to_ex()`
		///   - when `true`, this sets the point as the first point and does not draw a line.  The next call will draw from this point
		///   - when `false`, this acts just as `draw_line_to()` and draws the line from the current point to the next point
		/// - **color** — Color of the line.
		///
		/// Use `draw_line_s()` with additional keyword options, e.g. `draw_line_s(p1, p2, "green", kw::pen_size(10), ...)`
		///
		///
		/// **Keywords can be included. Some available keywords:**
		///
		/// - **pen_size** — Thickness of the line in pixels.  
		///   If not specified, the current default pen size is used.
		/// - **<others TBD>** — Additional keyword options are available but not public in the current version.
		///
		/// **notes**
		///
		/// - See the function <i>`move_to()`</i> to set the intiial drawing position for using <i>`draw_line_to()`</i>
		///
		/// **Examples:**
		///
		///     let p1 = (100, 100);
		///
		///     win.draw_line_to(p1, p2, pan_color::Green);             // Draw a green line
		///     win.draw_line_to(p1, p2, "Green");                      // Same as above using a string color
		///     win.draw_line_ex_s(false,p1, "green", kw::pen_size(4)); // Draw a green line with 4-pixel thickness	
		///	
		fn draw_line_to(&self, point1 : PairType, color : ColorType) -> bool;

		/// [**draw_line_to()**] - Draws a line from the last known point to the new point.
		///
		/// - Valid types for points are *`(f32, f32)`, `(i32, i32)`, `Point<i32>`, `Point<f32>`*
		/// - Valid types for colors are *`RgbColor`* or *`&str`*.  
		///   - Colors in the form *(i32, i32, i32)* can be used via `RgbColor::fromi32()`, e.g. `RgbColor::fromi32((0, 255, 0))`
		///
		/// **Functions**
		///
		/// 	draw_line_to()           // Draw line to next point
		/// 	draw_line_to_s()         // Draw line to next point (+ keywords)
		/// 	draw_line_to_ex()        // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only
		/// 	draw_line_to_ex_s()      // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only (+keywords)
		///
		/// **Function Parameter Forms:** 
		///
		///         draw_line_to(&self, p1: (f32, f32), color: RgbColor)
		///         draw_line_to(&self, p1: (i32, i32), color: RgbColor)
		///         draw_line_to(&self, p1: Point<f32>, color: RgbColor)
		///         draw_line_to(&self, p1: Point<i32>, color: RgbColor)
		///         draw_line_to(&self, p1: (f32, f32), color: &str)
		///         draw_line_to(&self, p1: (i32, i32), color: &str)
		///         draw_line_to(&self, p1: Point<f32>, color: &str)
		///         draw_line_to(&self, p1: Point<i32>, color: &str)
		///
		/// **Parameters**
		/// 
		/// - **p1** — point to draw line to from current set point
		/// - **is_first** — when using <i>`draw_line_to_ex()`
		///   - when `true`, this sets the point as the first point and does not draw a line.  The next call will draw from this point
		///   - when `false`, this acts just as `draw_line_to()` and draws the line from the current point to the next point
		/// - **color** — Color of the line.
		///
		/// Use `draw_line_s()` with additional keyword options, e.g. `draw_line_s(p1, p2, "green", kw::pen_size(10), ...)`
		///
		///
		/// **Keywords can be included. Some available keywords:**
		///
		/// - **pen_size** — Thickness of the line in pixels.  
		///   If not specified, the current default pen size is used.
		/// - **<others TBD>** — Additional keyword options are available but not public in the current version.
		///
		/// **notes**
		///
		/// - See the function <i>`move_to()`</i> to set the intiial drawing position for using <i>`draw_line_to()`</i>
		///
		/// **Examples:**
		///
		///     let p1 = (100, 100);
		///
		///     win.draw_line_to(p1, p2, pan_color::Green);             // Draw a green line
		///     win.draw_line_to(p1, p2, "Green");                      // Same as above using a string color
		///     win.draw_line_ex_s(false,p1, "green", kw::pen_size(4)); // Draw a green line with 4-pixel thickness	
		///	
		fn draw_line_to_s(&self, point1 : PairType, color : ColorType,keyw : kw) -> bool;

		/// [**draw_line_to()**] - Draws a line from the last known point to the new point.
		///
		/// - Valid types for points are *`(f32, f32)`, `(i32, i32)`, `Point<i32>`, `Point<f32>`*
		/// - Valid types for colors are *`RgbColor`* or *`&str`*.  
		///   - Colors in the form *(i32, i32, i32)* can be used via `RgbColor::fromi32()`, e.g. `RgbColor::fromi32((0, 255, 0))`
		///
		/// **Functions**
		///
		/// 	draw_line_to()           // Draw line to next point
		/// 	draw_line_to_s()         // Draw line to next point (+ keywords)
		/// 	draw_line_to_ex()        // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only
		/// 	draw_line_to_ex_s()      // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only (+keywords)
		///
		/// **Function Parameter Forms:** 
		///
		///         draw_line_to(&self, p1: (f32, f32), color: RgbColor)
		///         draw_line_to(&self, p1: (i32, i32), color: RgbColor)
		///         draw_line_to(&self, p1: Point<f32>, color: RgbColor)
		///         draw_line_to(&self, p1: Point<i32>, color: RgbColor)
		///         draw_line_to(&self, p1: (f32, f32), color: &str)
		///         draw_line_to(&self, p1: (i32, i32), color: &str)
		///         draw_line_to(&self, p1: Point<f32>, color: &str)
		///         draw_line_to(&self, p1: Point<i32>, color: &str)
		///
		/// **Parameters**
		/// 
		/// - **p1** — point to draw line to from current set point
		/// - **is_first** — when using <i>`draw_line_to_ex()`
		///   - when `true`, this sets the point as the first point and does not draw a line.  The next call will draw from this point
		///   - when `false`, this acts just as `draw_line_to()` and draws the line from the current point to the next point
		/// - **color** — Color of the line.
		///
		/// Use `draw_line_s()` with additional keyword options, e.g. `draw_line_s(p1, p2, "green", kw::pen_size(10), ...)`
		///
		///
		/// **Keywords can be included. Some available keywords:**
		///
		/// - **pen_size** — Thickness of the line in pixels.  
		///   If not specified, the current default pen size is used.
		/// - **<others TBD>** — Additional keyword options are available but not public in the current version.
		///
		/// **notes**
		///
		/// - See the function <i>`move_to()`</i> to set the intiial drawing position for using <i>`draw_line_to()`</i>
		///
		/// **Examples:**
		///
		///     let p1 = (100, 100);
		///
		///     win.draw_line_to(p1, p2, pan_color::Green);             // Draw a green line
		///     win.draw_line_to(p1, p2, "Green");                      // Same as above using a string color
		///     win.draw_line_ex_s(false,p1, "green", kw::pen_size(4)); // Draw a green line with 4-pixel thickness	
		///	
		fn draw_line_to_ex(&self, is_first_point : bool, point : PairType, color : ColorType) -> bool;

		/// [**draw_line_to()**] - Draws a line from the last known point to the new point.
		///
		/// - Valid types for points are *`(f32, f32)`, `(i32, i32)`, `Point<i32>`, `Point<f32>`*
		/// - Valid types for colors are *`RgbColor`* or *`&str`*.  
		///   - Colors in the form *(i32, i32, i32)* can be used via `RgbColor::fromi32()`, e.g. `RgbColor::fromi32((0, 255, 0))`
		///
		/// **Functions**
		///
		/// 	draw_line_to()           // Draw line to next point
		/// 	draw_line_to_s()         // Draw line to next point (+ keywords)
		/// 	draw_line_to_ex()        // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only
		/// 	draw_line_to_ex_s()      // Draw line to next point, if 'is_first' is false.  Otherwise set intitial point only (+keywords)
		///
		/// **Function Parameter Forms:** 
		///
		///         draw_line_to(&self, p1: (f32, f32), color: RgbColor)
		///         draw_line_to(&self, p1: (i32, i32), color: RgbColor)
		///         draw_line_to(&self, p1: Point<f32>, color: RgbColor)
		///         draw_line_to(&self, p1: Point<i32>, color: RgbColor)
		///         draw_line_to(&self, p1: (f32, f32), color: &str)
		///         draw_line_to(&self, p1: (i32, i32), color: &str)
		///         draw_line_to(&self, p1: Point<f32>, color: &str)
		///         draw_line_to(&self, p1: Point<i32>, color: &str)
		///
		/// **Parameters**
		/// 
		/// - **p1** — point to draw line to from current set point
		/// - **is_first** — when using <i>`draw_line_to_ex()`
		///   - when `true`, this sets the point as the first point and does not draw a line.  The next call will draw from this point
		///   - when `false`, this acts just as `draw_line_to()` and draws the line from the current point to the next point
		/// - **color** — Color of the line.
		///
		/// Use `draw_line_s()` with additional keyword options, e.g. `draw_line_s(p1, p2, "green", kw::pen_size(10), ...)`
		///
		///
		/// **Keywords can be included. Some available keywords:**
		///
		/// - **pen_size** — Thickness of the line in pixels.  
		///   If not specified, the current default pen size is used.
		/// - **<others TBD>** — Additional keyword options are available but not public in the current version.
		///
		/// **notes**
		///
		/// - See the function <i>`move_to()`</i> to set the intiial drawing position for using <i>`draw_line_to()`</i>
		///
		/// **Examples:**
		///
		///     let p1 = (100, 100);
		///
		///     win.draw_line_to(p1, p2, pan_color::Green);             // Draw a green line
		///     win.draw_line_to(p1, p2, "Green");                      // Same as above using a string color
		///     win.draw_line_ex_s(false,p1, "green", kw::pen_size(4)); // Draw a green line with 4-pixel thickness	
		///	
		fn draw_line_to_ex_s(&self, is_first_point : bool, point : PairType, color : ColorType,keyw : kw) -> bool;

		/// [**draw_rectangle()**] - Draws an open (wireframe) rectangle on the screen starting at `location` with dimensions from `size`
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			draw_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : RgbColor)
		///			draw_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : RgbColor)
		///			draw_rectangle(&self, location : Point<f32>, size : Point<f32>, color : RgbColor)
		///			draw_rectangle(&self, location : Point<i32>, size : Point<i32>, color : RgbColor)
		///			draw_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : &str)
		///			draw_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : &str)
		///			draw_rectangle(&self, location : Point<f32>, size : Point<f32>, color : &str)
		///			draw_rectangle(&self, location : Point<i32>, size : Point<i32>, color : &str)
		///			
		///         draw_rectangle_s(...) -- Use draw_rectangle_s() to use keywords, e.g. draw_rectangle_s(location, size, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**fill_rectangle()**</i> to draw a flled rectangle vs an open/wireframe rectangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **location** - Top-left corner of the rectangle
		/// - **size** - Width and height of the rectangle, specified as a point
		/// - **color** - Color of the of the rectangle border lines.
		/// 
		/// **Keywords can be included. Some useful keywords are as follows:**
		/// 
		/// - **pen_size** - Pen size for the border color.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Examples:**
		/// 
		///		let location = (100, 100);	
		///		let size = (200, 150); // Width: 200, Height: 150
		/// 
		///		win.draw_rectangle(location, size, pan_color::Yellow)                // Draw an open rectangle in Yellow
		///		win.draw_rectangle(location, size, "Yellow")                         // Same as above, using a text color
		///		win.draw_rectangle_s(location, size, "yellow", kw::pen_size(6))      // Draw a yellow rectangle with a thickness of 6 pixels
		///
		fn draw_rectangle(&self,location : PairType, size : PairType ,color : ColorType) -> bool;

		/// [**draw_rectangle()**] - Draws an open (wireframe) rectangle on the screen starting at `location` with dimensions from `size`
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			draw_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : RgbColor)
		///			draw_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : RgbColor)
		///			draw_rectangle(&self, location : Point<f32>, size : Point<f32>, color : RgbColor)
		///			draw_rectangle(&self, location : Point<i32>, size : Point<i32>, color : RgbColor)
		///			draw_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : &str)
		///			draw_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : &str)
		///			draw_rectangle(&self, location : Point<f32>, size : Point<f32>, color : &str)
		///			draw_rectangle(&self, location : Point<i32>, size : Point<i32>, color : &str)
		///			
		///         draw_rectangle_s(...) -- Use draw_rectangle_s() to use keywords, e.g. draw_rectangle_s(location, size, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**fill_rectangle()**</i> to draw a flled rectangle vs an open/wireframe rectangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **location** - Top-left corner of the rectangle
		/// - **size** - Width and height of the rectangle, specified as a point
		/// - **color** - Color of the of the rectangle border lines.
		/// 
		/// **Keywords can be included. Some useful keywords are as follows:**
		/// 
		/// - **pen_size** - Pen size for the border color.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Examples:**
		/// 
		///		let location = (100, 100);	
		///		let size = (200, 150); // Width: 200, Height: 150
		/// 
		///		win.draw_rectangle(location, size, pan_color::Yellow)                // Draw an open rectangle in Yellow
		///		win.draw_rectangle(location, size, "Yellow")                         // Same as above, using a text color
		///		win.draw_rectangle_s(location, size, "yellow", kw::pen_size(6))      // Draw a yellow rectangle with a thickness of 6 pixels
		///
		fn draw_rectangle_s(&self,location : PairType , size : PairType, color : ColorType,keyw : kw) -> bool;
		
		/// [**fill_rectangle()**] - Draws a filled rectangle on the screen starting at `location` with dimensions from `size`
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			fill_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : RgbColor)
		///			fill_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : RgbColor)
		///			fill_rectangle(&self, location : Point<f32>, size : Point<f32>, color : RgbColor)
		///			fill_rectangle(&self, location : Point<i32>, size : Point<i32>, color : RgbColor)
		///			fill_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : &str)
		///			fill_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : &str)
		///			fill_rectangle(&self, location : Point<f32>, size : Point<f32>, color : &str)
		///			fill_rectangle(&self, location : Point<i32>, size : Point<i32>, color : &str)
		///			
		///         fill_rectangle_s(...) -- Use fill_rectangle_s() to use keywords, e.g. fill_rectangle_s(location, size, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**draw_rectangle()**</i> to draw an open/wireframe Rectangle vs. a filled Rectangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **location** - Top-left corner of the rectangle
		/// - **size** - Width and height of the rectangle, specified as a point
		/// - **color** - Color of the interior of the rectangle
		/// 
		/// **Keywords can be included. Some useful keywords are as follows:**
		/// 
		/// - **pen_color** - Color of the rectangle’s border (based on the current Pen Size). When omitted, the entire rectangle is drawn in the fill color.  
		///                 - See: <i>**pen_size()**</i> (below) to set the thickness of the border when using pen_color.
		/// - **pen_size** - Pen size for the border color if specified. If pen_size is not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Notes:**
		///
		/// - If either <i>**pen_size()**</i> or <i>**pen_color()**</i> is used, a border is drawn around the filled rectangle using the specified values.
		/// - If only <i>**pen_color()**</i> is used, the current pen size is used (default is 1 pixel).
		///
		/// **Examples:**
		/// 
		///		let location = (100, 100);	
		///		let size = (200, 150); // Width: 200, Height: 150
		/// 
		///		win.fill_rectangle(location, size, pan_color::Yellow)                // Draw a filled rectangle in Yellow
		///		win.fill_rectangle(location, size, "Yellow")                         // Same as above, using a text color
		///		win.fill_rectangle_s(location, size, "yellow", kw::pen_color("red")) // Draw rectangle with red border, using current pen_size
		///		win.fill_rectangle_s(location, size, "yellow", kw::pen_size(6))      // Draw rectangle with default (white) border, with a thickness of 6 pixels
		///
		fn fill_rectangle(&self,location : PairType, size : PairType,color : ColorType) -> bool;

		/// [**fill_rectangle()**] - Draws a filled rectangle on the screen starting at `location` with dimensions from `size`
		/// 
		/// - Valid types for points are <i>**(f32,f32), (i32,i32), Point&lt;i32>, Point&lt;f32>**</i>
		/// - Valid types for colors are <i>**RgbColor**</i> or <i>**&str**</i>. 	
		///   - Colors with <i>**(i32,i32,i32)**</i> type values can be used by calling <i>**RgbColor::fromi32()**</i>, e.g. <i>**RgbColor::fromi32((0,255,0))**</i>	
		///
		/// **Function Parameter Forms:** 
		///				
		///			fill_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : RgbColor)
		///			fill_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : RgbColor)
		///			fill_rectangle(&self, location : Point<f32>, size : Point<f32>, color : RgbColor)
		///			fill_rectangle(&self, location : Point<i32>, size : Point<i32>, color : RgbColor)
		///			fill_rectangle(&self, location : (f32,f32) , size : (f32,f32) , color : &str)
		///			fill_rectangle(&self, location : (i32,i32) , size : (i32,i32) , color : &str)
		///			fill_rectangle(&self, location : Point<f32>, size : Point<f32>, color : &str)
		///			fill_rectangle(&self, location : Point<i32>, size : Point<i32>, color : &str)
		///			
		///         fill_rectangle_s(...) -- Use fill_rectangle_s() to use keywords, e.g. fill_rectangle_s(location, size, "green", kw::pen_size(10), etc.)
		///
		/// - See <i>**draw_rectangle()**</i> to draw an open/wireframe Rectangle vs. a filled Rectangle. 
		/// - This drawing function responds to current opacity and transformations and will anti-alias output results (unless anti-aliasing is turned off with set_smoothing_mode()).
		///
		/// **Parameters**
		/// 
		/// - **location** - Top-left corner of the rectangle
		/// - **size** - Width and height of the rectangle, specified as a point
		/// - **color** - Color of the interior of the rectangle
		/// 
		/// **Keywords can be included. Some useful keywords are as follows:**
		/// 
		/// - **pen_color** - Color of the rectangle’s border (based on the current Pen Size). When omitted, the entire rectangle is drawn in the fill color.  
		///                 - See: <i>**pen_size()**</i> (below) to set the thickness of the border when using pen_color.
		/// - **pen_size** - Pen size for the border color if specified. If pen_size is not specified, the current default pen size is used.
		/// - **&lt;others TBD>** - There are many other keywords available for this function, but are not yet public in current version.
		///
		/// **Notes:**
		///
		/// - If either <i>**pen_size()**</i> or <i>**pen_color()**</i> is used, a border is drawn around the filled rectangle using the specified values.
		/// - If only <i>**pen_color()**</i> is used, the current pen size is used (default is 1 pixel).
		///
		/// **Examples:**
		/// 
		///		let location = (100, 100);	
		///		let size = (200, 150); // Width: 200, Height: 150
		/// 
		///		win.fill_rectangle(location, size, pan_color::Yellow)                // Draw a filled rectangle in Yellow
		///		win.fill_rectangle(location, size, "Yellow")                         // Same as above, using a text color
		///		win.fill_rectangle_s(location, size, "yellow", kw::pen_color("red")) // Draw rectangle with red border, using current pen_size
		///		win.fill_rectangle_s(location, size, "yellow", kw::pen_size(6))      // Draw rectangle with default (white) border, with a thickness of 6 pixels
		///
		fn fill_rectangle_s(&self,location : PairType, size : PairType,color : ColorType,keyw : kw) -> bool;


	}
	


	//(Copy, Clone)]
	pub struct Conio
	{		
  		_id : i64,	// Dummy value for now
	}	

	pub trait SageConioSetFgColor<StringType>
	{
		fn set_fg_color(msg : StringType) -> bool;
		fn write(msg : StringType) -> bool;
	}

	impl Conio
	{  	
		pub fn new() -> Conio
		{
			Conio { _id: 0 }	// id is a dummy value (for now)
		}

		fn _set_fg_color(msg : &str) -> bool
		{
			unsafe { ext_func::rust_sagebox_console_set_fg_color(msg.as_ptr(),msg.len()) }
		}

		fn _write(msg : &str) -> bool
		{
			unsafe { ext_func::rust_sagebox_console_write(msg.as_ptr(),msg.len(),false,false) }
		}			
	}
	impl SageConioSetFgColor<&str> for Conio
	{
		fn set_fg_color(msg : &str) -> bool { Conio::_set_fg_color(msg) }
		fn write(msg : &str) -> bool { Conio::_write(msg) }
	}

	impl SageConioSetFgColor<&String> for Conio
	{
		fn set_fg_color(msg : &String) -> bool { Conio::_set_fg_color(msg.as_str()) }
		fn write(msg : &String) -> bool { Conio::_write(msg.as_str()) }
	}

#[derive(Copy, Clone)]
pub struct DevWindow
{
	id : i64,
}

impl DevWindow
{
 	pub fn new( _id : i64) -> DevWindow
	{
		DevWindow { id: _id }
	}
	pub const fn default() -> DevWindow
	{
		DevWindow { id: 0 }
	}

	/// Creates a slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are integer.  see: new_slider_f() to use floating-point values.
	///
	/// 	Forms: 
	/// 
	/// 	- new_slider(title : str)                  -- Slider with no keywords added
	/// 	- new_slider_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with new_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - dev_win.new_slider("This is a slider")
	///                 - dev_win.new_slider_s("This is a slider",kw::range(100,500) + kw::default(200))
	///                 - dev_win.new_slider_s("This is a slider",kw::text_color("yellow") + kw::style="small"	
	///
	pub fn new_slider_s(&self,title : &str,keyw: kw) -> Slider
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_slider(title.as_ptr(),title.len(),false,self.id,keyw.pointer); }
		Slider::new(ret_val)
	}
	/// Creates a slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are integer.  see: new_slider_f() to use floating-point values.
	///
	/// 	Forms: 
	/// 
	/// 	- new_slider(title : str)                  -- Slider with no keywords added
	/// 	- new_slider_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with new_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - dev_win.new_slider("This is a slider")
	///                 - dev_win.new_slider_s("This is a slider",kw::range(100,500) + kw::default(200))
	///                 - dev_win.new_slider_s("This is a slider",kw::text_color("yellow") + kw::style="small"	
	///
	pub fn new_slider(&self,title : &str) -> Slider
	{
		self.new_slider_s(title,KW_NONE)
	}

	///<summary> 
	/// Creates a floating-point slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are floating-point.  see: new_slider() to use integer values values.
	///
	/// 	Forms: 
	/// 
	/// 	- new_slider_f(title : str)                  -- Slider with no keywords added
	/// 	- new_slider_f_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with new_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - dev_win.new_sliderf("This is a slider")
	///                 - dev_win.new_sliderf_s("This is a slider",kw::range(-5.5,15.8) + kw::default(200))
	///                 - dev_win.new_sliderf_s("This is a slider",kw::text_color("yellow") + kw::style="small"
	/// </summary>
	pub fn new_slider_f_s(&self,title : &str,keyw: kw) -> Slider
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_slider(title.as_ptr(),title.len(),true,self.id,keyw.pointer); }
		Slider::new(ret_val)
	}

	///<summary> 
	/// Creates a floating-point slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are floating-point.  see: new_slider() to use integer values values.
	///
	/// 	Forms: 
	/// 
	/// 	- new_slider_f(title : str)                  -- Slider with no keywords added
	/// 	- new_slider_f_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with new_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - dev_win.new_sliderf("This is a slider")
	///                 - dev_win.new_sliderf_s("This is a slider",kw::range(-5.5,15.8) + kw::default(200))
	///                 - dev_win.new_sliderf_s("This is a slider",kw::text_color("yellow") + kw::style="small"
	/// </summary>
	pub fn new_slider_f(&self,title : &str) -> Slider
	{
		self.new_slider_f_s(title,KW_NONE)
	}

	
	/// [**new_inputbox()**] - Creates a new Input Box where you can enter text data as simple as a few characters are entire paragraphs. 
	/// 
	/// The Input box is placed in automaitcally in the Dev Window.
	///
	/// - Input boxes can be limited to only numbers, either integer or float values
	/// - Input boxes that accept only numbers can use the range() keyword to set the valid range. 
	/// - Input boxes can be set to automatically bring up an 'invalid' window for empty text or numbers out of range.
	/// - Input boxes can be set to readonly, and can also be set to display '*' for text when entering passwords.
	/// 
	/// **Function Parameter Forms:**
	///
	///		new_inputbox(title : &str)              // Creates in Input box with a a title
	///		new_inputbox_s(title : &str,keyw: kw)   // Used when entering keywords to shape the input box characteristics
	///
	/// **Available Keywords with dev_inputbox()**
	/// 
	/// Keyword | Action
	///---- | ----
	/// **width**  		| -Sets the width of the input box in pixels.  The default is 250 pixels.
	/// **height**        | -Sets the height of the input box in pixels.  The default is set to the default Font for 1 line of text. 
	/// **text**          | -This sets the starting text for the input box.  Otheriwse the input box is left blank at first. 
	/// **font**         | - Sets the font for the input box.  The default is the current font of the window
	/// **numbersonly**  | - Causes the input box to only accept numbers. 
	/// **readonly**     | - Sets the input box as read only so it can be used as a way to place a large amount of text that can be copied.
	/// **textcolor**    | - Sets the color of the text in the input box
	/// **bgcolor**      | - Sets the background color of the text in the input box
	/// **label**        | - Sets a label to the right of the input box. LabelRight, LabelLeft, LabelBottom, and LabelTop can be used to set the location of the label.
	/// **label_color**  | - Sets the color of the label (i.e. opt.label_color("Red"))
	/// **multiline**    | - Sets the input box as a multi-line input box.  This allows more than one line to be entered. A button or some method to end input must be used unless "WantReturn" is specified
	/// **wantreturn**   | - For multi-line boxes, this sends a "return pressed" message when the return key is pressed.
	/// **password**     | - Causes the input box to display '*' for all text.
	/// **noborder**     | - Causes the input box to not use a border so it will blend into the window more seamlessly.
	/// **thickborder,recessed** 	| - These are two different border styles that can be used.
	/// **vscroll, hscroll**      | - Adds a vertical scrollbar (useful for multi-line boxes) and horizontal scroll bar, respectively
	/// **wincolors**  	| - Sets the background input box color and text color to the current window color instead of the default white-and-black colors. 
	/// 
	/// Examples: 
	///
	///     dev_win.new_inputbox();                   // Create input box
	///     dev_win.new_inputbox_s(kw::height(500));  // Create an input box of height 500 pixels
	///     dev_win.new_inputbox_s(kw::font(20) + kw:label("Enter Data"));
	///     dev_win.new_inputbox_s(kw::text("This is the default text")); 		// Create an input box and set initial text.
	///
	pub fn new_input_box_s(&self,title : &str,keyw: kw) -> InputBox
	{
		unsafe { InputBox::new(ext_func::sage_rust_add_input_box(title.as_ptr(),title.len(),self.id,keyw.pointer)) }
	}

	/// [**new_inputbox()**] - Creates a new Input Box where you can enter text data as simple as a few characters are entire paragraphs. 
	/// 
	/// The Input box is placed in automaitcally in the Dev Window.
	///
	/// - Input boxes can be limited to only numbers, either integer or float values
	/// - Input boxes that accept only numbers can use the range() keyword to set the valid range. 
	/// - Input boxes can be set to automatically bring up an 'invalid' window for empty text or numbers out of range.
	/// - Input boxes can be set to readonly, and can also be set to display '*' for text when entering passwords.
	/// 
	/// **Function Parameter Forms:**
	///
	///		new_inputbox(title : &str)              // Creates in Input box with a a title
	///		new_inputbox_s(title : &str,keyw: kw)   // Used when entering keywords to shape the input box characteristics
	///
	/// **Available Keywords with dev_inputbox()**
	/// 
	/// Keyword | Action
	///---- | ----
	/// **width**  		| -Sets the width of the input box in pixels.  The default is 250 pixels.
	/// **height**        | -Sets the height of the input box in pixels.  The default is set to the default Font for 1 line of text. 
	/// **text**          | -This sets the starting text for the input box.  Otheriwse the input box is left blank at first. 
	/// **font**         | - Sets the font for the input box.  The default is the current font of the window
	/// **numbersonly**  | - Causes the input box to only accept numbers. 
	/// **readonly**     | - Sets the input box as read only so it can be used as a way to place a large amount of text that can be copied.
	/// **textcolor**    | - Sets the color of the text in the input box
	/// **bgcolor**      | - Sets the background color of the text in the input box
	/// **label**        | - Sets a label to the right of the input box. LabelRight, LabelLeft, LabelBottom, and LabelTop can be used to set the location of the label.
	/// **label_color**  | - Sets the color of the label (i.e. opt.label_color("Red"))
	/// **multiline**    | - Sets the input box as a multi-line input box.  This allows more than one line to be entered. A button or some method to end input must be used unless "WantReturn" is specified
	/// **wantreturn**   | - For multi-line boxes, this sends a "return pressed" message when the return key is pressed.
	/// **password**     | - Causes the input box to display '*' for all text.
	/// **noborder**     | - Causes the input box to not use a border so it will blend into the window more seamlessly.
	/// **thickborder,recessed** 	| - These are two different border styles that can be used.
	/// **vscroll, hscroll**      | - Adds a vertical scrollbar (useful for multi-line boxes) and horizontal scroll bar, respectively
	/// **wincolors**  	| - Sets the background input box color and text color to the current window color instead of the default white-and-black colors. 
	/// 
	/// Examples: 
	///
	///     dev_win.new_inputbox();                   // Create input box
	///     dev_win.new_inputbox_s(kw::height(500));  // Create an input box of height 500 pixels
	///     dev_win.new_inputbox_s(kw::font(20) + kw:label("Enter Data"));
	///     dev_win.new_inputbox_s(kw::text("This is the default text")); 		// Create an input box and set initial text.
	///
	pub fn new_input_box(&self,title : &str) -> InputBox
	{
		self.new_input_box_s(title,KW_NONE)
	}

   	/// new_text_widget - Creates a text widget in the Dev Window. 
   	/// 
   	/// - <i>dev_text()</i> returns a <i>TextWidget</i> object where you can control and write to the text widget.
   	/// - The created text widget can be used to create static or dynamic text of any font size in the Dev Window.
	/// - Text Widget Objects returned (<i>TextWidget</i> type) does not need to be kept of the text widget will never be updated once the text widget is created.  Otherwise,
	/// the returned <i>TextWidget</i> object can be used to change the text or other aspects of the text widget.
	///
	/// **Function Parameter Forms:**
	///
	///		new_text_widget(title : &str)              // Creates a text widget with a a title
	///		new_text_widget_s(title : &str,keyw: kw)   // Used when entering keywords to shape the text widget characteristics
	///
	/// **Available Keywords with new_text_widget()**
	/// 
	/// Keyword | Action
	///---- | ----
   	/// text  | - Sets the text of the widget.  This can be set later with <i>text_widget.write()</i>. When text is entered, the text widget is created to the width of the text.  Use the width() parameter to set a width or pad the text with spaces to reserve width.
   	/// textColor  | - Sets the text color of the widget (default is current window text color).  Same as <i>kw::fg_color()</i>
   	/// font       | - Sets the font of the text in the text widget
   	/// textCenter  | - Centers the text inside of the widget (which can be longer than the text itself). - Use <i>text_center_x()</i> and <i>center_x()</i> together to make sure text is centered in the window. This is only needed if the Width of the   	///                         - Text Widget and the text have been specificed separately.
   	/// 
   	/// Examples:
	///
   	///		let widget = dev_win.new_text_widget("This is a text Widget"); // Create a basic text widget with text.  Return value does not need to be kept.
	///
   	///		let widget = dev_win.new_text_widget("This is a text Widget",kw::font(15) + kw::fg_color("red")) // Create a text widget with text.  That has a font size of 15 and a text color of red.
	///
	pub fn new_text_widget_s(&self,title : &str,keyw: kw) -> TextWidget
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_text_widget(title.as_ptr(),title.len(),self.id,keyw.pointer); }
		TextWidget::new(ret_val)
	}

   	/// new_text_widget - Creates a text widget in the Dev Window. 
   	/// 
   	/// - <i>dev_text()</i> returns a <i>TextWidget</i> object where you can control and write to the text widget.
   	/// - The created text widget can be used to create static or dynamic text of any font size in the Dev Window.
	/// - Text Widget Objects returned (<i>TextWidget</i> type) does not need to be kept of the text widget will never be updated once the text widget is created.  Otherwise,
	/// the returned <i>TextWidget</i> object can be used to change the text or other aspects of the text widget.
	///
	/// **Function Parameter Forms:**
	///
	///		new_text_widget(title : &str)              // Creates a text widget with a a title
	///		new_text_widget_s(title : &str,keyw: kw)   // Used when entering keywords to shape the text widget characteristics
	///
	/// **Available Keywords with new_text_widget()**
	/// 
	/// Keyword | Action
	///---- | ----
   	/// text  | - Sets the text of the widget.  This can be set later with <i>text_widget.write()</i>. When text is entered, the text widget is created to the width of the text.  Use the width() parameter to set a width or pad the text with spaces to reserve width.
   	/// textColor  | - Sets the text color of the widget (default is current window text color).  Same as <i>kw::fg_color()</i>
   	/// font       | - Sets the font of the text in the text widget
   	/// textCenter  | - Centers the text inside of the widget (which can be longer than the text itself). - Use <i>text_center_x()</i> and <i>center_x()</i> together to make sure text is centered in the window. This is only needed if the Width of the   	///                         - Text Widget and the text have been specificed separately.
   	/// 
   	/// Examples:
	///
   	///		let widget = dev_win.new_text_widget("This is a text Widget"); // Create a basic text widget with text.  Return value does not need to be kept.
	///
   	///		let widget = dev_win.new_text_widget("This is a text Widget",kw::font(15) + kw::fg_color("red")) // Create a text widget with text.  That has a font size of 15 and a text color of red.
	///
	pub fn new_text_widget(&self,title : &str) -> TextWidget
	{
		self.new_text_widget_s(title,KW_NONE)
	}

    /// new_checkbox() - Create a checkbox in the Dev Window.  The Checkbox is automatically placed. 
    /// 
	/// - The checkbox is unchecked by default. 
	/// - The input title is the text displayed directly to the right of the square checkbox.
	///
	/// **Function Parameter Forms:**
	///
	///		new_checkbox(title : &str)             // Creates a checkbox with the title displayed to the right of the checkbox
	///		new_checkbox_s(title : &str,keyw: kw)  // Used when entering keywords to shape checkbox characteristics
	///
	/// **Some Available Keywords with new_checkbox() (partial list)**
	/// 
    /// - **default** - <i>default(true)</i> sets the check in the checkbox.  <i>default(false)</i> leaves the checkbox unchecked.
    /// - **text_color** - Set the text of the checkbox.  This is the same keyword as <i>fg_color()</i>
    /// - **font** - Set the font of the checkbox.
	///
    /// Examples:
	///
    ///		let checkbox = dev_win.new_checkbox("This is a checkbox");                           // Create an unchecked checkbox
    ///		let checkbox = dev_win.new_checkbox_s("This is a checkbox",kw::default(true));       // Create a checked checkbox
    ///		let checkbox = dev_win.new_checkbox_s("This is a checkbox",kw::text_color("green")); // Create a checkbox with green text.
	///
	pub fn new_checkbox_s(&self,title : &str,keyw: kw) -> Button
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_checkbox(title.as_ptr(),title.len(),self.id,keyw.pointer); }
		Button::new(ret_val,true)
	}

    /// new_checkbox() - Create a checkbox in the Dev Window.  The Checkbox is automatically placed. 
    /// 
	/// - The checkbox is unchecked by default. 
	/// - The input title is the text displayed directly to the right of the square checkbox.
	///
	/// **Function Parameter Forms:**
	///
	///		new_checkbox(title : &str)             // Creates a checkbox with the title displayed to the right of the checkbox
	///		new_checkbox_s(title : &str,keyw: kw)  // Used when entering keywords to shape checkbox characteristics
	///
	/// **Some Available Keywords with new_checkbox() (partial list)**
	/// 
    /// - **default** - <i>default(true)</i> sets the check in the checkbox.  <i>default(false)</i> leaves the checkbox unchecked.
    /// - **text_color** - Set the text of the checkbox.  This is the same keyword as <i>fg_color()</i>
    /// - **font** - Set the font of the checkbox.
	///
    /// Examples:
	///
    ///		let checkbox = dev_win.new_checkbox("This is a checkbox");                           // Create an unchecked checkbox
    ///		let checkbox = dev_win.new_checkbox_s("This is a checkbox",kw::default(true));       // Create a checked checkbox
    ///		let checkbox = dev_win.new_checkbox_s("This is a checkbox",kw::text_color("green")); // Create a checkbox with green text.
	///
	pub fn new_checkbox(&self,title : &str) -> Button
	{
		self.new_checkbox_s(title,KW_NONE)
	}

	/// [**new_button()**] — Create a button in the Dev Window
	/// 
	/// **Function Parameter Forms:**
	///
	///		new_button(location,title : &str)              // Creates a button with the title specified.
	///		new_button_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the button characteristics
	///
	/// - Buttons are automatically sized based on the text title and font (supplied font or default)
	/// - For buttons with short words, e.g. "stop", spaces may be added to make buttons longer, such as "    Stop    ".
	///   - Also see the **<i>width()</>** keyword below
	///
	/// **Some Keywords Available with Buttons**
	///
	/// - size() 	 - Set the size of the button
	/// - width()    - Set the width of the button (height is calculated automatically)
	/// - font() 	 - Set the font used in the button
	/// - hidden()   - Set the button as initially hidden
	/// - disabled() - Set the button as initially disabled
	/// - — There are many more keywords that will be documented in the next release.
	///
	/// **A note about the <i>kw::width()</> keyword**
	/// 
	/// - Using the **<i>kw::width()</>** keyword has a couple great uses:
	///
	///   - When defining multiple buttons, this can create uniform length for buttons, making them look more organized, also allowing lining them up together
	///   - When using the **<i>button.set_text()</>** function to change the text, this can reserve space for the longest text that may later be set in the button.
	///
	/// **[Special Note]**
	///
	/// - Buttons may take a wide variety of shapes and forms, with many keywords to affect their behavior.
	///  - This documentation is still being written and the interface still in-progress.
	/// - More will be added in the next release, with more button types and behavior controls.
	///
	/// **Examples**
	///
	/// 		dev_win.new_button((100,200),"This is a button");                   // Create a button at 100,200
	/// 		dev_win.new_button_s((100,200),"This is a button",kw::font(25));    // Create a button with a font size of 25
	/// 		dev_win.new_button_s((100,200),"Stop",kw::Width(100))				// Create a wider button, since the "stop" title will create a small one if automatic.
	///
	pub fn new_button_s(&self,title : &str,keyw: kw) -> Button
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_button(title.as_ptr(),title.len(),self.id,keyw.pointer); }
		Button::new(ret_val,false)
	}

	/// [**new_button()**] — Create a button in the Dev Window
	/// 
	/// **Function Parameter Forms:**
	///
	///		new_button(location,title : &str)              // Creates a button with the title specified.
	///		new_button_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the button characteristics
	///
	/// - Buttons are automatically sized based on the text title and font (supplied font or default)
	/// - For buttons with short words, e.g. "stop", spaces may be added to make buttons longer, such as "    Stop    ".
	///   - Also see the **<i>width()</>** keyword below
	///
	/// **Some Keywords Available with Buttons**
	///
	/// - size() 	 - Set the size of the button
	/// - width()    - Set the width of the button (height is calculated automatically)
	/// - font() 	 - Set the font used in the button
	/// - hidden()   - Set the button as initially hidden
	/// - disabled() - Set the button as initially disabled
	/// - — There are many more keywords that will be documented in the next release.
	///
	/// **A note about the <i>kw::width()</> keyword**
	/// 
	/// - Using the **<i>kw::width()</>** keyword has a couple great uses:
	///
	///   - When defining multiple buttons, this can create uniform length for buttons, making them look more organized, also allowing lining them up together
	///   - When using the **<i>button.set_text()</>** function to change the text, this can reserve space for the longest text that may later be set in the button.
	///
	/// **[Special Note]**
	///
	/// - Buttons may take a wide variety of shapes and forms, with many keywords to affect their behavior.
	///  - This documentation is still being written and the interface still in-progress.
	/// - More will be added in the next release, with more button types and behavior controls.
	///
	/// **Examples**
	///
	/// 		dev_win.new_button((100,200),"This is a button");                   // Create a button at 100,200
	/// 		dev_win.new_button_s((100,200),"This is a button",kw::font(25));    // Create a button with a font size of 25
	/// 		dev_win.new_button_s((100,200),"Stop",kw::Width(100))				// Create a wider button, since the "stop" title will create a small one if automatic.
	///
	pub fn new_button(&self,title : &str) -> Button
	{
		self.new_button_s(title,KW_NONE)
	}

    /// [**new_combobox()**] -- Creates a Combobox in the DevWindow.  The Combobox is automatically placed. 
    /// 
    /// - A combo box  is like a list box except that it consists of a single tab that expands when activated, 
    /// and rolls back up when released. 
    /// - This allows multiple listbox-style entries to take only the space of the height of one text line. 
    /// - <i>new_combobox()</i> returns a Combobox type object so that items may be added and deleted, and user selections retrieved. 
    /// - When the <i>title()</i> keyword is used, the title appears above the combobox to the left. 
	/// - <i>title_right()</i> and <i>title_left()</i> can be used to place the title to the left or right of the combo box, respectively.
	///
 	/// **Function Parameter Forms:**
	///
	///		new_combobox(title : &str)             // Creates a combo box with the title displayed to the right of the checkbox
	///		new_combobox_s(title : &str,keyw: kw)  // Used when entering keywords to shape combobox characteristics
	///
	/// **Some Available Keywords with new_combobox() (partial list)**
    /// 
    /// - **text** - Initial text in the combobox.  This text can be one line or multiple lines representing multiple entries.  See examples.
    /// - **title_cell** - Tells the combobox to display this string int the combobox tab when no item is selected.  Otherwise the first added item is displayed.
    /// - **default** - Default selection.  This is the index to the default selection (0 is the first selection, 1 the second, etc.)
    /// 
    /// **Examples:**
    /// 
    ///		let my_combobox = dev_win.new_combobox("First Item") 
    ///		let my_combobox = dev_win.new_combobox("First Item\\nSecond Item\\nThird Item") 
    ///		let my_combobox = dev_win.new_combobox(titlecell="This is a combobox") 
    ///		let my_combobox = dev_win.new_combobox_s("First Item\\nSecond Item\\nThird Item",kw::default(2)) 
    ///		let my_combobox = dev_win.new_combobox_s("First Item\\nSecond Item\\nThird Item",kw::title("This is a combobox"));
	///
	pub fn new_combobox_s(&self,title : &str,keyw: kw) -> ComboBox
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_combo_box(title.as_ptr(),title.len(),self.id,keyw.pointer); }
		ComboBox::new(ret_val)
	}

    /// [**new_combobox()**] -- Creates a Combobox in the DevWindow.  The Combobox is automatically placed. 
    /// 
    /// - A combo box  is like a list box except that it consists of a single tab that expands when activated, 
    /// and rolls back up when released. 
    /// - This allows multiple listbox-style entries to take only the space of the height of one text line. 
    /// - <i>new_combobox()</i> returns a Combobox type object so that items may be added and deleted, and user selections retrieved. 
    /// - When the <i>title()</i> keyword is used, the title appears above the combobox to the left. 
	/// - <i>title_right()</i> and <i>title_left()</i> can be used to place the title to the left or right of the combo box, respectively.
	///
 	/// **Function Parameter Forms:**
	///
	///		new_combobox(title : &str)             // Creates a combo box with the title displayed to the right of the checkbox
	///		new_combobox_s(title : &str,keyw: kw)  // Used when entering keywords to shape combobox characteristics
	///
	/// **Some Available Keywords with new_combobox() (partial list)**
    /// 
    /// - **text** - Initial text in the combobox.  This text can be one line or multiple lines representing multiple entries.  See examples.
    /// - **title_cell** - Tells the combobox to display this string int the combobox tab when no item is selected.  Otherwise the first added item is displayed.
    /// - **default** - Default selection.  This is the index to the default selection (0 is the first selection, 1 the second, etc.)
    /// 
    /// **Examples:**
    /// 
    ///		let my_combobox = dev_win.new_combobox("First Item") 
    ///		let my_combobox = dev_win.new_combobox("First Item\\nSecond Item\\nThird Item") 
    ///		let my_combobox = dev_win.new_combobox(titlecell="This is a combobox") 
    ///		let my_combobox = dev_win.new_combobox_s("First Item\\nSecond Item\\nThird Item",kw::default(2)) 
    ///		let my_combobox = dev_win.new_combobox_s("First Item\\nSecond Item\\nThird Item",kw::title("This is a combobox"));
	///
	pub fn new_combobox(&self,title : &str) -> ComboBox
	{
		self.new_combobox_s(title,KW_NONE)
	}

    /// [**new_radio_buttons()**] - Creates a group of Radio Buttons with an optional outer border and label.  
    /// 
	/// - With radio buttons, only one can be selected at a time.  If one is selected, the currently selected radio button is unselected.
    /// - The Radio Button group is placed in the Dev Window automatically.
	/// - One radio button is selected at all times.  This defaults to the first (e.g. position 0) button, but can be changed with <i>radio_button.set_selection()</i>. 
    /// - <i>new_radio_buttons()</i> returns a <i>ButtonGroup</i> object class where the buttons may be queried to see when pressed, and which button was pressed.
	/// - The radio buttons are outline with a thin line, with a title to the upper-left when the <i>title()</i> keyword is used.
	/// - Radio buttons are ordered vertically with one button per-line, unless the <i>horz()</i> and/or <i>columns()</i> keywords are used (see below)
	///
	/// **Function Parameter Forms:**
	///
	///		new_radio_buttons(buttons_text : &str)             // Creates a set of radio buttons
	///		new_radio_buttons_s(buttons_text : &str,keyw: kw)  // Used when entering keywords to shape the radio buttons characteristics
	///
	/// **Parameters:**
    /// 
    /// - button_text  - The radio button text for each radio button, separated by a new line ('\n') - see examples below. 
    ///
	/// **Some Available Keywords with new_radio_buttons() (partial list)**
	///
    /// - **title** - The title/label of the radio button group.  A box is drawn around the radio buttons with the title name.
    /// - **default** - Sets the default index for the highlighted button.  There can only be one active radio button.  default() sets the index of the active/highlighted button (i.e. 0 = first button, 1 = second button, etc.)
    /// - **text_color** - Sets the text color for the radio buttons (same as fg_color())
	/// - **font** - Sets the font of the radio button text
	/// - **horz** - Makes the radio buttons appear one one horizontal area, rather than vertically
	/// - **columns** - Sets the number of radio buttons per-line, so that more than one radio button can appear on a single line, but the radio buttons can also use
	/// multiple lines when there are many buttons.
	///
    /// **Examples:**   
    /// 
    ///		let my_radio_box = dev_win.new_radio_buttons("This is a single radio button button")
    ///		let my_radio_box = dev_win.new_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list"))
    ///		let my_radio_box = dev_win.new_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list") + kw::default(1))
	///
	pub fn new_radio_buttons_s(&self,title : &str,keyw: kw) -> ButtonGroup
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_radio_buttons(title.as_ptr(),title.len(),self.id,keyw.pointer); }
		ButtonGroup::new(ret_val)
	}

    /// [**new_radio_buttons()**] - Creates a group of Radio Buttons with an optional outer border and label.  
    /// 
	/// - With radio buttons, only one can be selected at a time.  If one is selected, the currently selected radio button is unselected.
    /// - The Radio Button group is placed in the Dev Window automatically.
	/// - One radio button is selected at all times.  This defaults to the first (e.g. position 0) button, but can be changed with <i>radio_button.set_selection()</i>. 
    /// - <i>new_radio_buttons()</i> returns a <i>ButtonGroup</i> object class where the buttons may be queried to see when pressed, and which button was pressed.
	/// - The radio buttons are outline with a thin line, with a title to the upper-left when the <i>title()</i> keyword is used.
	/// - Radio buttons are ordered vertically with one button per-line, unless the <i>horz()</i> and/or <i>columns()</i> keywords are used (see below)
	///
	/// **Function Parameter Forms:**
	///
	///		new_radio_buttons(buttons_text : &str)             // Creates a set of radio buttons
	///		new_radio_buttons_s(buttons_text : &str,keyw: kw)  // Used when entering keywords to shape the radio buttons characteristics
	///
	/// **Parameters:**
    /// 
    /// - button_text  - The radio button text for each radio button, separated by a new line ('\n') - see examples below. 
    ///
	/// **Some Available Keywords with new_radio_buttons() (partial list)**
	///
    /// - **title** - The title/label of the radio button group.  A box is drawn around the radio buttons with the title name.
    /// - **default** - Sets the default index for the highlighted button.  There can only be one active radio button.  default() sets the index of the active/highlighted button (i.e. 0 = first button, 1 = second button, etc.)
    /// - **text_color** - Sets the text color for the radio buttons (same as fg_color())
	/// - **font** - Sets the font of the radio button text
	/// - **horz** - Makes the radio buttons appear one one horizontal area, rather than vertically
	/// - **columns** - Sets the number of radio buttons per-line, so that more than one radio button can appear on a single line, but the radio buttons can also use
	/// multiple lines when there are many buttons.
	///
    /// **Examples:**   
    /// 
    ///		let my_radio_box = dev_win.new_radio_buttons("This is a single radio button button")
    ///		let my_radio_box = dev_win.new_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list"))
    ///		let my_radio_box = dev_win.new_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list") + kw::default(1))
	///
	pub fn new_radio_buttons(&self,title : &str) -> ButtonGroup
	{
		self.new_radio_buttons_s(title,KW_NONE)
	}

	pub fn set_bg_bitmap(&self,bitmap : &Bitmap) -> bool
	{
		unsafe { ext_func::sage_rust_dev_set_bg(self.id,bitmap.get_id()) }
	}

   	/// ### new_text() - This function is TBD
	///
	pub fn new_text_s(&self,title : &str,keyw: kw) -> Slider
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_text(title.as_ptr(),title.len(),self.id,keyw.pointer); }
		Slider::new(ret_val)
	}
   	/// ### new_text() - This function is TBD
	///
	pub fn new_text(&self,title : &str) -> Slider
	{
		self.new_text_s(title,KW_NONE)
	}


}

pub struct GdiPath
{
	id : i64,
}

impl GdiPath
{
	pub fn new( _id : i64) -> GdiPath
	{
		
		GdiPath { id: _id }
	}
	pub const fn default() -> GdiPath
	{
		GdiPath { id: 0 }
	}
	/// [**add_rectangle()**] - Add an integer (i32,i32) rectangle to the current path. 
	///
	pub fn add_rectangle(&self,point :  (i32,i32), size: (i32,i32))
	{
		unsafe { ext_func::sage_rust_gdipath_add_rectangle_int(self.id,point.0,point.1,size.0,size.1); }
	}


	/// [**add_polygon_f32()**] - Add a floating-point (f32,f32) array-based polygon to the current path. 
	///
	/// - The input is an a reference to an (f32,f32) array containing the polygon points.  
	/// - Points should be in clockwise order (but may work in counter-clockwise order)
	///
	/// [**note**] : This function is currently split into differently named functions to support multiple types. The Gdi functions are still being implemented, and 
	///the different names will probably be folded into a single <i>**add_polygon()**</i> function with trait overloading, to make it more straightforward to use.
	///              
	pub fn add_polygon_f32(&self,poly: &[(f32,f32)])
	{
		unsafe { ext_func::sage_rust_gdipath_add_polygon_f(self.id,poly.as_ptr(),poly.len()); }
	}

	/// [**add_polygon_f32()**] - Add a floating-point Point<f32> array-based polygon to the current path. 
	///
	/// - The input is an a reference to an Point<f32> array containing the polygon points.  
	/// - Points should be in clockwise order (but may work in counter-clockwise order)
	///
	/// [**note**] : This function is currently split into differently named functions to support multiple types. The Gdi functions are still being implemented, and 
	///the different names will probably be folded into a single <i>**add_polygon()**</i> function with trait overloading, to make it more straightforward to use.
	///    	
	pub fn add_polygon_f(&self,poly: &[Point<f32>])
	{
		unsafe { ext_func::sage_rust_gdipath_add_polygon_f(self.id,poly.as_ptr() as * const (f32,f32),poly.len()); }
	}
	pub fn add_line_f(&self,point1 : (f32,f32),point2 : (f32,f32))
	{
		unsafe { ext_func::sage_rust_gdipath_add_line_f(self.id,point1.0,point1.1,point2.0,point2.1); }
	}

}

impl Drop for GdiPath 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_gdi_create_path(self.id,true); }
    }
}
pub struct GdiBrush
{
	id : i64,
}

impl GdiBrush
{
	pub fn new( _id : i64) -> GdiBrush
	{
		
		GdiBrush { id: _id }
	}
	pub const fn default() -> GdiBrush
	{
		GdiBrush { id: 0 }
	}
}

impl Drop for GdiBrush 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_gdi_create_solid_brush(self.id,0,0,0,0,true); }
    }
}

pub struct GdiPen
{
	id : i64,
}

impl GdiPen
{
	pub fn new( _id : i64) -> GdiPen
	{
		
		GdiPen { id: _id }
	}
	pub const fn default() -> GdiPen
	{
		GdiPen { id: 0 }
	}
}

impl Drop for GdiPen 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_gdi_create_pen(self.id,0,0,0,0,0.0,true); }
    }
}

//
//#[derive(Copy,Clone)]
pub struct Gdi
{
	id : i64,
}
pub trait GdiBrushCreate<ColorType>
{
	/// [**new_solid_brush()**] - Create a brush to use with Gdi functions
	///
	/// - Brushes can be Rgb Colors, or RGBA colors with opacity levels in the color.
	///
	/// **Function Parameter Forms:** 
	///	
	///			new_solid_brush()                                // Creates a new brush (with default color Black)
	///			new_solid_brush_s(RgbColor)                      // RgbColor Type, e.g. pan_color::beige, RgbColor::new(0,255,0)
	///			new_solid_brush_s((i32,i32,i32))                 // e.g. (0,255,0)
	///			new_solid_brush_s(RgbColorA)                     // RgbColorA Type, e.g. pan_color_a::beige(192), RgbColorA::new(0,255,0,292)
	///			new_solid_brush_s(color : (i32,i32,i32,i32))     // e.g. (0,255,0,192)
	///	
	/// - brush.set_color() can be used to change the color or set the initial color.
	///   - This can be used when using new_solid_brush() without setting color on initiation.
	///
	/// **Example**
	///
	/// 	let gdi = win.gdi
	///  	let brush = new_solid_brush(pan_color::red)
	///  	gdi.fill_rectangle(&brush,(100,200),(400,400))
	///
	fn new_solid_brush_s(&self,color : ColorType) -> GdiBrush;	

	/// [**new_pen()**] - Create a pen to use with Gdi functions
	///
	/// - Pens can be Rgb Colors, or RGBA colors with opacity levels in the color.
	///
	/// **Function Parameter Forms:** 
	///	
	///			new_pen()                              // Creates a new pen (with default color Black)
	///			new_pen(RgbColor)                      // RgbColor Type, e.g. pan_color::beige, RgbColor::new(0,255,0)
	///			new_pen((i32,i32,i32))                 // e.g. (0,255,0)
	///			new_pen(RgbColorA)                     // RgbColorA Type, e.g. pan_color_a::beige(192), RgbColorA::new(0,255,0,292)
	///			new_pen(color : (i32,i32,i32,i32))     // e.g. (0,255,0,192)
	///	
	/// - pen.set_color() can be used to change the color or set the initial color.
	///   - This can be used when using new_pen() without setting color on initiation.
	///
	/// **Example**
	///
	/// 	let gdi = win.gdi
	///  	let pen = new_solid_pen(pan_color::red)
	///  	gdi.draw_rectangle(&pen,(100,200),(400,400))
	///
	fn new_pen_s(&self,color : ColorType,width : f32) -> GdiPen;	
}

pub enum AutoUpdateType
{
	//  Update() must be used to update the window.
	//
	Off,

	// Updates 10-20 millisconds.  Use an extra Update() at the end of all routines to make sure it is updated.
	//
	On,

	// Same as On, but installs a timer to ensure update status.  
	OnTime,

	// Updates after any action that outputs to the screen, no matter how small. 
	// *** Note: Immediate can slow down your program.  Use for diagnostics, status windows or other places where you don't want to do an Update(), but 
	// *** also don't care about it performing an Update() at every step
	//
	Immediate, 
}
pub enum GdiSmoothingMode
{
	Default,    
	HighSpeed,   
	HighQuality, 
	None,        
	AntiAlias,   
	Smooth, 
}

impl Gdi
{
   	fn new( window_id : i64) -> Gdi
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_window_get_gdi(window_id); }
		Gdi { id: _gdi }
	}
	pub const fn default() -> Gdi
	{
		Gdi { id: 0 }
	}

	pub fn clone(&self) -> Gdi
	{
		unsafe { Gdi { id: ext_func::sage_rust_gdi_clone(self.id), } }
	}

	/// [**new_path**] - Create a new shape path for the current GDI
	///
	/// - A GDI path is a way to define complex shapes by connecting lines and curves, which can then be drawn, filled, or used to limit drawing areas in the window associated with the GDI object.
	/// - You can draw the path to outline its shape or fill it to create a solid figure. 
	///   - Drawing a path with <i>**gdi.draw_path()**</i> shows only the edges, based on the width of the pen used.
	///   - Drawing a path with <i>**gdi.fill_path()**</i> fills the path fills the entire enclosed area defined by the path	
	/// - Multiple paths can be created and used in the same GDI object at the same time.
	/// - Multiple GDI objects can be created simultaneously
	///   - This can be helpful when using differerent transforms for different drawn areas.
	///
	pub fn new_path(&self) -> GdiPath
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_path(0,false); }
		GdiPath { id: _gdi }
	}

	/// [**new_solid_brush()**] - Create a brush to use with Gdi functions
	///
	/// - Brushes can be Rgb Colors, or RGBA colors with opacity levels in the color.
	///
	/// **Function Parameter Forms:** 
	///	
	///			new_solid_brush()                                // Creates a new brush (with default color Black)
	///			new_solid_brush_s(RgbColor)                      // RgbColor Type, e.g. pan_color::beige, RgbColor::new(0,255,0)
	///			new_solid_brush_s((i32,i32,i32))                 // e.g. (0,255,0)
	///			new_solid_brush_s(RgbColorA)                     // RgbColorA Type, e.g. pan_color_a::beige(192), RgbColorA::new(0,255,0,292)
	///			new_solid_brush_s(color : (i32,i32,i32,i32))     // e.g. (0,255,0,192)
	///	
	/// - brush.set_color() can be used to change the color or set the initial color.
	///   - This can be used when using new_solid_brush() without setting color on initiation.
	///
	/// **Example**
	///
	/// 	let gdi = win.gdi
	///  	let brush = new_solid_brush(pan_color::red)
	///  	gdi.fill_rectangle(&brush,(100,200),(400,400))
	///
	pub fn new_solid_brush(&self) -> GdiBrush
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_solid_brush(0,0,0,0,255,false); }
		GdiBrush { id: _gdi }
	}
	fn _new_solid_brush_s(&self,ref color : RgbColor) -> GdiBrush
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_solid_brush(0,color.red,color.green,color.blue,255,false); }
		GdiBrush { id: _gdi }
	}
	fn _new_solid_brush_a_s(&self,ref color : RgbColorA) -> GdiBrush
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_solid_brush(0,color.red,color.green,color.blue,color.opacity,false); }
		GdiBrush { id: _gdi }
	}
	fn _new_solid_brush_i32(&self,ref color : (i32,i32,i32)) -> GdiBrush
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_solid_brush(0,color.0,color.1,color.2,255,false); }
		GdiBrush { id: _gdi }
	}
	fn _new_solid_brush_i32_a(&self,ref color : (i32,i32,i32,i32)) -> GdiBrush
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_solid_brush(0,color.0,color.1,color.2,color.3,false); }
		GdiBrush { id: _gdi }
	}

	// pen

	/// [**new_pen()**] - Create a pen to use with Gdi functions
	///
	/// - Pens can be Rgb Colors, or RGBA colors with opacity levels in the color.
	///
	/// **Function Parameter Forms:** 
	///	
	///			new_pen()                              // Creates a new pen (with default color Black)
	///			new_pen(RgbColor)                      // RgbColor Type, e.g. pan_color::beige, RgbColor::new(0,255,0)
	///			new_pen((i32,i32,i32))                 // e.g. (0,255,0)
	///			new_pen(RgbColorA)                     // RgbColorA Type, e.g. pan_color_a::beige(192), RgbColorA::new(0,255,0,292)
	///			new_pen(color : (i32,i32,i32,i32))     // e.g. (0,255,0,192)
	///	
	/// - pen.set_color() can be used to change the color or set the initial color.
	///   - This can be used when using new_pen() without setting color on initiation.
	///
	/// **Example**
	///
	/// 	let gdi = win.gdi
	///  	let pen = new_solid_pen(pan_color::red)
	///  	gdi.draw_rectangle(&pen,(100,200),(400,400))
	///
	pub fn new_pen(&self) -> GdiPen
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_pen(0,0,0,0,255,0.0,false); }
		GdiPen { id: _gdi }
	}
	fn _new_pen_s(&self,ref color : RgbColor,width : f32) -> GdiPen
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_pen(0,color.red,color.green,color.blue,255,width,false); }
		GdiPen { id: _gdi }
	}
	fn _new_pen_a_s(&self,ref color : RgbColorA,width : f32) -> GdiPen
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_pen(0,color.red,color.green,color.blue,color.opacity,width,false); }
		GdiPen { id: _gdi }
	}
	fn _new_pen_i32(&self,ref color : (i32,i32,i32),width : f32) -> GdiPen
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_pen(0,color.0,color.1,color.2,255,width,false); }
		GdiPen { id: _gdi }
	}
	fn _new_pen_i32_a(&self,ref color : (i32,i32,i32,i32),width : f32) -> GdiPen
	{
		let _gdi : i64;
		unsafe { _gdi = ext_func::sage_rust_gdi_create_pen(0,color.0,color.1,color.2,color.3,width,false); }
		GdiPen { id: _gdi }
	}
	/// [**draw_path()**] - Draws the outline of the current GDI path using the selected pen.
	///
	/// - Use this to display the shape’s edges without filling its interior.
	/// - The drawn edges will be the color and thickness defined in the pen used
	//
	pub fn draw_path(&self,path : &GdiPath, ref brush : &GdiPen)
	{
		unsafe { ext_func::sage_rust_gdipath_draw_path(self.id, path.id, brush.id,false); }
	}

	/// [**fill_path()**] - Fills the interior of the current GDI path using the selected brush.
	///
	/// - Use this to create solid shapes based on the defined path.
	/// - The filled shape will be the color and opacity level of the brush used.
	///
	pub fn fill_path(&self,path : &GdiPath, brush : &GdiBrush)
	{
		unsafe { ext_func::sage_rust_gdipath_draw_path(self.id, path.id, brush.id,true); }
	}

	/// [**draw_line_f()**] - Draws a straight line between two points using the selected pen.
	///
	/// - Useful for creating borders, connectors, or simple shapes.
	///
	/// [**note**] : This function is currently split into differently named functions to support multiple types. The Gdi functions are still being implemented, and 
	///the different names will probably be folded into a single <i>**draw_line()**</i> function with trait overloading, to make it more straightforward to use.
	///         
	pub fn draw_line_f(&self,pen : &GdiPen,point1 : (f32,f32), point2 : (f32,f32))
	{
		unsafe { ext_func::sage_rust_gdi_draw_line_f(self.id,pen.id,point1.0,point1.1,point2.0,point2.1); }
	}

	/// [**draw_lines_f()**] - Draws a series of connected lines through a list of points using the selected pen.
	///
	/// - The points array should contain at least two points; each point is connected to the next in sequence.
	/// - Useful for creating polygonal lines, outlines, or specific shapes from a defined point path.
	///
	/// **Function Parameter Forms:** 
	///	
	///			draw_lines_f(&GdiPen,lines: &Vec<(f32,f32)>)
	///			draw_lines_size_f(&GdiPen,lines: &Vec<(f32,f32)>,size : i32)
	///
	/// **Parameters:**
    /// 
    /// - **`pen`**  - Gdi object to use to draw lines (usually win.gdi, but may also be one created elsewhere)
    /// - **`lines`** - (f32,f32) vector reference containing the points to connect.
	///                 - note: this will expand later to other types, such is <i>(i32,i32), Point&lt;i32> and Point&lt;f32></i>
	/// - **`size`** - When using <i>**draw_lines_size()**</i>, this is the number of lines to draw, when the value is less than the actual array size.
	///                - <i>**draw_lines()**</i> assumes the entire set of points in the array are to be drawn.
	///                                     
	/// [**note**] : This function is currently split into differently named functions to support multiple types. The Gdi functions are still being implemented, and 
	///the different names will probably be folded into a single <i>**draw_lines()**</i> function with trait overloading, to make it more straightforward to use.
	///         
	pub fn draw_lines_f(&self,pen : &GdiPen,lines: &Vec<(f32,f32)>)
	{
		unsafe { ext_func::sage_rust_gdi_draw_lines_f(self.id,pen.id,lines.as_ptr(),lines.len()); }
	}
	/// [**draw_lines_f()**] - Draws a series of connected lines through a list of points using the selected pen.
	///
	/// - The points array should contain at least two points; each point is connected to the next in sequence.
	/// - Useful for creating polygonal lines, outlines, or specific shapes from a defined point path.
	///
	/// **Function Parameter Forms:** 
	///	
	///			draw_lines_f(&GdiPen,lines: &Vec<(f32,f32)>)
	///			draw_lines_size_f(&GdiPen,lines: &Vec<(f32,f32)>,size : i32)
	///
	/// **Parameters:**
    /// 
    /// - **`pen`**  - Gdi object to use to draw lines (usually win.gdi, but may also be one created elsewhere)
    /// - **`lines`** - (f32,f32) vector reference containing the points to connect.
	///                 - note: this will expand later to other types, such is <i>(i32,i32), Point&lt;i32> and Point&lt;f32></i>
	/// - **`size`** - When using <i>**draw_lines_size()**</i>, this is the number of lines to draw, when the value is less than the actual array size.
	///                - <i>**draw_lines()**</i> assumes the entire set of points in the array are to be drawn.
	///                                     
	/// [**note**] : This function is currently split into differently named functions to support multiple types. The Gdi functions are still being implemented, and 
	///the different names will probably be folded into a single <i>**draw_lines()**</i> function with trait overloading, to make it more straightforward to use.
	///         
	pub fn draw_lines_size_f(&self,pen : &GdiPen,lines: &Vec<(f32,f32)>,size : i32)
	{
		let mut out_size = size;
		if size < 0 { out_size = 0; }
		if size > lines.len() as i32 { out_size = lines.len() as i32 }

		unsafe { ext_func::sage_rust_gdi_draw_lines_f(self.id,pen.id,lines.as_ptr(),out_size as usize); }
	}

	/// [**fill_circle_rgb_a()**] - fille the interior of a circle
	///
	/// - See <i>**fill_circle()**</i> to draw a circle with a pen rather than a specific color
	///	- See <i>**draw_circle()**</i> to draw an open/wireframe circle rather than a filled circle.
	///
	/// **Parameters:**
    /// 
    /// - **`center`**  - GDI object to use to draw lines (usually <i>**win.gdi**</i>, but may also be one created elsewhere)
    /// - **`radius`** - (f32,f32) vector reference containing the points to connect.
    /// - **`color`** - Color to draw the circle
	///				    - compared to <i>**draw_circle()**</i>, this function creates and deletes the pen for you in the desired color.
	/// 
	/// **Example:**
	///
	/// 	win.fill_circle_rgb((500,500),200,pan_color::Red)
	///
	pub fn fill_circle_rgb_a(&self,center : (f32,f32), radius : f32,color : RgbColorA)
	{
		unsafe { ext_func::sage_rust_gdi_draw_ellipse_rgb(self.id,center.0,center.1,radius,radius,color.red,color.green,color.blue,color.opacity,0.0,true); }

	}
	
	/// [**draw_circle_rgb()**] - Draws the outline of a circle
	///
	/// - See <i>**draw_circle()**</i> to draw a circle with a pen rather than a specific color
	///	- See <i>**fill_circle()**</i> to draw filled circle rather than an open/wireframe circle
	///
	/// **Parameters:**
    /// 
    /// - **`center`**  - GDI object to use to draw lines (usually <i>**win.gdi**</i>, but may also be one created elsewhere)
    /// - **`radius`** - (f32,f32) vector reference containing the points to connect.
    /// - **`color`** - Color to draw the circle
	///				    - compared to <i>**draw_circle()**</i>, this function creates and deletes the pen for you in the desired color.
	/// 
	/// **Example:**
	///
	/// 	win.draw_circle_rgb((500,500),200,pan_color::Red)
	///
	pub fn draw_circle_rgb(&self,center : (f32,f32), radius : f32,color : RgbColor)
	{
		unsafe { ext_func::sage_rust_gdi_draw_ellipse_rgb(self.id,center.0,center.1,radius,radius,color.red,color.green,color.blue,255,0.0,false); }
	}

	/// [**set_smoothing_mode()**] - Sets the graphics anti-aliasing ON or OFF
	///
	/// - Default smoothing is Smooth (anti-aliased)
	/// - Anti-aliased (or smooth) graphics makes shapes blend in much better, and floating-point (x,y), (width,height) and radius values
	///   anti-alias properly in the window. 
	///   - With anti-aliasing off (e.g. None), these values are clipped to the nearest integer.
	///   - Shapes such as circles, rectangles, triangles, polygons, paths, lines, etc. respond to this mode.
	/// - Turning Smoothing OFF can be useful for performance under some circumstances
	/// - Some programs may prefer to have no anti-aliasing, leading to an edgier look 
	///	- [**note**] : This function is specific to the GDI object, and will not affect other GDIs that have been created.
	///	  - When calling <i>**win.set_smoothing_mode()**</i>, it uses the windows pre-allocated GDI, which is the equivalent of calling <i>**win.gdi.set_smoothing_mode()**</i>
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_smoothing_mode(smoothing_mode : GdiSmoothingMode)
	///
	/// **GDiSmoothingMode Values**
	/// 
	///		 Default     // Default setting (anti-aliasing/smooth)    
	///		 HighSpeed   // Does not anti-alias
	///		 HighQuality // Anti-Aliases
	///		 None        // Does not anti-alias
	///		 AntiAlias   // Anti-Aliases   
	///		 Smooth      // Anti-Aliases
	///
	/// **Notes:**
	///
	/// - Note that the values above are mostly duplicates.  The two main values that are useful are **`None`**, **`AntiAlias`** (or **`Smooth`**). 
	/// - These come from GDI values which are basically duplicated, for some reason, but are left here to support GDI usage in full.
	///
	/// **Examples**
	///
	///			gdi.set_smoothing_mode(GdiSmoothingMode::None);
	///
	pub fn set_smoothing_mode(&self,smoothing_mode : GdiSmoothingMode)
	{
		unsafe { ext_func::sage_rust_gdi_set_smoothing_mode(self.id,smoothing_mode as i32); }
	}

}

impl Drop for Gdi 
{
	fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_gdi_delete(self.id); }
	}
}
impl GdiBrushCreate<RgbColor> for Gdi
{

	fn new_solid_brush_s(&self,color : RgbColor) -> GdiBrush { self._new_solid_brush_s(color) }
	fn new_pen_s(&self,color : RgbColor,width : f32) -> GdiPen { self._new_pen_s(color,width) }
}
impl GdiBrushCreate<RgbColorA> for Gdi
{

	fn new_solid_brush_s(&self,color : RgbColorA) -> GdiBrush { self._new_solid_brush_a_s(color) }
	fn new_pen_s(&self,color : RgbColorA,width : f32) -> GdiPen { self._new_pen_a_s(color,width) }
}
impl GdiBrushCreate<(i32,i32,i32)> for Gdi
{

	fn new_solid_brush_s(&self,color : (i32,i32,i32)) -> GdiBrush { self._new_solid_brush_i32(color) }
	fn new_pen_s(&self,color : (i32,i32,i32),width : f32) -> GdiPen { self._new_pen_i32(color,width) }
}
impl GdiBrushCreate<(i32,i32,i32,i32)> for Gdi
{

	fn new_solid_brush_s(&self,color : (i32,i32,i32,i32)) -> GdiBrush { self._new_solid_brush_i32_a(color) }
	fn new_pen_s(&self,color : (i32,i32,i32,i32),width : f32) -> GdiPen { self._new_pen_i32_a(color,width) }
}

pub struct Pgr
{
	id : i64, 
}

impl Pgr
{
	pub fn copy(&self) -> Pgr
	{
		Pgr {id: self.id  }
	}
	pub const fn default() -> Pgr
	{
		Pgr { id: 0 }
	}

	/// [**is_valid()**] - Returns whether or not the PGR file in memory is a valid and verified PGR file
	///
	/// - PGR Files are CRC checked when loaded.
	/// - If the PGR file is a valid, working PGR file, <i>**is_valid()**</i> returns `true`. 
	/// - if the PGR file was not found or did not pass the CRC check, then <i>**is_valid()**</i> will return `false`.
	///
	pub fn is_valid(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_pgr_is_valid(self.id,true); }
		ret_val
	}

	/// [**read_image_file32()**] - Reads an embedded image file and returns a 32-bit RGB bitmap in a Bitmap object. 
	///
    /// - Embedded Input files may be **.JPG, .BMP, or .PNG.** 
	/// - See <i>**read_image_file()**</i> to load files without alpha channels or transparencies into a 24-bit bitmap.
	///   - Most functions support 32-bitmaps directly, but there is some flexibility in using 24-bit bitmaps when there is no alpha-channel/mask in the image.
	///
	/// **Example:** 
	/// 
	/// 	let bitmap = pgr.read_image_file_32("Bitmaps:my_bitmap32")	// Embeded 32-bit png, or bitmap file
	///
	pub fn read_image_file32(&self,file_name : &str) -> Bitmap32
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_pgr_read_image_file_str(self.id,file_name.as_ptr(),file_name.len(),true); }
		Bitmap32::create(ret_val)
	}

	/// [**read_image_file()**] - Reads an embedded image file and returns a 32-bit RGB bitmap in a Bitmap object. 
	///
    /// - Embedded Input files may be **.JPG, .BMP, or .PNG.** 
	/// - See <i>**read_image_file32()**</i> to load files with alpha channels or transparencies into a 32-bit bitmap.
	///
	/// **Example:** 
	/// 
	///		let bitmap = pgr.read_image_file("Bitmaps:my_bitmap")	// Embeded jpeg, png, or bitmap file
	///
	pub fn read_image_file(&self,file_name : &str) -> Bitmap
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_pgr_read_image_file_str(self.id,file_name.as_ptr(),file_name.len(),false); }
		Bitmap::create(ret_val)
	}


	/// [**read_raw_binary_file()**] — Reads an embedded file and returns a Vec&lt;u8> with the raw file data 
	///
    /// - Embedded Input files are any format. 
	/// - Test for an empty vector to ensure the file was found and read. 
	/// - The size of the vector (when not empty) is the size of the file itself.
	///
	/// **Example:** 
	/// 
	///		let my_data = pgr.read_raw_binary_file("my_files:my_binrary_file")	// A binary file of any type
	///
	/// **Returns**
	///
	/// - If the file was found, a Vec&lt;u8> is returned with the file data
	/// - If the filw was not found, an empty Vec&lt;u8> is returned.
	///
	pub fn read_raw_binary_file(&self,file : & str) -> Vec<u8>
	{
		unsafe {
			let vector = ext_func::sage_rust_pgr_read_binary_file(self.id,file.as_ptr(), file.len());
			if vector.len > 0 { Vec::from_raw_parts(vector.data, vector.len, vector.len) } else { vec![] }
		}
	}

	/// [**read_pair_i32()**] — Reads an a pair in the .PGR file that was in the original text files
	///
	///
	/// **Function Parameter Forms:** 
	///	
	///			read_pair_i32(key)                  // Returns the pair associated with key.  If not found, None is returned.
	///			read_pair_i32_or(key,default_value) // Returns the pair associated with the key, or default_value is not found
	///			read_pair_i32_or_0(key)             // Returns the parr associated with the key, or (0,0) if not found.
	///                         
	/// **Parameters**
	///
	/// - **key** — the text name of the pair, value, e.g. "my_value = 5,6" as embeded in the original .txt file before .pgr compilation
	/// - **default** — (only when using <i>read_pair_i32_or()</i>) - The (i32,i32) value to return if the key value was not found.
	///
	/// **Example from the original .text file (before compiled to the .PGR) file**
	///
	/// 	my_pair = 123,456
	///
	/// **Returns**
	///
	/// - **read_pair_i32()** — returns Some((i32,i32)) if found, or None if not found.
	/// - **read_pair_i32_or()** — returns the key value found in (i32,i32) form, or the default_value parameter if not found
	/// - **read_pair_i32_or_0()** — returns the key value find in (i32,i32) form, or (0,0) if the key value was not found.
	///
	/// **Examples**
	///
	///		let found_pair = pgr.read_pair_i32("my_pair)"            // Returns Some((i32.i32)) or None
	///		let found_pair = pgr.read_pair_i32_or("my_pair,(12,13))" // Returns Key value if found, or (12,13) if not found
	///		let found_pair = pgr.read_pair_i32_or_0("my_pair"        // Returns Key value if found, or (0,0) if not found
	///
	pub fn read_pair_i32(&self,key : &str) -> Option<(i32,i32)> 
	{ 
		unsafe {
		let  ret = ext_func::sage_rust_pgr_read_int_pair(self.id,key.as_ptr(),key.len());
		if !ret.found { return None; }

		Some((ret.x,ret.y)) 
		}

	}

	/// [**read_pair_i32()**] — Reads an a pair in the .PGR file that was in the original text files
	///
	///
	/// **Function Parameter Forms:** 
	///	
	///			read_pair_i32(key)                  // Returns the pair associated with key.  If not found, None is returned.
	///			read_pair_i32_or(key,default_value) // Returns the pair associated with the key, or default_value is not found
	///			read_pair_i32_or_0(key)             // Returns the parr associated with the key, or (0,0) if not found.
	///                         
	/// **Parameters**
	///
	/// - **key** — the text name of the pair, value, e.g. "my_value = 5,6" as embeded in the original .txt file before .pgr compilation
	/// - **default** — (only when using <i>read_pair_i32_or()</i>) - The (i32,i32) value to return if the key value was not found.
	///
	/// **Example from the original .text file (before compiled to the .PGR) file**
	///
	/// 	my_pair = 123,456
	///
	/// **Returns**
	///
	/// - **read_pair_i32()** — returns Some((i32,i32)) if found, or None if not found.
	/// - **read_pair_i32_or()** — returns the key value found in (i32,i32) form, or the default_value parameter if not found
	/// - **read_pair_i32_or_0()** — returns the key value find in (i32,i32) form, or (0,0) if the key value was not found.
	///
	/// **Examples**
	///
	///		let found_pair = pgr.read_pair_i32("my_pair)"            // Returns Some((i32.i32)) or None
	///		let found_pair = pgr.read_pair_i32_or("my_pair,(12,13))" // Returns Key value if found, or (12,13) if not found
	///		let found_pair = pgr.read_pair_i32_or_0("my_pair"        // Returns Key value if found, or (0,0) if not found
	///
	pub fn read_pair_i32_or(&self,key : &str,default : (i32,i32)) -> (i32,i32) 
	{ 
		unsafe {
		let  ret = ext_func::sage_rust_pgr_read_int_pair(self.id,key.as_ptr(),key.len());

		if ret.found { (ret.x,ret.y) } else { default }
		}

	}

	/// [**read_pair_i32()**] — Reads a pair in the .PGR file defined in the original text file converted to a .PGR file.
	///
	///
	/// **Function Parameter Forms:** 
	///	
	///			read_pair_i32(key)                  // Returns the pair associated with key.  If not found, None is returned.
	///			read_pair_i32_or(key,default_value) // Returns the pair associated with the key, or default_value if not found
	///			read_pair_i32_or_0(key)             // Returns the parr associated with the key, or (0,0) if not found.
	///                         
	/// **Parameters**
	///
	/// - **key** — the text name of the pair, value, e.g. "my_value = 5,6" as embeded in the original .txt file before .pgr compilation
	/// - **default_value** — (only when using <i>read_pair_i32_or()</i>) - The (i32,i32) value to return if the key value was not found.
	///
	/// **Example from the original .text file (before compiled to the .PGR) file**
	///
	/// 	my_pair = 123,456
	///
	/// **Returns**
	///
	/// - **read_pair_i32()** — returns Some((i32,i32)) if found, or None if not found.
	/// - **read_pair_i32_or()** — returns the key value found in (i32,i32) form, or the default_value parameter if not found
	/// - **read_pair_i32_or_0()** — returns the key value found in (i32,i32) form, or (0,0) if the key value was not found.
	///
	/// **Examples**
	///
	///		let found_pair = pgr.read_pair_i32("my_pair")            // Returns Some((i32.i32)) or None
	///		let found_pair = pgr.read_pair_i32_or("my_pair,(12,13))" // Returns Key value if found, or (12,13) if not found
	///		let found_pair = pgr.read_pair_i32_or_0("my_pair")       // Returns Key value if found, or (0,0) if not found
	///
	pub fn read_pair_i32_or_0(&self,key : &str) -> (i32,i32) { self.read_pair_i32_or(key,(0,0)) }


	/// [**read_str()**] — Reads a text string in the .PGR file defined in the original text file converted to a .PGR file.
	///
	/// **Function Parameter Forms:** 
	///	
	///			read_str(key)                   // Returns the pair associated with string.  If not found, None is returned.
	///			read_str_or(key,default_string) // Returns the pair associated with the key, or default_string if not found
	///			read_str_or_empty(key)          // Returns the parr associated with the key, or an empty String if not found.
	///                         
	/// **Parameters**
	///
	/// - **key** — the text name of the pair, value, e.g. "my_string = Hello World" as embeded in the original .txt file before .pgr compilation
	/// - **default_string** — (only when using <i>read_str_or()</i>) - The String to return if the key string was not found.
	///
	/// **Example from the original .text file (before compiled to the .PGR) file**
	///
	/// 	my_string = "Hello  World"      // Used to read string in a case-sensitive and literal manner (i.e. spaces, multiple words, cases, etc.)
	///                                     // (e.g. returns "Hello  World", including the extra space) 			
	/// 	my_string = Hello               // Used for identifiers, which returns an upper-case translation, e.g. returns "HELLO" 	
	///
	/// **Returns**
	///
	/// - **read_str()** — returns Some(String) if found, or None if not found.
	/// - **read_str_or()** — returns the key string found in String form, or the default_string String if not found
	/// - **read_str_or_empty()** — returns the key string found in Sring form, or an empty styring if the key string was not found.
	///
	/// **Examples**
	///
	///		let found_pair = pgr.read_str("my_string")                    // Returns Some(String) or None
	///		let found_pair = pgr.read_str_or("my_my_string","Not Found")  // Returns Key string if found, or "Not Found" if not found
	///		let found_pair = pgr.read_str_or_empty("my_string")           // Returns Key value if found, or an empty String if not found
	///
	pub fn read_str(&self,key : &str) -> Option<String> 
	{ 
		unsafe {
			let  ret = ext_func::sage_rust_pgr_read_string(self.id,key.as_ptr(),key.len());

			if !ret.found { return None; }

			if ret.len > 0 { Some(String::from_raw_parts(ret.data,ret.len,ret.len)) } else { Some(String::new()) } 
		}
	}

	/// [**read_str()**] — Reads a text string in the .PGR file defined in the original text file converted to a .PGR file.
	///
	/// **Function Parameter Forms:** 
	///	
	///			read_str(key)                   // Returns the pair associated with string.  If not found, None is returned.
	///			read_str_or(key,default_string) // Returns the pair associated with the key, or default_string if not found
	///			read_str_or_empty(key)          // Returns the parr associated with the key, or an empty String if not found.
	///                         
	/// **Parameters**
	///
	/// - **key** — the text name of the pair, value, e.g. "my_string = Hello World" as embeded in the original .txt file before .pgr compilation
	/// - **default_string** — (only when using <i>read_str_or()</i>) - The String to return if the key string was not found.
	///
	/// **Example from the original .text file (before compiled to the .PGR) file**
	///
	/// 	my_string = "Hello  World"      // Used to read string in a case-sensitive and literal manner (i.e. spaces, multiple words, cases, etc.)
	///                                     // (e.g. returns "Hello  World", including the extra space) 			
	/// 	my_string = Hello               // Used for identifiers, which returns an upper-case translation, e.g. returns "HELLO" 	
	///
	/// **Returns**
	///
	/// - **read_str()** — returns Some(String) if found, or None if not found.
	/// - **read_str_or()** — returns the key string found in String form, or the default_string String if not found
	/// - **read_str_or_empty()** — returns the key string found in Sring form, or an empty styring if the key string was not found.
	///
	/// **Examples**
	///
	///		let found_pair = pgr.read_str("my_string")                    // Returns Some(String) or None
	///		let found_pair = pgr.read_str_or("my_my_string","Not Found")  // Returns Key string if found, or "Not Found" if not found
	///		let found_pair = pgr.read_str_or_empty("my_string")           // Returns Key value if found, or an empty String if not found
	///
	pub fn read_str_or(&self,key : &str,default : &str) -> String 
	{ 
		unsafe {
			let  ret = ext_func::sage_rust_pgr_read_string(self.id,key.as_ptr(),key.len());

			if !ret.found || ret.len == 0 { return default.to_string(); }

			String::from_raw_parts(ret.data,ret.len,ret.len) 
		}
	}

	/// [**read_str()**] — Reads a text string in the .PGR file defined in the original text file converted to a .PGR file.
	///
	/// **Function Parameter Forms:** 
	///	
	///			read_str(key)                   // Returns the pair associated with string.  If not found, None is returned.
	///			read_str_or(key,default_string) // Returns the pair associated with the key, or default_string if not found
	///			read_str_or_empty(key)          // Returns the parr associated with the key, or an empty String if not found.
	///                         
	/// **Parameters**
	///
	/// - **key** — the text name of the pair, value, e.g. "my_string = Hello World" as embeded in the original .txt file before .pgr compilation
	/// - **default_string** — (only when using <i>read_str_or()</i>) - The String to return if the key string was not found.
	///
	/// **Example from the original .text file (before compiled to the .PGR) file**
	///
	/// 	my_string = "Hello  World"      // Used to read string in a case-sensitive and literal manner (i.e. spaces, multiple words, cases, etc.)
	///                                     // (e.g. returns "Hello  World", including the extra space) 			
	/// 	my_string = Hello               // Used for identifiers, which returns an upper-case translation, e.g. returns "HELLO" 	
	///
	/// **Returns**
	///
	/// - **read_str()** — returns Some(String) if found, or None if not found.
	/// - **read_str_or()** — returns the key string found in String form, or the default_string String if not found
	/// - **read_str_or_empty()** — returns the key string found in Sring form, or an empty styring if the key string was not found.
	///
	/// **Examples**
	///
	///		let found_pair = pgr.read_str("my_string")                    // Returns Some(String) or None
	///		let found_pair = pgr.read_str_or("my_my_string","Not Found")  // Returns Key string if found, or "Not Found" if not found
	///		let found_pair = pgr.read_str_or_empty("my_string")           // Returns Key value if found, or an empty String if not found
	///
	pub fn read_str_or_empty(&self,key : &str) -> String
	{
		unsafe {
			let  ret = ext_func::sage_rust_pgr_read_string(self.id,key.as_ptr(),key.len());

			if !ret.found || ret.len == 0 { return String::new(); }

			String::from_raw_parts(ret.data,ret.len,ret.len) 
		}
	}
    fn create(_id : i64 ) -> Pgr { Pgr { id : _id } }
}

impl Drop for Pgr 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_pgr_delete(self.id); }
    }
}
pub struct Bitmap32
{
	id : i64,
}

impl Bitmap32
{
    pub fn new() -> Bitmap32
	{
		Bitmap32 { id: 0 }
	}

	/// [default()] - Returns an empty bitmap that can later be used or copied over. 
	///
	pub const fn default() -> Bitmap32
	{
		Bitmap32 { id: 0 }
	}

	/// [**is_empty()**] - Returns `true` is the bitmap contains a valid bitmap with width > 1 and height > 1.  
	///
	/// - `false` is returned if the bitmap is invalid or contains no data. 
	///
	pub fn is_empty(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_bitmap_is_valid(self.id,true,true); }
		ret_val
	}

	/// [**is_valid()**] - Returns `true` is the bitmap contains a valid bitmap with width > 1 and height > 1.  
	///
	/// - `false` is returned if the bitmap is invalid or contains no data. 
	///
	pub fn is_valid(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_bitmap_is_valid(self.id,false,true); }
		ret_val
	}

	/// [**size()**] - Returns the width and height of the bitmap.
	///
	pub fn size(&self) -> (i32,i32)
	{
		unsafe { let ret = ext_func::sage_rust_bitmap_size(self.id,true);
		( ret.x, ret.y )
}
	}

	/// [**width()**] - Returns the width of the bitmap
	///
	/// - See <i>**size()**</i> to return both <i>width</i> and <i>height</i> at the same time.
	///
	pub fn width(&self) -> i32 { self.size().0 }

	/// [**height()**] - Returns the height of the bitmap
	///
	/// - See <i>**size()**</i> to return both <i>width</i> and <i>height</i> at the same time.
	///
	pub fn height(&self) -> i32 { self.size().1 }
    fn create(_id : i64 ) -> Bitmap32 { Bitmap32 { id : _id } }

}
impl Drop for Bitmap32 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_delete_bitmap(self.id,true); }
    }
}

pub struct Bitmap
{
	id : i64,
}

pub struct BitmapArray
{
	pub bitmap : Vec<u8>,
	pub size   : (i32,i32),
}
pub trait BitmapColorType<P,T>
{
	/// [**set_pixel()**] — Sets an RGB Pixel in the bitmap.
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_pixel(location : Point<i32>,RgbColor)
	///			set_pixel(location : Point<i32>,(i32,i32,i32))
	///			set_pixel(location : (i32,i32),RgbColor)
	///			set_pixel(location : (i32,i32),(i32,i32,i32))
	///             
	/// - If the location value is out-of-bounds, the function returns without setting a pixel
	///
	/// **Examples** 
	///
	/// 	my_window.set_pixel((100,200),(255,0,0))   // Set a red pixel at point (100,200)
	/// 	my_window.set_pixel((100,200),MyPixel)     // Set pixel at (100,200) with Mypixel that can be (i32,i32,i32) or an RgbColor
	///
	fn set_pixel(&self,location : P,color : T);
}
impl Bitmap 
{
    pub fn new() -> Bitmap
	{
		Bitmap { id: 0 }
	}
	pub fn copy(&self) -> Bitmap
	{
		Bitmap {id: self.id  }
	}
	pub const fn default() -> Bitmap
	{
		Bitmap { id: 0 }
	}
	pub fn get_id(&self) -> i64 { self.id }


	/// [**get_pixel()**] — Returns the RGB color of the point location in the bitmap
	///             
	/// - If the location value is out-of-bounds, the function returns a color of BLACK (i.e. (0,0,0))
	///
	/// **Example** 
	///
	/// 	let my_pixel = my_window.get_pixel((100,200))   // Retrieve the pixel value from the point (100,200) inside of the bitmap
	///
	pub fn get_pixel(&self,x : i32,y : i32) -> RgbColor
	{
		unsafe { let rgb_color = ext_func::sage_rust_bitmap_get_pixel(self.id,x,y);
		RgbColor{ red : rgb_color.red, green : rgb_color.green, blue : rgb_color.blue }
		}
	}

	fn _set_pixel_rgb(&self,location : &(i32,i32),color : &RgbColor)
	{
		unsafe { ext_func::sage_rust_bitmap_set_pixel(self.id,location.0,location.1,color.red,color.green,color.blue); }
	}
	fn _set_pixel_i32(&self,location : &(i32,i32),color : &(i32,i32,i32))
	{
		unsafe { ext_func::sage_rust_bitmap_set_pixel(self.id,location.0,location.1,color.0,color.1,color.2); }
	}

	fn _set_pixel_rgb_point(&self,location : &Point<i32>,color : &RgbColor)
	{
		unsafe { ext_func::sage_rust_bitmap_set_pixel(self.id,location.x,location.y,color.red,color.green,color.blue); }
	}
	fn _set_pixel_i32_point(&self,location : &Point<i32>,color : &(i32,i32,i32))
	{
		unsafe { ext_func::sage_rust_bitmap_set_pixel(self.id,location.x,location.y,color.0,color.1,color.2); }
	}
	/// [**is_empty()**] - Returns `true` is the bitmap contains a valid bitmap with width > 1 and height > 1.  
	///
	/// - `false` is returned if the bitmap is invalid or contains no data. 
	///
	pub fn is_empty(&self) -> bool
	{
		if self.id == 0 { return true; }
		unsafe { ext_func::sage_rust_bitmap_is_valid(self.id,true,false) }
	}

	/// [**is_valid()**] - Returns `true` is the bitmap contains a valid bitmap with width > 1 and height > 1.  
	///
	/// - `false` is returned if the bitmap is invalid or contains no data. 
	///
	pub fn is_valid(&self) -> bool
	{
		if self.id == 0 { return false; }
		unsafe {ext_func::sage_rust_bitmap_is_valid(self.id,false,false) }
	}

	/// [**size()**] - Returns the width and height of the bitmap.
	///
	pub fn size(&self) -> (i32,i32)
	{
		unsafe { let ret = ext_func::sage_rust_bitmap_size(self.id,false);
		( ret.x, ret.y ) }
	}

	/// [**width()**] - Returns the width of the bitmap
	///
	/// - See <i>**size()**</i> to return both <i>width</i> and <i>height</i> at the same time.
	///
	pub fn width(&self) -> i32 { self.size().0 }

	/// [**height()**] - Returns the height of the bitmap
	///
	/// - See <i>**size()**</i> to return both <i>width</i> and <i>height</i> at the same time.
	///
	pub fn height(&self) -> i32 { self.size().1 }

	/// [**widthbytes()** and **stride()**] — Returns number of bytes for each row in the bitmap
	///
	/// For 8-bit RGB bitmaps, the format used in the Bitmap is that each row must be divisible by 4.  
	/// - This is a standard in Bitmaps.
	/// - Some RGB bitmaps have a width of pixels not divisible by 4, and padding is added in these bitmaps.
	///   - For example, a bitmap width of 75 is 225 (75*3) bytes in length for each row (due r,g,b values)
	///    - - In this case, 1 bytes is added to make the row divisible by 4
	/// - Because of this, it can be important to understand the width, or <i>**stride**</i> of a bitmap row when calculating memory addresses or indexes.
	///
	/// For example: 
	///
	///		let total_size = my_bitmap.stride()*my_bitmap.height()  // Gives the total size of bitmap memory
	///
	/// Where using <i>**~~let total_size = my_bitmap.width()*my_bitmap.height()~~**</i> may give the wrong value.
	///
	///	- note: **stride()** and **widthbytes()** are the same function
	///   - the term **<i>'stride'</i>** is the technical term.  The function **widthbytes()** is included for compatibility between Sagebox versions
	///
	pub fn widthbytes(&self) -> i32 { unsafe { ext_func::sage_rust_bitmap_widthbytes(self.id,false) as i32 } }

	/// [**widthbytes()** and **stride()**] — Returns number of bytes for each row in the bitmap
	///
	/// For 8-bit RGB bitmaps, the format used in the Bitmap is that each row must be divisible by 4.  
	/// - This is a standard in Bitmaps.
	/// - Some RGB bitmaps have a width of pixels not divisible by 4, and padding is added in these bitmaps.
	///   - For example, a bitmap width of 75 is 225 (75*3) bytes in length for each row (due r,g,b values)
	///    - - In this case, 1 bytes is added to make the row divisible by 4
	/// - Because of this, it can be important to understand the width, or <i>**stride**</i> of a bitmap row when calculating memory addresses or indexes.
	///
	/// For example: 
	///
	///		let total_size = my_bitmap.stride()*my_bitmap.height()  // Gives the total size of bitmap memory
	///
	/// Where using <i>**~~let total_size = my_bitmap.width()*my_bitmap.height()~~**</i> may give the wrong value.
	///
	///	- note: **stride()** and **widthbytes()** are the same function
	///   - the term **<i>'stride'</i>** is the technical term.  The function **widthbytes()** is included for compatibility between Sagebox versions
	///
	pub fn stride(&self) -> usize { unsafe { ext_func::sage_rust_bitmap_widthbytes(self.id,false) } }

	/// [**memory_size()**] — Returns number of bytes used in the bitmap memory
	///
	/// - This calculattion can be formed with <i>**my_bitmap.stride()*my_bitmap.height()**</i>
	/// - See notes on <i>**stride()**</i> and <i>**widthbytes()**</i> for more information
	///
	pub fn memory_size(&self) -> usize { self.stride()*self.height() as usize }
    fn create(_id : i64 ) -> Bitmap { Bitmap { id : _id } }

	pub fn get_vector<'a>(&self) -> Vec<u8>
	{
		if self.width() <= 0 || self.height() <= 0 { return vec![]; }

		unsafe 
		{
			let vector = ext_func::sage_rust_bitmap_get_vector(self.id);
			if vector.len > 0 { return Vec::from_raw_parts(vector.data, vector.len, vector.len) }

			vec![]
		}
	}

	/// [**from_vec()**] — Returns a Bitmap from the input vector
	///
	/// This function returns a Bitmap from an input vector.
	///
	/// - The Bitmap returned is a copy of the vector
	/// - The vector is still valid and retains the same memory values. 
	///
	/// **Note:**
	///
	/// - This function will be expanded to accept a large range of vectors.
	/// - Currently, it accepts unaligned color &lt;u8> or monochrome &lt;u8> vectors
	///   - Any other vector type returns an empty Bitmap
	/// - Functions such as display_bitmap() or display_bitmap_vec() will also take vectors as input, in the next update or so.
	///
	pub fn from_vec(size : (i32,i32), bitmap_vec : &Vec<u8>) -> Bitmap
	{
		Bitmap { id: unsafe { ext_func::sage_rust_bitmap_from_vector(size.0,size.1,bitmap_vec.as_ptr(),bitmap_vec.len()) } }
	}

	/// [**get_array()**] — Converts a Bitmap to a Bitmap Array that includes a vector rather than a bitmap.
	///
	/// This function returns a **<i>BitmapArray</i>**, which contains the following:
	///
	/// - A Vec&lt;u8> vector containing the bitmap in unaligned form. 
	/// - a size value that specifies the width and height of the returned bitmap. 
	///
	/// This allows the bitmap data do be manipulated directly. 
	///
	/// Notes: 
	///
	/// - The original bitmap is still active and not altered. 
	/// - Many bitmap functions accept vectors directly. 
	///
	pub fn get_array<'a>(&self) -> BitmapArray
	{
		if self.width() <= 0 || self.height() <= 0 { return BitmapArray{ bitmap : vec![], size : (0,0) }; }

		unsafe 
		{
			let vector = ext_func::sage_rust_bitmap_get_vector(self.id);
			if vector.len > 0 { return BitmapArray { bitmap : Vec::from_raw_parts(vector.data, vector.len, vector.len), size : (self.width(), self.height()) }; }

			BitmapArray{ bitmap : vec![], size : (0,0) }
		}
	}


}

impl Drop for Bitmap 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::sage_rust_delete_bitmap(self.id,false); }
    }
}
impl BitmapColorType<(i32,i32),RgbColor> for Bitmap
{
	fn set_pixel(&self,location : (i32,i32),color : RgbColor) { self._set_pixel_rgb(&location,&color); }
}
impl BitmapColorType<(i32,i32),(i32,i32,i32)> for Bitmap
{
	fn set_pixel(&self,location : (i32,i32),color : (i32,i32,i32)) { self._set_pixel_i32(&location,&color); }
}
impl BitmapColorType<Point<i32>,RgbColor> for Bitmap
{
	fn set_pixel(&self,location : Point<i32>,color : RgbColor) { self._set_pixel_rgb_point(&location,&color); }
}
impl BitmapColorType<Point<i32>,(i32,i32,i32)> for Bitmap
{
	fn set_pixel(&self,location : Point<i32>,color : (i32,i32,i32)) { self._set_pixel_i32_point(&location,&color); }
}
//#[derive(Clone)]
pub struct Window
{
	id : i64,
	pub gdi : Gdi,
}
pub trait WindowFontInputType<FontInputType>	
{

	/// [**set_font()**] — Sets the font for the text output of the window when using the write() function
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_font(font_type : i32)      // Sets a font of size font_type in the default font (Arial)	
	///			set_font(font_type : &str)     // Sets a font with a String slice
	///			set_font(font_type : &String)  // Sets a font with a String reference
	///			set_font(font_type : Font)     // Sets a font with a font previously created 	
	///                                        // **note** - This function is not yet in the interface
	///			
	/// **Parameters**
	///
	/// - font_type — Sets the font type via an integer or a string.
	///
	/// **About Font Strings**
	///
	/// - Numbers in strings — a font string can be as simple as a number, which sets that point value in the default arial font.
	/// - Emphasis — Emphasis such as "bold" and "italic" can also be used.
	/// - Font Names — Font names can be used, such as "Times New Roman", "Courier new", etc. 
	///   - Terminal and non-true-type fonts are recognized. 
	///   - - e.g. using Courier New will set it to a terminal font which will have the same spacing for each character. 
	///
	/// **About Font Storage**
	///
	/// - The first time a font is used, the font and it's glyphs (character data) are created in memory.
	/// - Once a font is created, it is remembered.
	///   - Using fonts such as **<i>"Times New Roman,12,bold"</i>** more than once does not create a new font, but uses the previous instance.
	/// - Font can be assigned to Font values via the create_font() functions. 
	///   - with names fonts, e.g. **<i>win.create_font_s("arial,12,bold",kw::name("MyFont"))</i>**, these fonts can be used in embedded text
	///   - - e.g. **<i>"This is written in {MyFont}My personal Font{}."</i>**
	///
	/// **Notes**
	///
	/// - If a font is not found, the Sagebox default font (Arial, 12) is returned. 
	/// - See **`create_font()`** to create a font that can be re-used easily
	///   - using a created font can be faster, since Sagebox does need to parse the text and lookup fonts. 
	///   - - It only needs to match the name and find the font, or the font can be set with **`set_font()`** to set it globally for the window.
	///
	/// **Examples**
	///
	/// 		win.set_font(15);                  // Set a font size of 15 in an Arial Font
	///         win.set_font("15");                // Also sets a font size of 15 in an Arial font
	///			win.set_font("27,bold.italic")     // Sets an Arial font with point size 27 that is bold and italic.
	/// 		win.set_font("courier new,15")     // Set's a terminal font of Courier New with a point size of 15
	///			win.set_font("Times New Roman,12") // Sets a Times New Roman Font with a size of 12 points.
	fn set_font(&self,font_type : FontInputType);	

}

// For functions that only need the difference betweem i32 and Point<i32>
pub trait SageWinPairTypei32<T>
{
	/// [**get_window_bitmap()**] - Returns a bitmap with a copy of the contents of the window. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			get_window_bitmap()
	///			get_window_bitmap_s(upper_left : (i32,i32), size : (i32,i32))
	///			get_window_bitmap_s(upper_left : Point<i32>, size : Point<i32>)
	///                                     
	/// **Parameters:**
    /// 
    /// - **`upper_left`**  - Upper-left coordinate of the window where the bitmap should start.
    /// - **`size`** - size of the bitmap top copy. 
	///
	/// **Notes:**
	///
	/// - using <i>**get_window_bitmap()**</i> (i.e. with no parameters()) copies the entire window area.
	///
	///  **Example:**
	///
	///			win.get_window_bitmap(); 		// Copy the entire contents of the window
	///			win.get_window_bitmap((100,200),(400,400)); // Copy a 400x400 bitmap starting at location (100,200)
	///
	fn get_window_bitmap_s(&self,upper_right : T, size : T) -> Bitmap;

	/// [**set_write_pos()**] — Sets the next write position in the window when using the **`write()`** function.
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_write_pos(write_pos : (i32,i32))
	///			set_write_pos(write_pos : Point<i32>)
	///                                     
	/// - The write position can be set anywhere in the window.
	/// - When the next newline (**'\n'**) is seen, the write position returns to the 
	///   leftmost position in the next row, unless a **`write indent`** has been used. 
	///   - see **`set_write_indent()`** for more information
	///
	fn set_write_pos(&self,write_pos : T);
}

impl Window 
{
    pub fn new( _id : i64) -> Window
	{
		Window { id: _id, gdi : Gdi::new(_id) }
	}

//	pub fn copy(&self) -> Window
//	{
//		Window {id: self.id, gdi : Gdi::new(self.id) }
//	}
	pub const fn default() -> Window
	{
		Window { id: 0, gdi : Gdi::default() }
	}
	pub fn get_id(&self) -> i64 { self.id }
	pub fn auto(&self) -> Window
	{	
		unsafe { ext_func::rust_window_set_auto_autoupdate(self.id,true); }
		Window{id: self.id, gdi : Gdi::default() }
	}



	///<summary>
	///Write Text out to the window. 
	///<br />
	///<br />
	///The write() function will write text to the window and can be used with many options.
	///<br />
	///<br />
	/// Forms: 
	///<br />
	///<br />
	/// <i>- write(&str);</i><br />
	/// <i>- write_s(&str,keyword : kw);</i><br />
	/// <i>- write_str(&string);</i><br />
	/// <i>- write_str_s(&string,keywords : kw)</i><br />
	///<br />
	///Basic Examples:<br /><br />
	///<b>window.write("Hello World");</b><br />
	/// <b>window.write(&format!(<em>"My Variable is: {MyVariable}").as_str())</em></b>);<br />
	/// <b>window.write_str(format!(<em>"My Variable is: {MyVariable}"))</em></b>);
	///<br />
	///<br />
	///Sagebox keywords (e.g. <em><b>kw::font(100)</b></em> options can be included.  Some various options are as follows:
	///<br />
	///<br />
	/// <list type="bullet">
	///<item><param name="Font"><b>Font</b> - Set the font to be used for the text</param></item><br />
	///<item><param name="Center,CenterX,CenterY"><b>Center,CenterX,CenterY</b> - Center the text in various ways 
	/// (i.e. CenterX centers in the X-axis, etc.)</param></item><br />
	///<item><param name="TextColor,fgColor"><b>fgColor or TextColor</b> - Set the text color for the text.</param></item><br />
	///<item><param name="bgColor"><b>bgColor</b> - Set the background color for the text</param></item><br />
	///<item><param name="Angle"><b>Angle</b> - Set the rotational angle of the text</param></item><br />
	///<item><param name="Opacity"><b>Opacity</b> - Set the opacity of the text</param></item><br />
	///<item><param name="Pos"><b>Pos</b> - Set the position in the window of the text, 
	///e.g. Write("Hello World",kw::pos(x,y))</param></item><br />
	///</list>
	///<br />
	///<br />
	///Controls can be embedded in the text line to change colors and font sizes:
	///<br />
	///<br />
	///For example:
	///<list type="bullet">
	///<item><param name="Font"><b>window.write("This <em>{r}word{}</em> is in the color red").</b>
	/// <br />   
	///    - Note the {} to close.  
	/// <br />   
	///    - With Rust formatted strings, an extra "{}" is needed, such as <em>"MyValue {{r}}{myvalue}{{}} is in red".</em>
	/// </param></item>
	///</list>
	///<br />
	///<br />
	///More than one control can be used such as: <b>win.write("This is in <em>{r}Red{}</em> and this is in 
	///<em>{b}Blue</em>)".</b> 
	///<br />
	///You do not need the closing {} if it is as the end of the line.
	///<br />
	///<br />
	///<b>Some Curly-Brace Embedded Controls</b><br /><br />
	///<list type="bullet">
	///<item><param name="Color"><b>{&lt;color&gt;}</b> - Where the color is a defined color such as {red} (or {r}), 
	///{blue}, {skyblue}, etc. e.g. <em>"this {blue}word{} is in blue"</em> ({} closes the color or font)
	///<br />
	///- You can use abbreviations for most primary colors, such as <em>{y} = {yellow}</em>, etc.
	///</param></item><br />                       
	///<item><param name="FontSize"><b>{font size}</b> - e.g. <em>"This is in the normal font, and 
	///                      {30}this is in a 30-point font"</em></param></item><br />
	///<item><param name="FontName"><b>{font name}</b> - e.g. <em>"This is in the normal font, and 
	///                             {Courier New,20}This is in a 20-point Courier New font"</em></param></item><br />
	///<item><param name="SetXPos"><b>{x = &lt;value&gt;}</b> - Sets the X position on the current line,
	/// e.g. <em>"This value{x=100}starts at position 100 pixels from the left."</em>
	/// <br />- This is useful in aligning data values with titling to the left, since typical fonts have varied sizes for each printed letter/numeral vs. static sizes.</param></item><br />
	///</list>
	///<br />
	///<br />
	///note: <em>when using <b>kw::angle()</b> and <b>kw::opacity()</b>, embedded options no longer work, 
	///such as <b>{font}</b>, <b>{color}</b> etc.
	///<br />
	/// - e.g. rather than "{50}This is a big font", when using Angle() and/or Opacity() options, use write("This is a big font",kw.Font(50)) instead.</em>
	///<br />
	///<br />
	///<b>Examples:</b>
	///<br />
	///<br />
	///<list type="bullet">
	///<item><param name="Example1"><b>MyWindow.write_s("Hello World",kw::font(40) + kw::center())</b>
	///<- Writes a big "Hello World" in the center of the screen<br /></param></item>
	/// <item><param name="Example1"><b>MyWindow.write_s("Hello World",kw::fg_color("red")</b>
	///- Writes "Hello World" in red<br /></param></item>
	///     
	///<item><param name="Example1"><b>MyWindow.write("{r}Hello World")</b>
	///- Also writes "Hello World" in red<br /></param></item>
	///     
	///<item><param name="Example1"><b>MyWindow.write_s("Hello World",kw::font(50))</b>
	///- Writes "Hello World" in a 50-point font size.<br /></param></item>
	///     
	///<item><param name="Example1"><b>MyWindow.write("{50}Hello World")</b>
	///<br />- Also writes "Hello World" in a 50-point font size.<br /></param></item>
	///</list>
	/// </summary>
	fn _write_s(&self,msg : &str,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_write(msg.as_ptr(),msg.len(),self.id, keyw.pointer); }
		ret_val
	}	


	/// [**set_write_indent()**] — Sets the leftmost column return point for newlines when using the write() function
	///
	/// - When using the **`write()`** function and a newline (**'\n'**) is encountered, the next write position will be set on the next line (based
	/// on the largest font seen in the current line) in the leftmost part of the window (at pixel 0). 
	/// - If a **`write indent`** is set using **<i>`set_write_indent()`</i>**, this will put the next write position at this pixel position on the next line for each 
	///   subsequent newline (**'\n'**) until the **`write indent`** is reset back to 0 or another value.
	///	
	/// **Notes**
	///
	/// - use **<i>`set_write_indent(0)`</i>** to remove the write indent and set the indent back to the leftmost position in the window.
	///
	/// **Example**
	///
	///			win.set_write_indent(100); 	// Causes all new lines to start at horizontal pixel 100 on new lines until reset or changed.
	///
	pub fn set_write_indent(&self,indent : i32) 
	{
		unsafe { ext_func::sage_rust_window_generic_integer(self.id,0,indent) }
	}
	/// [**set_write_padding()**] — Adds vertical spacing for each newline when using the write() function
	/// 
	/// - By default, when using the **`write()`** function and a newline (**'\n'**) is encountered, the next vertical place in the
	///   window is set by adding the height of the largest font used in the current line, starting the next vertical text just underneath the previous line of text.
	/// - To provide spacing, **<i>`set_write_padding()`</i>** can be used to add vertical spacing between lines.
	/// - Negative values can be used to shrink spacing, also. 
	///
	/// **Notes**
	///
	/// - use **<i>`set_write_padding(0)`</i>** to remove the write padding and return it to it's default 0 value.
	///
	/// **Example**
	///
	///			win.set_write_padding(10); 	// Add 10 pixels to each line of writing to provide a vertical spacing effect.
	///
	pub fn set_write_padding(&self,padding : i32) 
	{
		unsafe { ext_func::sage_rust_window_generic_integer(self.id,1,padding) }
	}
	fn _set_write_pos(&self,write_pos : (i32,i32))
	{
		unsafe { ext_func::sage_rust_window_set_write_pos(self.id,write_pos.0,write_pos.1) }
	}
	fn _write(&self,msg : &str) -> bool 	{ self._write_s(msg,KW_NONE) }

	fn _write_xy_s(&self,location : (i32,i32),msg : &str,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_write_xy(msg.as_ptr(),msg.len(),location.0,location.1,self.id, keyw.pointer); }
		ret_val
	}	

	fn _write_xy(&self,location : (i32,i32),msg : &str) -> bool 	{ self._write_xy_s(location,msg,KW_NONE) }


	fn _writeln(&self,msg : &str) -> bool
	{
		// Placeholder until writeln() is placed into DLL

		self._write_s(msg,KW_NONE);
		self._write("\n")
	}	

	fn _writeln_s(&self,msg : &str,keyw : kw) -> bool
	{
		// Placeholder until writeln() is placed into DLL
		self._write_s(msg,keyw);
		self._write("\n")
	}	



	fn _write_str(&self,msg : &String) -> bool
	{
		self._write(msg.as_str())
	}	

	fn _write_str_s(&self,msg : &String,keyw : kw) -> bool
	{
		self._write_s(msg.as_str(),keyw)
	}	

	fn _write_xy_str(&self,location : (i32,i32),msg : &String) -> bool
	{
		self._write_xy(location,msg.as_str())
	}	

	fn _write_xy_str_s(&self,location : (i32,i32),msg : &String,keyw : kw) -> bool
	{
		self._write_xy_s(location,msg.as_str(),keyw)
	}

	fn _writeln_str(&self,msg : &String) -> bool
	{
		// Placeholder until writeln() is placed into DLL

		self._write(msg.as_str());
		self._write("\n")
	}	

	fn _writeln_str_s(&self,msg : &String,keyw : kw) -> bool
	{
		// Placeholder until writeln() is placed into DLL
		self._write_s(msg.as_str(),keyw);
		self._write("\n")
	}	

    /// [**cls**()] - Clears the window to a blank canvas, based on previous color or new color when using cls_s() or cls_grad()
	///
	/// Function Parameter Forms: 
	/// 
	///			cls()                     // Clear the window to the last cls() color
	///			cls_s(color : RgbColor)   // Clear with an RgbColor, variable, or preset such as pan_color::ForestGreen or sage_color::Blue
	///			cls_s(color : &str)       // Clear with a string slice, e.g. cls("green")
	///			cls_s(color : &String)    // Clear with a string e.g. cls(my_string)
	///
	/// **Notes**
	///
	/// - using <i>**cls()**</i> with no parameters clears the window to the last known <i>cls() color</i>, which can be a solid color (e.g. black or blue), 
	/// a bitmap, a vertical gradient, a radial gradient, depending on the last cls() that used any parameters. 
	/// - using <i>**cls_s()**</i> with parameter (e.g. <i>**cls_s("green")**</i>) sets the new color for <i>cls()</i> when used with no parameters.
	/// - A vertical gradient can be used when using &str or &String, e.g. <i>**cls("black,blue")**</i>
	/// - See <i>**cls_grad()**</i> to clear the window with a vertical gradient , e.g. <i>**cls_grad_s(pan_color::black,pan_color::blue)**</i>, 
	///when using <i>RgbColor</i> or <i>(i32,i32,i32)</i> Rgb values
	/// - See <i>**cls_radial()**</i> to clear the window with a radial gradient eminating from the center of the window, e.g. <i>**cls_radial("black,darkblue")**</i>
	///
	/// **Examples:**
	///
	///	win.cls()                     // Clear the window with the current background color of the window
	///	win.cls_s("red")              // Clear the window with the color red
	///	win.cls_s(sage_color::Red)    // Clears the color with red, also
	///	win.cls_s(pan_color::beige)   // Clears the window with PanColor.ForestGreen
	///	win.cls_s(my_color)           // Clears the window with a defined my_color, which may be an RgbColor, &str, &String, or (i32,i32,i32)
	///	win.cls_s((0,255,255)         // Clears the window with a defined "MyColor", which may be an RgbColor, &str, &String, or (i32,i32,i32)
	///	win.cls_s("black,blue")       // Clears the window with a gradient from black to blue
	///
	pub fn cls(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_cls_rgb(self.id,-1,-1,-1,-1,-1,-1,false); }
		ret_val
	}

	fn _cls_s(&self,color : RgbColor) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_cls_rgb(self.id,color.red,color.green,color.blue,-1,-1,-1,false); }
		ret_val
	}

	fn _cls_grad(&self,color : RgbColor,color2 : RgbColor) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_cls_rgb(self.id,color.red,color.green,color.blue,color2.red,color2.green,color2.blue,false); }
		ret_val
	}

	fn _cls_str(&self,color : &str) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_cls_str(self.id,color.as_ptr(),color.len(),false); }
		ret_val
	}

	/// [**cls_radial**()] - Clears the window to a blank canvas with a radial gradient (first color in the center of the window, to the second color at edges of the window)
	///
	/// Function Parameter Forms: 
	///                    					
	///			cls_radial(color1 : RgbColor,color1 : RgbColor)   			
	///			cls_radial(color1 : (i32,i32,i32),color2 : (i32,i32.i32))  
	///			cls_radial_str(color_string : &str)  
	///
	/// **Notes**
	///
	/// - After using <i>**cls_radial()**</i> to create a vertical gradient, the next <i>cls()</i> with no parameters will clear the image to the same gradient.
	/// - See <i>**cls()**</i> to clear the window to a single color
	/// - See <i>**cls_grad()**</i> to clear the window with a vertical gradient instead of a radial gradient.
	///
	/// **Examples:**
	///
	///		win.cls_radial(pan_color::blue,pan_color::black)      // Clear the window with radial gradient from blue (center) to black (edges)
	///		win.cls_radial_str("blue,black")                      // Same as the above, using string slice with two colors embedded
	///		win.cls()                                             // Clear the window with the current background color or gradient of the window
	///
	pub fn cls_radial_str(&self,color : &str) -> bool 
	{ 
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_cls_str(self.id,color.as_ptr(),color.len(),true); } 
		ret_val
	}
	fn _cls_radial(&self,color : RgbColor,color2 : RgbColor) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_cls_rgb(self.id,color.red,color.green,color.blue,color2.red,color2.green,color2.blue,true); }
		ret_val
	}

	fn _set_fg_color(&self,color : RgbColor)
	{
		unsafe { ext_func::sage_rust_window_set_win_color_rgb(self.id,color.red,color.green,color.blue,false); }
	}

	fn _set_bg_color(&self,color : RgbColor)
	{
		unsafe { ext_func::sage_rust_window_set_win_color_rgb(self.id,color.red,color.green,color.blue,true); }
	}

	fn _set_fg_color_str(&self,color : &str)
	{
		unsafe { ext_func::sage_rust_window_set_win_color_str(self.id,color.as_ptr(),color.len(),false); }
	}

	fn _set_bg_color_str(&self,color : &str)
	{
		unsafe { ext_func::sage_rust_window_set_win_color_str(self.id,color.as_ptr(),color.len(),true); }
	}

	/// Returns the center coordinates of the window in a Point<i32> format
	/// <br />
	/// - Point<i32> allows overladed operator use, such as a+b, a/b, etc.<br />
	/// - Point<i32> can be converted to (i32,i32) with function and Point<i32>::i32<br /><br />
	pub fn get_window_center(&self) -> Point<i32>
	{
		let mut _x : i32 = 0;
		let mut _y : i32 = 0;
		unsafe { 
			let v = ext_func::sage_rust_window_get_size(self.id); 
			_x = (v >> 32) as i32;
			_y = (v & 0xFFFFFFFF) as i32;
		}
		Point::<i32>::new(_x,_y)
	}

	/// Returns the center coordinates of the window in a Point<f32> format
	/// <br />
	/// - Point<f32> allows overladed operator use, such as a+b, a/b, etc.<br />
	/// - Point<f32> can be converted to (f32,f32) or (i32,i32) with function Point<f32>::f32() and Point<f32>::i32<br /><br />
	pub fn get_window_center_f(&self) -> Point<f32>
	{
		let mut _x : i32 = 0;
		let mut _y : i32 = 0;
		unsafe { 
			let v = ext_func::sage_rust_window_get_center(self.id); 
			_x = (v >> 32) as i32;
			_y = (v & 0xFFFFFFFF) as i32;
		}
		Point::<f32>::new(_x as f32,_y as f32)
	}	

	/// Returns the current window's client area size Point<i32> format
	/// <br />
	/// - Point<i32> allows overladed operator use, such as a+b, a/b, etc.<br />
	/// - Point<i32> can be converted to (i32,i32) with function and Point<i32>::i32<br /><br />
	pub fn get_window_size(&self) -> Point<i32>
	{
		let mut _x : i32 = 0;
		let mut _y : i32 = 0;
		unsafe { 
			let v = ext_func::sage_rust_window_get_size(self.id); 
			_x = (v >> 32) as i32;
			_y = (v & 0xFFFFFFFF) as i32;
		}
		Point::<i32>::new(_x,_y)
	}

	/// Returns the current window's client area size Point<f32> format
	/// <br />
	/// - Point<f32> allows overladed operator use, such as a+b, a/b, etc.<br />
	/// - Point<f32> can be converted to (f32,f32) or (i32,i32) with function Point<f32>::f32() and Point<f32>::i32<br /><br />
	pub fn get_window_size_f32(&self) -> Point<f32>
	{
		let mut _x : i32 = 0;
		let mut _y : i32 = 0;
		unsafe { 
			let v = ext_func::sage_rust_window_get_size(self.id); 
			_x = (v >> 32) as i32;
			_y = (v & 0xFFFFFFFF) as i32;
		}
		Point::<f32>::new(_x as f32,_y as f32)
	}		
	
	/// Returns the current window's client area size Point<f32> format
	/// <br />
	/// - Point<f64> allows overladed operator use, such as a+b, a/b, etc.<br />
	///
	pub fn get_window_size_f64(&self) -> Point<f64>
	{
		let mut _x : i32 = 0;
		let mut _y : i32 = 0;
		unsafe { 
			let v = ext_func::sage_rust_window_get_size(self.id); 
			_x = (v >> 32) as i32;
			_y = (v & 0xFFFFFFFF) as i32;
		}
		Point::<f64>::new(_x as f64,_y as f64)
	}		

	fn _draw_pixel_rgbi(&self,x : i32, y : i32, color : (i32,i32,i32)  ) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_set_pixel_rgb(self.id,x,y,color.0,color.1,color.2); }
		ret_val
	}

	fn _draw_pixel_rgb(&self,x : i32, y : i32, color : RgbColor) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_set_pixel_rgb(self.id,x,y,color.red,color.green,color.blue); }
		ret_val
	}

 	/// Returns True if the Window was closed (popup-window mode only). 
    ///    
    /// This can be used in an event-loop to determine if the window was closed by the user.  
	/// When window_closed() returns <i>true</i>, this means the window was closed
	///without pressing any of the buttons, representing a cancel button press at the same time.
    ///    
    /// Notes:
    /// - <i>window_closing()</i> and <i>window_closed()</i> are the same function.  They are duplicated for
	/// compatibility between platforms.
	///
    /// Useful Keywords:
    ///    
    /// - peek -- When true (e.g. kw::peek()), the window-closed status is not reset and will always read 'true'
	//(when a change has occurred) until a call occurs without the 'peek' keyword set to true
    ///    
	/// Example:
	///
	///		if my_window.window_closed() { println!("Window was closed by the user."); }
	///
	pub fn window_closing(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_window_closing(self.id) }
	}

 	/// Returns True if the Window was closed (popup-window mode only). 
    ///    
    /// This can be used in an event-loop to determine if the window was closed by the user.  
	/// When window_closed() returns <i>true</i>, this means the window was closed
	///without pressing any of the buttons, representing a cancel button press at the same time.
    ///    
    /// Notes:
    /// - <i>window_closing()</i> and <i>window_closed()</i> are the same function.  They are duplicated for
	/// compatibility between platforms.
	///
    /// Useful Keywords:
    ///    
    /// - peek -- When true (e.g. kw::peek()), the window-closed status is not reset and will always read 'true'
	//(when a change has occurred) until a call occurs without the 'peek' keyword set to true
    ///    
	/// Example:
	///
	///		if my_window.window_closed() { println!("Window was closed by the user."); }
	///
	pub fn window_closed(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_window_closing(self.id) }
	}

	// -----------
	// Draw Circle
	// -----------

	fn _draw_circle_s(&self,x : f32, y: f32, radius : f32,color : RgbColor,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_rgb(0,self.id,x,y,radius,radius,
					color.red,color.green,color.blue,-1,keyw.pointer); }
		ret_val
	}

	fn _draw_circle_a_s(&self,x : f32, y: f32, radius : f32,color : RgbColorA,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_rgb(0,self.id,x,y,radius,radius,
					color.red,color.green,color.blue,color.opacity,keyw.pointer); }
		ret_val
	}
	fn _draw_circle(&self,x : f32, y: f32, radius : f32,color : RgbColor) -> bool
	{
		self._draw_circle_s(x,y,radius,color,KW_NONE)
	}	


	fn _draw_circle_a(&self,x : f32, y: f32, radius : f32,color : RgbColorA) -> bool
	{
		self._draw_circle_a_s(x,y,radius,color,KW_NONE)
	}

	fn _draw_circle_str_s(&self,x : f32, y: f32, radius : f32,color : &str,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_str(0,self.id,x,y,radius,radius,
					color.as_ptr(),color.len(),keyw.pointer); }
		ret_val
	}
	fn _draw_circle_str(&self,x : f32, y: f32, radius : f32,color : &str) -> bool
	{
		self._draw_circle_str_s(x,y,radius,color,KW_NONE)
	}	

	// -----------
	// Fill Circle
	// -----------

	fn _fill_circle_s(&self,x : f32, y: f32, radius : f32,color : RgbColor,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_rgb(1,self.id,x,y,radius,radius,
					color.red,color.green,color.blue,-1,keyw.pointer); }
		ret_val
	}


	fn _fill_circle_a_s(&self,x : f32, y: f32, radius : f32,color : RgbColorA,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_rgb(1,self.id,x,y,radius,radius,
					color.red,color.green,color.blue,color.opacity,keyw.pointer); }
		ret_val
	}

	fn _fill_circle(&self,x : f32, y: f32, radius : f32,color : RgbColor) -> bool
	{
		self._fill_circle_s(x,y,radius,color,KW_NONE)
	}	


	fn _fill_circle_a(&self,x : f32, y: f32, radius : f32,color : RgbColorA) -> bool
	{
		self._fill_circle_a_s(x,y,radius,color,KW_NONE)
	}	

	fn _fill_circle_str_s(&self,x : f32, y: f32, radius : f32,color : &str,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_str(1,self.id,x,y,radius,radius,
					color.as_ptr(),color.len(),keyw.pointer); }
		ret_val
	}

	fn _fill_circle_str(&self,x : f32, y: f32, radius : f32,color : &str) -> bool
	{
		self._fill_circle_str_s(x,y,radius,color,KW_NONE)
	}	

	// *** here ***

	// --------------
	// Draw Rectangle
	// --------------

	fn _draw_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColor,keyw : kw) -> bool
	{ unsafe { ext_func::rust_window_draw_generic_rgb(3,self.id,location.0,location.1,size.0,size.1,color.red,color.green,color.blue,-1,keyw.pointer) } }
	
	fn _draw_rectangle_a_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA,keyw : kw) -> bool
	{ unsafe { ext_func::rust_window_draw_generic_rgb(3,self.id,location.0,location.1,size.0,size.1,color.red,color.green,color.blue,color.opacity,keyw.pointer) } }
	
    fn _draw_rectangle(&self,location : (f32,f32), size : (f32,f32),color : RgbColor) -> bool
	{ self._draw_rectangle_s(location,size,color,KW_NONE) }	

	fn _draw_rectangle_a(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA) -> bool
	{ self._draw_rectangle_a_s(location,size,color,KW_NONE) }	


	fn _draw_rectangle_str_s(&self,location : (f32,f32), size : (f32,f32),color : &str,keyw : kw) -> bool
	{ unsafe { ext_func::rust_window_draw_generic_str(3,self.id,location.0,location.1,size.0,size.1,color.as_ptr(),color.len(),keyw.pointer) } }

	fn _draw_rectangle_str(&self,location : (f32,f32), size : (f32,f32),color : &str) -> bool
	{ self._draw_rectangle_str_s(location,size,color,KW_NONE) }	


	// --------------
	// Fill Rectangle
	// --------------

	fn _fill_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColor,keyw : kw) -> bool
	{ unsafe { ext_func::rust_window_draw_generic_rgb(4,self.id,location.0,location.1,size.0,size.1,color.red,color.green,color.blue,-1,keyw.pointer) } }

	fn _fill_rectangle_a_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA,keyw : kw) -> bool
	{ unsafe { ext_func::rust_window_draw_generic_rgb(4,self.id,location.0,location.1,size.0,size.1,color.red,color.green,color.blue,color.opacity,keyw.pointer) } }

	fn _fill_rectangle(&self,location : (f32,f32), size : (f32,f32),color : RgbColor) -> bool
	{ self._fill_rectangle_s(location,size,color,KW_NONE) }	

	fn _fill_rectangle_a(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA) -> bool
	{ self._fill_rectangle_a_s(location,size,color,KW_NONE) }	

	fn _fill_rectangle_str_s(&self,location : (f32,f32), size : (f32,f32),color : &str,keyw : kw) -> bool
	{ unsafe { ext_func::rust_window_draw_generic_str(4,self.id,location.0,location.1,size.0,size.1,color.as_ptr(),color.len(),keyw.pointer) } }

	fn _fill_rectangle_str(&self,location : (f32,f32), size : (f32,f32),color : &str) -> bool
	{ self._fill_rectangle_str_s(location,size,color,KW_NONE) }	

	// ---------
	// Draw Line
	// ---------

	fn _draw_line_s(&self,x : f32, y: f32, x2 : f32, y2: f32,color : RgbColor,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_rgb(2,self.id,x,y,x2,y2,
					color.red,color.green,color.blue,-1,keyw.pointer); }
		ret_val
	}


	fn _draw_line_a_s(&self,x : f32, y: f32, x2 : f32, y2: f32,color : RgbColorA,keyw : kw) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_rgb(2,self.id,x,y,x2,y2,
					color.red,color.green,color.blue,color.opacity,keyw.pointer); }
		ret_val
	}

	fn _draw_line(&self,x : f32, y: f32, x2 : f32, y2: f32,color : RgbColor) -> bool
	{
		self._draw_line_s(x,y,x2,y2,color,KW_NONE)
	}	


	fn _draw_line_a(&self,x : f32, y: f32, x2 : f32, y2: f32,color : RgbColorA) -> bool
	{
		self._draw_line_a_s(x,y,x2,y2,color,KW_NONE)
	}	

	fn _draw_line_str_s(&self,x : f32, y: f32, x2 : f32, y2: f32,color : &str,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_generic_str(2,self.id,x,y,x2,y2,
					color.as_ptr(),color.len(),keyw.pointer); }
		ret_val
	}

	fn _draw_line_str(&self,x : f32, y: f32, x2 : f32, y2: f32,color : &str) -> bool
	{
		self._draw_line_str_s(x,y,x2,y2,color,KW_NONE)
	}	
	// ------------
	// Draw Line To
	// ------------


	fn _draw_line_to_s(&self,x : f32, y: f32 ,color : RgbColor,keyw : kw) -> bool { unsafe { ext_func::rust_window_draw_generic_rgb(5,self.id,x,y,0.0,0.0, color.red,color.green,color.blue,-1,keyw.pointer) } }
	fn _draw_line_to_str_s(&self,x : f32, y: f32 ,color : &str,keyw : kw) -> bool { unsafe { ext_func::rust_window_draw_generic_str(5,self.id,x,y,0.0,0.0, color.as_ptr(),color.len(),keyw.pointer) } }
	fn _draw_line_to_a_s(&self,x : f32, y: f32 ,color : RgbColorA,keyw : kw) -> bool { unsafe { ext_func::rust_window_draw_generic_rgb(5,self.id,x,y,0.0,0.0, color.red,color.green,color.blue,color.opacity,keyw.pointer) } }

	fn _draw_line_to_ex_s(&self,is_first_point : bool, x : f32, y: f32 ,color : RgbColor,keyw : kw) -> bool { unsafe { ext_func::rust_window_draw_generic_rgb(6,self.id,x,y,if is_first_point { 1.0 } else { 0.0 },0.0, color.red,color.green,color.blue,-1,keyw.pointer) } 	}
	fn _draw_line_to_ex_str_s(&self,is_first_point : bool, x : f32, y: f32 ,color : &str,keyw : kw) -> bool { unsafe { ext_func::rust_window_draw_generic_str(6,self.id,x,y,if is_first_point { 1.0 } else { 0.0 },0.0, color.as_ptr(),color.len(),keyw.pointer) } 	}
	fn _draw_line_to_ex_a_s(&self,is_first_point : bool, x : f32, y: f32 ,color : RgbColorA,keyw : kw) -> bool { unsafe { ext_func::rust_window_draw_generic_rgb(6,self.id,x,y,if is_first_point { 1.0 } else { 0.0 },0.0, color.red,color.green,color.blue,color.opacity,keyw.pointer) } }

	// -------------
	// Draw Triangle
	// -------------

	fn _draw_triangle_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_triangle_rgb(self.id,p1.0,p1.1,p2.0,p2.1,p3.0,p3.1,
			color.red,color.green,color.blue,-1,false, keyw.pointer); }
		ret_val
	}
	fn _draw_triangle(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor) -> bool
	{
		self._draw_triangle_s(p1,p2,p3,color,KW_NONE)
	}

	fn _draw_triangle_str_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_triangle_str(self.id,p1.0,p1.1,p2.0,p2.1,p3.0,p3.1,
			color.as_ptr(),color.len(),-1,false, keyw.pointer); }
		ret_val
	}

	fn _draw_triangle_str(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str) -> bool
	{
		self._draw_triangle_str_s(p1,p2,p3,color,KW_NONE)
	}

	// -------------
	// Fill Triangle
	// -------------

	fn _fill_triangle_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_triangle_rgb(self.id,p1.0,p1.1,p2.0,p2.1,p3.0,p3.1,
			color.red,color.green,color.blue,-1,true, keyw.pointer); }
		ret_val
	}

	fn _fill_triangle(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor) -> bool
	{
		self._fill_triangle_s(p1,p2,p3,color,KW_NONE)
	}

	fn _fill_triangle_str_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str,keyw : kw) -> bool
	{
		//fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
		//	blue : i32, alpha : i32);

		let ret_val : bool;
		unsafe { ret_val = ext_func::rust_window_draw_triangle_str(self.id,p1.0,p1.1,p2.0,p2.1,p3.0,p3.1,
			color.as_ptr(),color.len(),-1,true, keyw.pointer); }
		ret_val
	}

	fn _fill_triangle_str(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str) -> bool
	{
		self._fill_triangle_str_s(p1,p2,p3,color,KW_NONE)
	}

	/// [**get_event()**] - GetEvent waits for an event to occur in your main body of code. 
	///
	/// - Events are any events such as a mouse move, click, keyboard press, etc. 
	/// - Events are also caused by any control change such as moving a slider, pressing a button, or pressing return in an input box. 
	/// - Events can also be a window closing, a window moving or changing size -- basically anything happening in the system sends a message
	///
	/// Until an event occurs, the program is sleeping and not using any CPU time.  Sagebox wakes up the program when an event happens. 
	///
	/// In the event loop, you can check for events.  
	///
	/// get_event() returns true until all main windows are closed. 
	///
	/// **Function Parameter Forms:**
	///
	///		get_event()                      // returns true if an event occurs.  Returns false if that event was the user closed the window.
	///		get_event_s(allow_close : bool)	 // when false, does not return false if the user clicked to close the window. 
	///
	/// **Notes**
	///
	/// - When windows are created, the default setting is that the user cannot close the window. 
	///   - When the user clickes the **`X`** button in the upper-right, or presses **`ALT F4`** or otherwise tries to close the window.
	///   - this generates a **`Window Close Button Pressed`** event, which may be detected by calling **<i>win.close_button_pressed()</i>**
	/// - When **<i>get_event()</i>** and **<i>get_event_s(true)</i>** are used, this will detect this event and return false to allow an easy exit to the event loop
	/// - when **<i>get_event_s(false)</i>** is used, the function will not return false if a **`Window Close Button Pressed`** event is detected.
	///   - In this case, call **<i>win.close_button_pressed()</i>** within the event loop to break out of the loop manually.
	/// - Setting the keyword  **<i>kw::allow_close()</i>** when creating the window allows the window to close the window, and  **<i>get_event()</i>** will return false
	///   under all circumstance because the window has been closed physically, rather than just sending the event.
	///
	/// **Note**
	///	
	/// - **<i>wait_event()</i>** and **<i>get_event()</i>** are the same function.
	/// - **<i>get_event()</i>** is deprecated and is included to maintain compatibility between platforms.
	///
	/// Example:
	///
	///         let my_button = Sagebox::dev_button("Press Me");
	///         let my_slider = Sagebox::dev_slider();
	///
	///         while win.get_event()
	/// 		{
	///             if my_button.pressed() { println!("My Button was Pressed"); }
	///
	/// 			let pos = my_slider.get_pos();
	///             if my_slider.moved() { println!("Range Slider is now at position",pos); }
	///			}
	///
	/// In this example, the program sleeps until an event occurs, and then the program checks to see if an event occured with a button or a slider movement. 
	///
	/// Events are typically one-time:  They report "true" on the first call, and then fals afterwards until another event of the same type happens. 
	/// 
	/// You can also use a callback to retrieve events.  Though this is not recommended or useful for most programs, it can be useful for some specific purposes.
	/// See set_event_callback() for more information.
	/// 
	pub fn get_event(&self) -> bool { unsafe { ext_func::sage_rust_window_get_event(self.id,false,false,true) } }


	/// [**get_event()**] - GetEvent waits for an event to occur in your main body of code. 
	///
	/// - Events are any events such as a mouse move, click, keyboard press, etc. 
	/// - Events are also caused by any control change such as moving a slider, pressing a button, or pressing return in an input box. 
	/// - Events can also be a window closing, a window moving or changing size -- basically anything happening in the system sends a message
	///
	/// Until an event occurs, the program is sleeping and not using any CPU time.  Sagebox wakes up the program when an event happens. 
	///
	/// In the event loop, you can check for events.  
	///
	/// get_event() returns true until all main windows are closed. 
	///
	/// **Function Parameter Forms:**
	///
	///		get_event()                      // returns true if an event occurs.  Returns false if that event was the user closed the window.
	///		get_event_s(allow_close : bool)	 // when false, does not return false if the user clicked to close the window. 
	///
	/// **Notes**
	///
	/// - When windows are created, the default setting is that the user cannot close the window. 
	///   - When the user clickes the **`X`** button in the upper-right, or presses **`ALT F4`** or otherwise tries to close the window.
	///   - this generates a **`Window Close Button Pressed`** event, which may be detected by calling **<i>win.close_button_pressed()</i>**
	/// - When **<i>get_event()</i>** and **<i>get_event_s(true)</i>** are used, this will detect this event and return false to allow an easy exit to the event loop
	/// - when **<i>get_event_s(false)</i>** is used, the function will not return false if a **`Window Close Button Pressed`** event is detected.
	///   - In this case, call **<i>win.close_button_pressed()</i>** within the event loop to break out of the loop manually.
	/// - Setting the keyword  **<i>kw::allow_close()</i>** when creating the window allows the window to close the window, and  **<i>get_event()</i>** will return false
	///   under all circumstance because the window has been closed physically, rather than just sending the event.
	///
	/// **Note**
	///	
	/// - **<i>wait_event()</i>** and **<i>get_event()</i>** are the same function.
	/// - **<i>get_event()</i>** is deprecated and is included to maintain compatibility between platforms.
	///
	/// Example:
	///
	///         let my_button = Sagebox::dev_button("Press Me");
	///         let my_slider = Sagebox::dev_slider();
	///
	///         while win.get_event()
	/// 		{
	///             if my_button.pressed() { println!("My Button was Pressed"); }
	///
	/// 			let pos = my_slider.get_pos();
	///             if my_slider.moved() { println!("Range Slider is now at position",pos); }
	///			}
	///
	/// In this example, the program sleeps until an event occurs, and then the program checks to see if an event occured with a button or a slider movement. 
	///
	/// Events are typically one-time:  They report "true" on the first call, and then fals afterwards until another event of the same type happens. 
	/// 
	/// You can also use a callback to retrieve events.  Though this is not recommended or useful for most programs, it can be useful for some specific purposes.
	/// See set_event_callback() for more information.
	/// 
	pub fn get_event_s(&self,allow_close : bool) -> bool { unsafe { ext_func::sage_rust_window_get_event(self.id,false,false,allow_close) } }



	/// [**wait_event()**] - GetEvent waits for an event to occur in your main body of code. 
	///
	/// - Events are any events such as a mouse move, click, keyboard press, etc. 
	/// - Events are also caused by any control change such as moving a slider, pressing a button, or pressing return in an input box. 
	/// - Events can also be a window closing, a window moving or changing size -- basically anything happening in the system sends a message
	///
	/// Until an event occurs, the program is sleeping and not using any CPU time.  Sagebox wakes up the program when an event happens. 
	///
	/// In the event loop, you can check for events.  
	///
	/// wait_event() returns true until all main windows are closed. 
	///
	/// **Function Parameter Forms:**
	///
	///		wait_event()                      // returns true if an event occurs.  Returns false if that event was the user closed the window.
	///		wait_event_s(allow_close : bool)	 // when false, does not return false if the user clicked to close the window. 
	///
	/// **Notes**
	///
	/// - When windows are created, the default setting is that the user cannot close the window. 
	///   - When the user clickes the **`X`** button in the upper-right, or presses **`ALT F4`** or otherwise tries to close the window.
	///   - this generates a **`Window Close Button Pressed`** event, which may be detected by calling **<i>win.close_button_pressed()</i>**
	/// - When **<i>wait_event()</i>** and **<i>wait_event_s(true)</i>** are used, this will detect this event and return false to allow an easy exit to the event loop
	/// - when **<i>wait_event_s(false)</i>** is used, the function will not return false if a **`Window Close Button Pressed`** event is detected.
	///   - In this case, call **<i>win.close_button_pressed()</i>** within the event loop to break out of the loop manually.
	/// - Setting the keyword  **<i>kw::allow_close()</i>** when creating the window allows the window to close the window, and  **<i>get_event()</i>** will return false
	///   under all circumstance because the window has been closed physically, rather than just sending the event.
	///
	/// **Note**
	///	
	/// - **<i>wait_event()</i>** and **<i>get_event()</i>** are the same function.
	/// - **<i>get_event()</i>** is deprecated and is included to maintain compatibility between platforms.
	///
	/// Example:
	///
	///         let my_button = Sagebox::dev_button("Press Me");
	///         let my_slider = Sagebox::dev_slider();
	///
	///         while win.wait_event()
	/// 		{
	///             if my_button.pressed() { println!("My Button was Pressed"); }
	///
	/// 			let pos = my_slider.get_pos();
	///             if my_slider.moved() { println!("Range Slider is now at position",pos); }
	///			}
	///
	/// In this example, the program sleeps until an event occurs, and then the program checks to see if an event occured with a button or a slider movement. 
	///
	/// Events are typically one-time:  They report "true" on the first call, and then fals afterwards until another event of the same type happens. 
	/// 
	/// You can also use a callback to retrieve events.  Though this is not recommended or useful for most programs, it can be useful for some specific purposes.
	/// See set_event_callback() for more information.
	/// 
	pub fn wait_event(&self) -> bool { unsafe { ext_func::sage_rust_window_get_event(self.id,false,false,true) } }

	/// [**wait_event()**] - GetEvent waits for an event to occur in your main body of code. 
	///
	/// - Events are any events such as a mouse move, click, keyboard press, etc. 
	/// - Events are also caused by any control change such as moving a slider, pressing a button, or pressing return in an input box. 
	/// - Events can also be a window closing, a window moving or changing size -- basically anything happening in the system sends a message
	///
	/// Until an event occurs, the program is sleeping and not using any CPU time.  Sagebox wakes up the program when an event happens. 
	///
	/// In the event loop, you can check for events.  
	///
	/// wait_event() returns true until all main windows are closed. 
	///
	/// **Function Parameter Forms:**
	///
	///		wait_event()                      // returns true if an event occurs.  Returns false if that event was the user closed the window.
	///		wait_event_s(allow_close : bool)	 // when false, does not return false if the user clicked to close the window. 
	///
	/// **Notes**
	///
	/// - When windows are created, the default setting is that the user cannot close the window. 
	///   - When the user clickes the **`X`** button in the upper-right, or presses **`ALT F4`** or otherwise tries to close the window.
	///   - this generates a **`Window Close Button Pressed`** event, which may be detected by calling **<i>win.close_button_pressed()</i>**
	/// - When **<i>wait_event()</i>** and **<i>wait_event_s(true)</i>** are used, this will detect this event and return false to allow an easy exit to the event loop
	/// - when **<i>wait_event_s(false)</i>** is used, the function will not return false if a **`Window Close Button Pressed`** event is detected.
	///   - In this case, call **<i>win.close_button_pressed()</i>** within the event loop to break out of the loop manually.
	/// - Setting the keyword  **<i>kw::allow_close()</i>** when creating the window allows the window to close the window, and  **<i>get_event()</i>** will return false
	///   under all circumstance because the window has been closed physically, rather than just sending the event.
	///
	/// **Note**
	///	
	/// - **<i>wait_event()</i>** and **<i>get_event()</i>** are the same function.
	/// - **<i>get_event()</i>** is deprecated and is included to maintain compatibility between platforms.
	///
	/// Example:
	///
	///         let my_button = Sagebox::dev_button("Press Me");
	///         let my_slider = Sagebox::dev_slider();
	///
	///         while win.wait_event()
	/// 		{
	///             if my_button.pressed() { println!("My Button was Pressed"); }
	///
	/// 			let pos = my_slider.get_pos();
	///             if my_slider.moved() { println!("Range Slider is now at position",pos); }
	///			}
	///
	/// In this example, the program sleeps until an event occurs, and then the program checks to see if an event occured with a button or a slider movement. 
	///
	/// Events are typically one-time:  They report "true" on the first call, and then fals afterwards until another event of the same type happens. 
	/// 
	/// You can also use a callback to retrieve events.  Though this is not recommended or useful for most programs, it can be useful for some specific purposes.
	/// See set_event_callback() for more information.
	/// 
	pub fn wait_event_s(&self,allow_close : bool) -> bool { unsafe { ext_func::sage_rust_window_get_event(self.id,false,false,allow_close) } }


	/// [**wait_for_close()**] - Waits for the user to close the window.
	/// 
	/// - This function waits for the user to close the window manually.
	/// - If the system is closing the window (e.g. a global message), this function will return.
	/// - If the window is invalid or already closed, this function returns immediately.
	/// - This function is useful before returning to the operating system if there is no event loop to avoid automatic closure of the window.
	///
	/// **Notes**
	///
	/// - See **<i>Sagebox::exit_button()</i>** which brings up a window with a button.
	/// - See **<i>win.exit_button()</i>** which brings up a button on the bottom of the window.
	/// 
	/// **Example**
	///
	///		win.write("Hello World");
	/// 	win.wait_for_close(); 	
	///
	pub fn wait_for_close(&self) { unsafe { ext_func::sage_rust_window_wait_for_close(self.id); } }

	/// [**update()**] - Updates the window with any contents that have been drawn or written to it since the last update.
	/// 
	/// - Depending on the window's mode, updates may occur automatically and semi-automatically
	///
	/// **Auto-Update Modes**
	/// 
	/// - **`ON`** - When auto update is ON, windows update on a periodic basis (about 10-20ms) while drawing, or whenever a Sagebox function is called that asks for input, creates a control, etc.
	///        - update() can be called here whenever a drawing function or set of functinos have finished.
	/// - **`OFF`** - When auto update of OFF, windows update only when a Sagebox function is called that creates a control, asks for input, etc.
	///       - update() almost never need to be called, except at the end of set of drawing or other window output where no Sagebox function is called for a noticeable time.
	/// - **`OnTime`** - Windows update at constant rate of about 10-20ms, but only if the window contents have changed since the last time.
	///          - update() never needs to be called.
	/// - **`Realtime`** - When a window is set to realtime() the window never updates unless update() is called.
	///            - update() is called whenever you with to update the window, as there is no automstic updating in realtime mode.
	///	
	/// **Notes**:
	///
	/// - Different update types make sense for different programs. 
	/// - Updating during drawing can cause shearing and flickering, whereas automatic or semi-automatic for non-realtime or graphics-oriented program can make things much hands-free by
	/// automatically updating the window for you.
	/// - Even when Updates are <i>**ON**</i> i.e. passive, <i>**get_event()**</i> updates any window that needs it.  If you're in an event loop, the Window will only not be updated (as needed) when
	/// updates are completely set to OFF.
	/// - See <i>**set_auto_update()**</i> for more information.
	///
	pub fn update(&self)
	{
		unsafe { ext_func::sage_rust_window_update(self.id); }
	}

	/// [**vsync_wait**] - Waits for the vertical sync to occur before returning. 
	/// 
	/// This function can be useful for real-time graphics to sync on the vertical refresh.
	/// 
	/// See the Sagebox GPU/SDL functions (TBD, but being worked on) for accurate and more flexible GPU graphic
	///
	pub fn vsync_wait(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_window_vsync_wait(self.id); }
		ret_val
	}

	/// [**set_smoothing_mode()**] - Sets the graphics anti-aliasing ON or OFF
	///
	/// - Default smoothing is Smooth (anti-aliased)
	/// - Anti-aliased (or smooth) graphics makes shapes blend in much better, and floating-point (x,y), (width,height) and radius values
	///   anti-alias properly in the window. 
	///   - With anti-aliasing off (e.g. None), these values are clipped to the nearest integer.
	///   - Shapes such as circles, rectangles, triangles, polygons, paths, lines, etc. respond to this mode.
	/// - Turning Smoothing OFF can be useful for performance under some circumstances
	/// - Some programs may prefer to have no anti-aliasing, leading to an edgier look 
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_smoothing_mode(smoothing_mode : GdiSmoothingMode)
	///
	/// **GDiSmoothingMode Values**
	/// 
	///		 Default     // Default setting (anti-aliasing/smooth)    
	///		 HighSpeed   // Does not anti-alias
	///		 HighQuality // Anti-Aliases
	///		 None        // Does not anti-alias
	///		 AntiAlias   // Anti-Aliases   
	///		 Smooth      // Anti-Aliases
	///
	/// **Notes:**
	///
	/// - Note that the values above are mostly duplicates.  The two main values that are useful are **`None`**, **`AntiAlias`** (or **`Smooth`**). 
	/// - These come from GDI values which are basically duplicated, for some reason, but are left here to support GDI usage in full.
	///
	/// **Examples**
	///
	///			win.set_smoothing_mode(GdiSmoothingMode::None);
	///
	pub fn set_smoothing_mode(&self,smoothing_mode : GdiSmoothingMode)
	{
		unsafe { ext_func::sage_rust_gdi_set_smoothing_mode(self.gdi.id,smoothing_mode as i32); }
	}
    /// [**set_auto_mouse_capture()**] - Tells the window to automatically capture the mouse when the mouse button is pressed. 
    ///
	/// - When the mouse is <i>captured</i> when the left mouse button is pressed, this means that the mouse can exceed the bounds of the window while the button remains pressed.
	///   - This means the mouse can return mouse point values outside of the range of the window dimensions, including negative values.
	///   - During this time, the window owns the mouse, and no other window or control can see the mouse move. 
	/// - This allows the mouse to drag items or set values outside of the window.  Once the mouse button is released, the capture is removed.
	/// - The Default for the window is to not automatically capture the mouse.
	///
	/// **Function Parameter Forms:** 
	///	
	///		set_auto_mouse_capture()                       // Sets auto-capture ON
	///		set_auto_mouse_capture_s(auto_capture : bool)  // Sets auto-capture ON if auto_capture is true, OFF if false.
	///
	/// **Notes:**
	///
    /// - See <i>**capture_mouse()**</i> to manually set this mode when needed.
    /// - Use <i>**reset_auto_mouse_capture()**</i> to change back to the default value, or call <i>**set_auto_mouse_capture(off)**</i> to turn auto-mouse capture off.
    /// - [**note:**]  when creating a window, the <i>**kw::auto_capture()**</i> keyword may be used to set this option as a window setting upon creation.
	///
	pub fn set_auto_mouse_capture(&self)
	{
		unsafe { ext_func::sage_rust_window_set_auto_mouse_capture(self.id,true); }
	}

    /// [**set_auto_mouse_capture()**] - Tells the window to automatically capture the mouse when the mouse button is pressed. 
    ///
	/// - When the mouse is <i>captured</i> when the left mouse button is pressed, this means that the mouse can exceed the bounds of the window while the button remains pressed.
	///   - This means the mouse can return mouse point values outside of the range of the window dimensions, including negative values.
	///   - During this time, the window owns the mouse, and no other window or control can see the mouse move. 
	/// - This allows the mouse to drag items or set values outside of the window.  Once the mouse button is released, the capture is removed.
	/// - The Default for the window is to not automatically capture the mouse.
	///
	/// **Function Parameter Forms:** 
	///	
	///		set_auto_mouse_capture()                       // Sets auto-capture ON
	///		set_auto_mouse_capture_s(auto_capture : bool)  // Sets auto-capture ON if auto_capture is true, OFF if false.
	///
	/// **Notes:**
	///
    /// - See <i>**capture_mouse()**</i> to manually set this mode when needed.
    /// - Use <i>**reset_auto_mouse_capture()**</i> to change back to the default value, or call <i>**set_auto_mouse_capture(off)**</i> to turn auto-mouse capture off.
    /// - [**note:**]  when creating a window, the <i>**kw::auto_capture()**</i> keyword may be used to set this option as a window setting upon creation.
	///
	pub fn set_auto_mouse_capture_s(&self,auto_capture : bool)
	{
		unsafe { ext_func::sage_rust_window_set_auto_mouse_capture(self.id,auto_capture); }
	}

	fn _set_font_i32(&self,font_type : i32)
	{
		unsafe { ext_func::sage_rust_window_set_font_int(self.id,font_type); }
	}
	fn _set_font_str(&self,font_type : &str)
	{
		unsafe { ext_func::sage_rust_window_set_font_str(self.id,font_type.as_ptr(),font_type.len()); }
	}

	/// [**display_bitmap()**] - Displays a 24-bit bitmap on the window
	///
	/// - Bitmaps use standard BGR (blue,green,red), upside-down format, with each bitmap line aligned on a 4-byte memory boundary.
	///
	/// **Function Parameter Forms:** 
	///	
	///			display_bitmap(location : (i32,i32), bitmap : &Bitmap)
	///			display_bitmap_s(location : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`Bitmap`** - a 24-bit bitmap to be displayed.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap_r()**</i> to display bitmaps upside-down.
	/// - **size**     - Sets the size of the bitmap -- the default is the same size as the bitmap. 
	///            - note: leaving one value at 0 will cause the bitmap to be scaled proportionally.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.display_bitmap((100,200),my_bitmap); 
	///	 win.display_bitmap_s((100,200),my_bitmap,kw::reversed())           // Displays the bitmap upside-down
	///	 win.display_bitmap_s((0,0),my_bitmap,kw::top_center() + pad_y(20));  // Displays bitmap top-center, then moves it 20 pixels down.
	///
	pub fn display_bitmap_s(&self,location : (i32,i32), bitmap : &Bitmap,keyw : kw)
	{
		unsafe { ext_func::sage_rust_window_display_bitmap(self.id,bitmap.id,location.0,location.1,keyw.pointer,false); }
	}
	/// [**display_bitmap()**] - Displays a 24-bit bitmap on the window
	///
	/// - Bitmaps use standard BGR (blue,green,red), upside-down format, with each bitmap line aligned on a 4-byte memory boundary.
	///
	/// **Function Parameter Forms:** 
	///	
	///			display_bitmap(location : (i32,i32), bitmap : &Bitmap)
	///			display_bitmap_s(location : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`Bitmap`** - a 24-bit bitmap to be displayed.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap_r()**</i> to display bitmaps upside-down.
	/// - **size**     - Sets the size of the bitmap -- the default is the same size as the bitmap. 
	///            - note: leaving one value at 0 will cause the bitmap to be scaled proportionally.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.display_bitmap((100,200),my_bitmap); 
	///	 win.display_bitmap_s((100,200),my_bitmap,kw::reversed())           // Displays the bitmap upside-down
	///	 win.display_bitmap_s((0,0),my_bitmap,kw::top_center() + pad_y(20));  // Displays bitmap top-center, then moves it 20 pixels down.
	///	
	pub fn display_bitmap(&self,location : (i32,i32), bitmap : &Bitmap)
	{
		self.display_bitmap_s(location,bitmap,KW_NONE);
	}

	/// [**display_bitmap_vec_s()**] — Displays a bitmap within a vector to the window
	///
	/// - Displays the vector data to the window, translating it as bitmap data.
	///
	/// **Function Parameter Forms:** 
	///	
	///			display_bitmap_vec(location : (i32,i32), size : (i32,i32), bitmap : &Vec<u8>
	///			display_bitmap_s(location : (i32,i32), size : (i32,i32), bitmap : &Vec<u8>,keyw : kw
	///
	/// - Vector data may be a monochrome Vec&lt;u8> or unaligned RGB Vec&lt;u8>, where each set of 3 values
	///in the order of blue,green, red.
	/// - Bitmaps are assume to be in upside-down order unless the **`kw::reverse()`** keyword is used.
	/// - [note:] Currently, this function supports only Vec&lt;<u8> vectors.
	///
	///   - An upcoming release will support more formats, such as i32, f32, 32-bit RGBA vectors, etc. 
	///
	/// - [The interface for this function is still being implemented]
	///
	pub fn display_bitmap_vec_s(&self,location : (i32,i32), size : (i32,i32), bitmap : &Vec<u8>,keyw : kw)
	{
		unsafe { ext_func::sage_rust_window_display_bitmap_vec(self.id,bitmap.as_ptr(),bitmap.len(),location.0,location.1,size.0,size.1,keyw.pointer); }
	}

	pub fn display_bitmap_vec(&self,location : (i32,i32), size: (i32,i32), bitmap : &Vec<u8>)
	{
		self.display_bitmap_vec_s(location,size,bitmap,KW_NONE);
	}


	/// [**display_bitmap_vec_s()**] — Displays a bitmap within a vector to the window
	///
	/// - Displays the vector data to the window, translating it as bitmap data.
	///
	/// **Function Parameter Forms:** 
	///	
	///			display_bitmap_vec(location : (i32,i32), size : (i32,i32), bitmap : &Vec<u8>
	///			display_bitmap_s(location : (i32,i32), size : (i32,i32), bitmap : &Vec<u8>,keyw : kw
	///
	/// - Vector data may be a monochrome Vec&lt;u8> or unaligned RGB Vec&lt;u8>, where each set of 3 values
	///in the order of blue,green, red.
	/// - Bitmaps are assume to be in upside-down order unless the **`kw::reverse()`** keyword is used.
	/// - [note:] Currently, this function supports only Vec&lt;<u8> vectors.
	///
	///   - An upcoming release will support more formats, such as i32, f32, 32-bit RGBA vectors, etc. 
	///
	/// - [The interface for this function is still being implemented]
	///
	pub fn display_bitmap_array_s(&self,location : (i32,i32), bitmap : &BitmapArray,keyw : kw)
	{
		unsafe { ext_func::sage_rust_window_display_bitmap_vec(self.id,bitmap.bitmap.as_ptr(),bitmap.bitmap.len(),location.0,location.1,bitmap.size.0,bitmap.size.1,keyw.pointer); }
	}

	/// ### [display_bitmap_array()] -  This function is currently TBD
	///
	pub fn display_bitmap_array(&self,location : (i32,i32), bitmap : &BitmapArray)
	{
		self.display_bitmap_array_s(location,bitmap,KW_NONE);
	}

	/// [**display_bitmap32**] - Displays a 32-bit bitmap on the window
	///
	/// - 32-bit Bitmaps use standard BGRA (blue,green,red), upside-down format
	/// - 32-bit bitmaps can also be displayed with transparency masks.  See blend_bitmap32().
	///
	/// **Function Parameter Forms:** 
	///	
	///			display_bitmap32(location : (i32,i32), bitmap : &Bitmap)
	///			display_bitmap32_s(location : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`Bitmap`** - a 32-bit bitmap to be displayed.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap32_r()**</i> to display bitmaps upside-down.
	/// - **size**     - Sets the size of the bitmap -- the default is the same size as the bitmap. 
	///            - note: leaving one value at 0 will cause the bitmap to be scaled proportionally.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.display_bitmap32((100,200),my_bitmap); 
	///	 win.display_bitmap32_s((100,200),my_bitmap,kw::reversed())           // Displays the bitmap upside-down
	///	 win.display_bitmap32_s((0,0),my_bitmap,kw::top_center() + pad_y(20));  // Displays bitmap top-center, then moves it 20 pixels down.
	///
	pub fn display_bitmap32_s(&self,location : (i32,i32), bitmap : &Bitmap32,keyw : kw)
	{
		unsafe { ext_func::sage_rust_window_display_bitmap(self.id,bitmap.id,location.0,location.1,keyw.pointer,true); }
	}

	/// [**display_bitmap32()**] - Displays a 32-bit bitmap on the window
	///
	/// - 32-bit Bitmaps use standard BGRA (blue,green,red), upside-down format
	/// - 32-bit bitmaps can also be displayed with transparency masks.  See blend_bitmap32().
	///
	/// **Function Parameter Forms:** 
	///	
	///			display_bitmap32(location : (i32,i32), bitmap : &Bitmap)
	///			display_bitmap32_s(location : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`Bitmap`** - a 32-bit bitmap to be displayed.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap32_r()**</i> to display bitmaps upside-down.
	/// - **size**     - Sets the size of the bitmap -- the default is the same size as the bitmap. 
	///            - note: leaving one value at 0 will cause the bitmap to be scaled proportionally.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.display_bitmap32((100,200),my_bitmap); 
	///	 win.display_bitmap32_s((100,200),my_bitmap,kw::reversed())           // Displays the bitmap upside-down
	///	 win.display_bitmap32_s((0,0),my_bitmap,kw::top_center() + pad_y(20));  // Displays bitmap top-center, then moves it 20 pixels down.
	///	
	pub fn display_bitmap32(&self,location : (i32,i32), bitmap : &Bitmap32)
	{
		self.display_bitmap32_s(location,bitmap,KW_NONE);
	}

	/// [**blend_bitmap32()**] - Blends a 32-bit bitmap with a transparency mask/alha channel onto the window
	///
	/// - This function blends the bitmap based on the Alpha channel transparency, blending the bitmap 
	/// onto the window.
	///   - If there is no Alpha channel (all 0s), the entire image is displayed without blending.
	/// - 32-bit Bitmaps use standard BGRA (blue,green,red), upside-down format
	/// 
	///
	/// **Function Parameter Forms:** 
	///	
	///			blend_bitmap32(location : (i32,i32), bitmap : &Bitmap)
	///			blend_bitmap32_s(location : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`Bitmap`** - a 32-bit bitmap to be blended.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap32_r()**</i> to display bitmaps upside-down.
	/// - **size**     - Sets the size of the bitmap -- the default is the same size as the bitmap. 
	///            - note: leaving one value at 0 will cause the bitmap to be scaled proportionally.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.blend_bitmap32((100,200),my_bitmap); 
	///	 win.blend_bitmap32_s((100,200),my_bitmap,kw::reversed())           // Blends the bitmap upside-down
	///	 win.blend_bitmap32_s((0,0),my_bitmap,kw::top_center() + pad_y(20));  // Blends bitmap top-center, then moves it 20 pixels down.
	///	
	pub fn blend_bitmap32_s(&self,location : (i32,i32), bitmap : &Bitmap32,keyw : kw)
	{
		unsafe { ext_func::sage_rust_window_blend_bitmap(self.id,bitmap.id,location.0,location.1,keyw.pointer,true); }
	}
	
	/// [**blend_bitmap32()**] - Blends a 32-bit bitmap with a transparency mask/alha channel onto the window
	///
	/// - This function blends the bitmap based on the Alpha channel transparency, blending the bitmap 
	/// onto the window.
	///   - If there is no Alpha channel (all 0s), the entire image is displayed without blending.
	/// - 32-bit Bitmaps use standard BGRA (blue,green,red), upside-down format
	/// 
	///
	/// **Function Parameter Forms:** 
	///	
	///			blend_bitmap32(location : (i32,i32), bitmap : &Bitmap)
	///			blend_bitmap32_s(location : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`Bitmap`** - a 32-bit bitmap to be blended.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap32_r()**</i> to display bitmaps upside-down.
	/// - **size**     - Sets the size of the bitmap -- the default is the same size as the bitmap. 
	///            - note: leaving one value at 0 will cause the bitmap to be scaled proportionally.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.blend_bitmap32((100,200),my_bitmap); 
	///	 win.blend_bitmap32_s((100,200),my_bitmap,kw::reversed())           // Blends the bitmap upside-down
	///	 win.blend_bitmap32_s((0,0),my_bitmap,kw::top_center() + pad_y(20));  // Blends bitmap top-center, then moves it 20 pixels down.
	///	
	pub fn blend_bitmap32(&self,location : (i32,i32), bitmap : &Bitmap32)
	{
		self.blend_bitmap32_s(location,bitmap,KW_NONE);
	}

	/// [**stretch_bitmap()**] - Displays a 24-bit bitmap on the window, stretched to the desired size
	///
	/// - Bitmaps use standard BGR (blue,green,red), upside-down format, with each bitmap line aligned on a 4-byte memory boundary.
	/// - [**note:**] stretch_bitmap() is the same as calling display_bitmap_s() with the kw::size() keyword.
	///
	/// **Function Parameter Forms:** 
	///	
	///			stretch_bitmap(location : (i32,i32), size : (i32,i32), bitmap : &Bitmap)
	///			stretch_bitmap_s(location : (i32,i32), size : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`size`** - Altered size of the bitmap to be displayed.
	///- **`Bitmap`** - a 24-bit bitmap to be displayed.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap_r()**</i> to display bitmaps upside-down.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.stretch_bitmap((100,200),(500,400),my_bitmap); 
	///	 win.stretch_bitmap_s((100,200),new_size,my_bitmap,kw::reversed())            // Displays the stretched bitmap upside-down
	///	 win.stretch_bitmap_s((0,0),new_size,my_bitmap,kw::top_center() + pad_y(20)); // Displays stretched bitmap top-center, then moves it 20 pixels down.
	///	
	pub fn stretch_bitmap_s(&self,location : (i32,i32), size : (i32,i32), bitmap : &Bitmap,keyw : kw)
	{
		unsafe { ext_func::sage_rust_window_stretch_bitmap(self.id,bitmap.id,location.0,location.1,size.0,size.1,keyw.pointer,false); }
	}

	/// [**stretch_bitmap()**] - Displays a 24-bit bitmap on the window, stretched to the desired size
	///
	/// - Bitmaps use standard BGR (blue,green,red), upside-down format, with each bitmap line aligned on a 4-byte memory boundary.
	/// - [**note:**] stretch_bitmap() is the same as calling display_bitmap_s() with the kw::size() keyword.
	///
	/// **Function Parameter Forms:** 
	///	
	///			stretch_bitmap(location : (i32,i32), size : (i32,i32), bitmap : &Bitmap)
	///			stretch_bitmap_s(location : (i32,i32), size : (i32,i32), bitmap : &Bitmap,keyw : kw) // When using keywords
	///
	/// **Parameters:**
	///
	/// - **`location`** - Position to display the bitmap in the window.
	///                  - Coordinates that cause the bitmap to exceed the window (or negative values) are fine.
	///                  - The location may be ignored of justification keywords are used (see examples)
	///- **`size`** - Altered size of the bitmap to be displayed.
	///- **`Bitmap`** - a 24-bit bitmap to be displayed.
	///                - If the bitmap is invalid or empty, the bitmap is ignored and not displayed.
	/// 
	/// **Useful Keywords for display_bitmap() (partial list)**
	/// 
	/// - **reversed** - This will cause the bitmap to be displayed upside-down
	///            - Also see <i>**display_bitmap_r()**</i> to display bitmaps upside-down.
	/// - <i>**Justification Keywords**</i> - Keywords such as <i>**top_center()**</i>, <i>**bottom_center()**</i>, <i>**top_right()**</i>, etc. will cause the 
	/// (x,y) coordinates to be ignored and justify the bitmap accordingly. 
	/// - **padx, pady** - <i>**padx()**</i> and <i>**pady()**</i> can be used to adjust the bitmap position, such as when using a justification keyword (e.g. top_center())
	///
	/// **Examples**
	///
	///	 win.stretch_bitmap((100,200),(500,400),my_bitmap); 
	///	 win.stretch_bitmap_s((100,200),new_size,my_bitmap,kw::reversed())            // Displays the stretched bitmap upside-down
	///	 win.stretch_bitmap_s((0,0),new_size,my_bitmap,kw::top_center() + pad_y(20)); // Displays stretched bitmap top-center, then moves it 20 pixels down.
	///	
	pub fn stretch_bitmap(&self,location : (i32,i32), size : (i32,i32), bitmap : &Bitmap)
	{
		self.stretch_bitmap_s(location,size,bitmap,KW_NONE);
	}

    /// [**set_as_topmost_window()**] - Sets the  Window as <i>topmost</i>, which means no other window can overlap it.  
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_as_topmost_window()                  // Sets window as topmost window
	///			set_as_topmost_window_s(top_most : bool) // Sets windows as topmost when true. Resets (undoes) tompost status when false.
	///
	/// - Other windows will be forced to remain underneath the the window, even if they are clicked on with the mouse.
	/// - The window cannot be overlapped by another window while in the <i>topmost</i> status.
	///   - The only exception is if another window has also been set with a <i>topmost</i> status.
	/// - <i>Topmost</i> status is retained the program ends or <i>**set_as_topmost_window(false)**</i> is called to remove the <i>topmost</i> status.
	///
	/// **Example**
	///
	///			win.set_as_topmost_window();
	///
	pub fn set_as_topmost_window_s(&self,top_most : bool) { unsafe { ext_func::sage_rust_window_set_z_position(self.id,top_most,0); } }

    /// [**set_as_topmost_window()**] - Sets the  Window as <i>topmost</i>, which means no other window can overlap it.  
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_as_topmost_window()                  // Sets window as topmost window
	///			set_as_topmost_window_s(top_most : bool) // Sets windows as topmost when true. Resets (undoes) tompost status when false.
	///
	/// - Other windows will be forced to remain underneath the the window, even if they are clicked on with the mouse.
	/// - The window cannot be overlapped by another window while in the <i>topmost</i> status.
	///   - The only exception is if another window has also been set with a <i>topmost</i> status.
	/// - <i>Topmost</i> status is retained the program ends or <i>**set_as_topmost_window(false)**</i> is called to remove the <i>topmost</i> status.
	///
	/// **Example**
	///
	///			win.set_as_topmost_window();
	///
	pub fn set_as_topmost_window(&self) { self.set_as_topmost_window_s(true); }

	/// [**disable_close()**] - Disable ability for user to close the window by pressing the upper-right "X" button or pressing ALT-F4
	///
	/// **Function Parameter Forms:** 
	///	
	///			disable_close()                  // Disable closing of the window by the user
	///			disable_close_s(disable : bool)  // Disable close if true, enables/re-enabled closing of the window when false.
	///
    /// - Calling <i>**win.close_button_pressed()**</i> can be used to determine if the user pressed the close button (or ALT-F4). 
	/// - Calling <i>**win.window_closed()**</i> can be used to determine if the window was actually closed, such as by the operating system, or the user
	/// has pressed F4 (or the 'X' button) with closing enabled.
	///
	pub fn disable_close_s(&self, disable : bool) { unsafe { ext_func::sage_rust_window_disable_close(self.id,disable,false); } }

	/// [**disable_close()**] - Disable ability for user to close the window by pressing the upper-right "X" button or pressing ALT-F4
	///
	/// **Function Parameter Forms:** 
	///	
	///			disable_close()                  // Disable closing of the window by the user
	///			disable_close_s(disable : bool)  // Disable close if true, enables/re-enabled closing of the window when false.
	///
 	///
	/// - By default, the user cannot intiate closing a window, and the **<i>`win.close_button_pressed()`</i>** function must be used to determine
	///that the user has attempted to close the window.
	///   - After which, the program can close the window itself with the **<i>win.close()</i>** function or simply exit the program.
	/// - If the closing the window is enabled, the user can tell Sagebox to close the window, which allows some Sagebox functions to react 
	///   and make programming a little easier:
	///   - **<i>get_event()</i>** will return false if the user has closed the window, obviating the need to check win.close_button_pressed()
	///   - The window will close on its own, which means it needs less management to determine its status.
	///
	/// - Calling <i>**win.close_button_pressed()**</i> can be used to determine if the user pressed the close button (or ALT-F4). 
	/// - Calling <i>**win.window_closed()**</i> can be used to determine if the window was actually closed, such as by the operating system, or the user
	/// has pressed F4 (or the 'X' button) with closing enabled.
	///
	pub fn disable_close(&self) { self.disable_close_s(true);} 

	/// [**enable_close()**] — Enable ability for user to close the window by pressing the upper-right "X" button or pressing ALT-F4
	///
	/// **Function Parameter Forms:** 
	///	
	///			enable_close()                  // Enable closing of the window by the user
	///			enable_close_s(enable : bool)  // Enable close if true, enables/re-enabled closing of the window when false.
	///
	/// - By default, the user cannot intiate closing a window, and the **<i>`win.close_button_pressed()`</i>** function must be used to determine
	///that the user has attempted to close the window.
	///   - After which, the program can close the window itself with the **<i>win.close()</i>** function or simply exit the program.
	/// - If the closing the window is enabled, the user can tell Sagebox to close the window, which allows some Sagebox functions to react 
	///   and make programming a little easier:
	///   - **<i>get_event()</i>** will return false if the user has closed the window, obviating the need to check win.close_button_pressed()
	///   - The window will close on its own, which means it needs less management to determine its status.
	///
    /// - Calling <i>**win.close_button_pressed()**</i> can be used to determine if the user pressed the close button (or ALT-F4). 
	/// - Calling <i>**win.window_closed()**</i> can be used to determine if the window was actually closed, such as by the operating system, or the user
	/// has pressed F4 (or the 'X' button) with closing enabled.
	///
	pub fn enable_close_s(&self, enable : bool) { unsafe { ext_func::sage_rust_window_disable_close(self.id,enable,true); } }

	/// [**enable_close()**] — Enable ability for user to close the window by pressing the upper-right "X" button or pressing ALT-F4
	///
	/// **Function Parameter Forms:** 
	///	
	///			enable_close()                  // Enable closing of the window by the user
	///			enable_close_s(enable : bool)  // Enable close if true, enables/re-enabled closing of the window when false.
	///
	/// - By default, the user cannot intiate closing a window, and the **<i>`win.close_button_pressed()`</i>** function must be used to determine
	///that the user has attempted to close the window.
	///   - After which, the program can close the window itself with the **<i>win.close()</i>** function or simply exit the program.
	/// - If the closing the window is enabled, the user can tell Sagebox to close the window, which allows some Sagebox functions to react 
	///   and make programming a little easier:
	///   - **<i>get_event()</i>** will return false if the user has closed the window, obviating the need to check win.close_button_pressed()
	///   - The window will close on its own, which means it needs less management to determine its status.
	///
    /// - Calling <i>**win.close_button_pressed()**</i> can be used to determine if the user pressed the close button (or ALT-F4). 
	/// - Calling <i>**win.window_closed()**</i> can be used to determine if the window was actually closed, such as by the operating system, or the user
	/// has pressed F4 (or the 'X' button) with closing enabled.
	///
	pub fn enable_close(&self) { self.enable_close_s(true);} 

	/// [**mouse_drag_event()**] - Returns **`true`** if a <u><i>mouse drag event</i></u> has occured in the window.
	/// 
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	/// - This may include the first click of the mouse (before moving it) if the parameter **`include_mouse_click`** is set to **`true`**.
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_drag_event(include_mouse_click : bool)
	///			mouse_drag_event_s(include_mouse_click : bool, point : &mut Point<i32>)
	///			mouse_drag_event_peek(include_mouse_click : bool)
	///			mouse_drag_event_peek_s(include_mouse_click : bool, point : &mut Point<i32>)
	///
	/// **Parameters:** 
	/// 
	/// - **`include_mouse`** - When true, the mouse drag event will include the first mouse click. 
	///                       - All other mouse drag events are based on the mouse movement after the first click.
	///                       - When false, the first click is not included, and mouse drag events only 
	///                         refer to the mouse being moved while the mouse button is pressed.
	/// 					  - The originating mouse point (the first mouse click) can be retrieved with <i>**get_mouse_drag_start()**</i>.
	/// - **`point`** - when this value is used with <i>**mouse_drag_event_s()**</i>, the `point` value is filled with the mouse coordinates of the drag event, if the drag event returns true.
	///           - If there was no drag event (<i>**mouse_drag_event_s()**</i> returns **`false`**), this value is not altered.	
	///
	/// <u><i>**mouse_drag_event_peek()**</i></u>
	///
	/// - When this form is used, this will return the drag event status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_drag_event()**</i> is used without peeking, the mouse drag status is reset until the next mouse drag event.
	/// 
	/// **Examples:** 
	/// 
	/// 	let my_point : Point::<i32>::default(); 
	///
	/// 	win.mouse_drag_event(false)             // Returns true if the mouse is being dragged (does not include initial click)
	/// 	win.mouse_drag_event_peek(true)         // Include the initial mouse button click, and do not reset the event
	/// 	win.mouse_drag_event_s(true,&my_point)  // Get the mouse drag event, and if it returns true, fill my_point with the mouse position.
	///
	pub fn mouse_drag_event_peek_s(&self,include_mouse_click : bool,point : &mut Point<i32>) -> bool
	{
		let _point = (0,0);

 		unsafe { let point2 = *ext_func::sage_rust_window_mouse_drag_event(self.id,include_mouse_click,true,&_point); 
			*point = if point2.0 > -50000 { Point::<i32>::fromi32(point2) } else { Point::<i32>::default() };
			point2.0 > -50000 }
	}

	/// [**mouse_drag_event()**] - Returns **`true`** if a <u><i>mouse drag event</i></u> has occured in the window.
	/// 
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	/// - This may include the first click of the mouse (before moving it) if the parameter **`include_mouse_click`** is set to **`true`**.
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_drag_event(include_mouse_click : bool)
	///			mouse_drag_event_s(include_mouse_click : bool, point : &mut Point<i32>)
	///			mouse_drag_event_peek(include_mouse_click : bool)
	///			mouse_drag_event_peek_s(include_mouse_click : bool, point : &mut Point<i32>)
	///
	/// **Parameters:** 
	/// 
	/// - **`include_mouse`** - When true, the mouse drag event will include the first mouse click. 
	///                       - All other mouse drag events are based on the mouse movement after the first click.
	///                       - When false, the first click is not included, and mouse drag events only 
	///                         refer to the mouse being moved while the mouse button is pressed.
	/// 					  - The originating mouse point (the first mouse click) can be retrieved with <i>**get_mouse_drag_start()**</i>.
	/// - **`point`** - when this value is used with <i>**mouse_drag_event_s()**</i>, the `point` value is filled with the mouse coordinates of the drag event, if the drag event returns true.
	///           - If there was no drag event (<i>**mouse_drag_event_s()**</i> returns **`false`**), this value is not altered.	
	///
	/// <u><i>**mouse_drag_event_peek()**</i></u>
	///
	/// - When this form is used, this will return the drag event status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_drag_event()**</i> is used without peeking, the mouse drag status is reset until the next mouse drag event.
	/// 
	/// **Examples:** 
	/// 
	/// 	let my_point : Point::<i32>::default(); 
	///
	/// 	win.mouse_drag_event(false)             // Returns true if the mouse is being dragged (does not include initial click)
	/// 	win.mouse_drag_event_peek(true)         // Include the initial mouse button click, and do not reset the event
	/// 	win.mouse_drag_event_s(true,&my_point)  // Get the mouse drag event, and if it returns true, fill my_point with the mouse position.
	///
	pub fn mouse_drag_event_s(&self,include_mouse_click : bool, point : &mut Point<i32>) -> bool
	{
	let _point = (0,0);

 		unsafe { let point2 = *ext_func::sage_rust_window_mouse_drag_event(self.id,include_mouse_click,false,&_point); 
			*point = if point2.0 > -50000 { Point::<i32>::fromi32(point2) } else { Point::<i32>::default() };
			point2.0 > -50000 }
	}

	/// [**mouse_drag_event()**] - Returns **`true`** if a <u><i>mouse drag event</i></u> has occured in the window.
	/// 
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	/// - This may include the first click of the mouse (before moving it) if the parameter **`include_mouse_click`** is set to **`true`**.
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_drag_event(include_mouse_click : bool)
	///			mouse_drag_event_s(include_mouse_click : bool, point : &mut Point<i32>)
	///			mouse_drag_event_peek(include_mouse_click : bool)
	///			mouse_drag_event_peek_s(include_mouse_click : bool, point : &mut Point<i32>)
	///
	/// **Parameters:** 
	/// 
	/// - **`include_mouse`** - When true, the mouse drag event will include the first mouse click. 
	///                       - All other mouse drag events are based on the mouse movement after the first click.
	///                       - When false, the first click is not included, and mouse drag events only 
	///                         refer to the mouse being moved while the mouse button is pressed.
	/// 					  - The originating mouse point (the first mouse click) can be retrieved with <i>**get_mouse_drag_start()**</i>.
	/// - **`point`** - when this value is used with <i>**mouse_drag_event_s()**</i>, the `point` value is filled with the mouse coordinates of the drag event, if the drag event returns true.
	///           - If there was no drag event (<i>**mouse_drag_event_s()**</i> returns **`false`**), this value is not altered.	
	///
	/// <u><i>**mouse_drag_event_peek()**</i></u>
	///
	/// - When this form is used, this will return the drag event status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_drag_event()**</i> is used without peeking, the mouse drag status is reset until the next mouse drag event.
	/// 
	/// **Examples:** 
	/// 
	/// 	let my_point : Point::<i32>::default(); 
	///
	/// 	win.mouse_drag_event(false)             // Returns true if the mouse is being dragged (does not include initial click)
	/// 	win.mouse_drag_event_peek(true)         // Include the initial mouse button click, and do not reset the event
	/// 	win.mouse_drag_event_s(true,&my_point)  // Get the mouse drag event, and if it returns true, fill my_point with the mouse position.
	///
	pub fn mouse_drag_event(&self,include_mouse_click : bool) -> bool
	{
		let mut _point = Point::<i32>::new(0,0);
		self.mouse_drag_event_s(include_mouse_click,&mut _point)
	}

	/// [**mouse_drag_event()**] - Returns **`true`** if a <u><i>mouse drag event</i></u> has occured in the window.
	/// 
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	/// - This may include the first click of the mouse (before moving it) if the parameter **`include_mouse_click`** is set to **`true`**.
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_drag_event(include_mouse_click : bool)
	///			mouse_drag_event_s(include_mouse_click : bool, point : &mut Point<i32>)
	///			mouse_drag_event_peek(include_mouse_click : bool)
	///			mouse_drag_event_peek_s(include_mouse_click : bool, point : &mut Point<i32>)
	///
	/// **Parameters:** 
	/// 
	/// - **`include_mouse`** - When true, the mouse drag event will include the first mouse click. 
	///                       - All other mouse drag events are based on the mouse movement after the first click.
	///                       - When false, the first click is not included, and mouse drag events only 
	///                         refer to the mouse being moved while the mouse button is pressed.
	/// 					  - The originating mouse point (the first mouse click) can be retrieved with <i>**get_mouse_drag_start()**</i>.
	/// - **`point`** - when this value is used with <i>**mouse_drag_event_s()**</i>, the `point` value is filled with the mouse coordinates of the drag event, if the drag event returns true.
	///           - If there was no drag event (<i>**mouse_drag_event_s()**</i> returns **`false`**), this value is not altered.	
	///
	/// <u><i>**mouse_drag_event_peek()**</i></u>
	///
	/// - When this form is used, this will return the drag event status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_drag_event()**</i> is used without peeking, the mouse drag status is reset until the next mouse drag event.
	/// 
	/// **Examples:** 
	/// 
	/// 	let my_point : Point::<i32>::default(); 
	///
	/// 	win.mouse_drag_event(false)             // Returns true if the mouse is being dragged (does not include initial click)
	/// 	win.mouse_drag_event_peek(true)         // Include the initial mouse button click, and do not reset the event
	/// 	win.mouse_drag_event_s(true,&my_point)  // Get the mouse drag event, and if it returns true, fill my_point with the mouse position.
	///
	pub fn mouse_drag_event_peek(&self,include_mouse_click : bool) -> bool
	{
			let mut _point = Point::<i32>::new(0,0);
		self.mouse_drag_event_peek_s(include_mouse_click,&mut _point)
	}


	/// [**mouse_drag_get_pos()**] - Returns the position of the <u><i>last mouse drag event</i></u>
	/// 
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	///
	/// **Example:** 
	/// 
	/// 	if win.mouse_drag_event(true)
	/// 	{
	/// 		let mouse_pos = win.mouse_drag_get_pos();
	/// 		println!("drag event occured at {},{}",mouse_pos.x,mouse_pos.y);
	/// 	}
	///
	pub fn mouse_drag_get_pos(&self) -> Point<i32>	
	{
		let point = (0,0);
		unsafe { let _point2  = *ext_func::sage_rust_window_mouse_drag_get_pos(self.id,&point,0); 
		
		Point::<i32>::fromi32(_point2)  }
	}
	/// [**mouse_drag_get_start()**] - Returns the position of the first <u><i>mouse drag event</i></u> mouse position.
	///
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	///
	/// **Example:** 
	/// 
	/// 	if win.mouse_drag_event(false)
	/// 	{
	/// 		let mouse_pos = win.mouse_drag_get_pos();
	/// 		let dist = mouse_pos-win.mouse_drag_get_start();
	/// 		println!("mouse has moved {},{} during this mouse-drag session",dist.x,dist.y);
	/// 	}
	///
	pub fn mouse_drag_get_start(&self) -> Point<i32>
	{
		let _point = (0,0);
		unsafe { let _point2  = *ext_func::sage_rust_window_mouse_drag_get_pos(self.id,&_point,1); 
		
		Point::<i32>::fromi32(_point2)  }	
	}

	/// [**mouse_drag_ended()**] - Returns **`true`** if the user has released the mouse after moving it (initiating a mouse drag event and session)
	///
	/// - When the mouse is clicked and unclicked without moving, this function will return **`true`**, even though the mouse has not been moved.
	///   - In this case, the function <i>**mouse_drag_event()**</i> may have never returned true to initiate a <u><i>mouse drag session</i></u>. 
	/// - use <i>**mouse_drag_get_pos()**</i> and <i>**mouse_drag_get_start()**</i> to get the first and last points of the <i>mouse drag session</i>.
	/// - A <u><i>mouse drag event</i></u> is when the mouse is moved while the left mouse button is pressed.
	///
	/// **Example:** 
	/// 
	/// 	if win.mouse_drag_ended()
	/// 	{
	/// 		let mouse_pos = win.mouse_drag_get_pos();
	/// 		let dist = mouse_pos-win.mouse_drag_get_start();
	/// 		println!("mouse has moved {},{} during this mouse-drag session",dist.x,dist.y);
	/// 	}
	///
	pub fn mouse_drag_ended(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_mouse_drag_ended(self.id) }
	}

    /// [**mouse_clicked()**] - Returns **`true`** if the <i>left mouse button</i> was clicked, **`false`** if not and for subsequent events until the next mouse click.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_clicked()
	///			mouse_clicked_peek()
	///
	/// <u><i>**mouse_clicked_peek()**</i></u>
	///
	/// - When this form is used, this will return the mouse-clicked status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_clicked()**</i> is used without peeking, the mouse-clicked status is reset until the next time the user clicks the mouse.
	///
	pub fn mouse_clicked_peek(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,true,false,false) }
	}

    /// [**mouse_clicked()**] - Returns **`true`** if the <i>left mouse button</i> was clicked, **`false`** if not and for subsequent events until the next mouse click.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_clicked()
	///			mouse_clicked_peek()
	///
	/// <u><i>**mouse_clicked_peek()**</i></u>
	///
	/// - When this form is used, this will return the mouse-clicked status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_clicked()**</i> is used without peeking, the mouse-clicked status is reset until the next time the user clicks the mouse.
	///
	/// **Note**
	///
	/// - **<i>mouse_clicked()</i>** and **<i>mouse_button_clicked()</i>** are the same function
	///
	pub fn mouse_clicked(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,false,false,false) } }


    /// [**mouse_button_clicked()**] - Returns **`true`** if the <i>left mouse button</i> was clicked, **`false`** if not and for subsequent events until the next mouse click.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_button_clicked()
	///			mouse_button_clicked_peek()
	///
	/// <u><i>**mouse_clicked_peek()**</i></u>
	///
	/// - When this form is used, this will return the mouse-clicked status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_clicked()**</i> is used without peeking, the mouse-clicked status is reset until the next time the user clicks the mouse.
	///
	/// **Note**
	///
	/// - **<i>mouse_clicked()</i>** and **<i>mouse_button_clicked()</i>** are the same function
	///
	pub fn mouse_button_clicked_peek(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,true,false,false) }
	}

    /// [**mouse_button_clicked()**] - Returns **`true`** if the <i>left mouse button</i> was clicked, **`false`** if not and for subsequent events until the next mouse click.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_button_clicked()
	///			mouse_button_clicked_peek()
	///
	/// <u><i>**mouse_clicked_peek()**</i></u>
	///
	/// - When this form is used, this will return the mouse-clicked status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_clicked()**</i> is used without peeking, the mouse-clicked status is reset until the next time the user clicks the mouse.
	///
	/// **Note**
	///
	/// - **<i>mouse_clicked()</i>** and **<i>mouse_button_clicked()</i>** are the same function
	///
	pub fn mouse_button_clicked(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,false,false,false) } }

	/// [**mouse_button_down()**] — Returns true if the left mouse button is currently pressed by the user.
	///
	/// - This is not an event-based function such as <i>**mouse_clicked()**</i> or <i>**mouse_moved()**</i>.
	/// - This function returns true of the mouse button is down at the time it is called.
	/// - See <i>**mouse_button_released()**</i> to determine if the user released the left mouse button and caused an event. 
	///
	pub fn mouse_button_down(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,false,false,true) } }

    /// [**mouse_r_button_clicked()**] - Returns **`true`** if the <i>right mouse button</i> was clicked, **`false`** if not and for subsequent events until the next right-mouse click.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_r_button_clicked()
	///			mouse_r_button_clicked_peek()
	///
	/// <u><i>**mouse_r_button_clicked_peek()**</i></u>
	///
	/// - When this form is used, this will return the right-mouse-clicked status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_r_button_clicked()**</i> is used without peeking, the right-mouse-clicked status is reset until the next time the user clicks the mouse.
	///
	pub fn mouse_r_button_clicked_peek(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,true,true,false) } }

    /// [**mouse_r_button_clicked()**] - Returns **`true`** if the <i>right mouse button</i> was clicked, **`false`** if not and for subsequent events until the next right-mouse click.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_r_button_clicked()
	///			mouse_r_button_clicked_peek()
	///
	/// <u><i>**mouse_r_button_clicked_peek()**</i></u>
	///
	/// - When this form is used, this will return the right-mouse-clicked status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_r_button_clicked()**</i> is used without peeking, the right-mouse-clicked status is reset until the next time the user clicks the mouse.
	///
	pub fn mouse_r_button_clicked(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,false,true,false) } }

	/// [**mouse_r_button_down()**] — Returns true if the right mouse button is currently pressed by the user.
	///
	/// - This is not an event-based function such as <i>**mouse_r_button_clicked()**</i> or <i>**mouse_moved()**</i>.
	/// - This function returns true of the right mouse button is down at the time it is called.
	/// - See <i>**mouse_button_r_released()**</i> to determine if the user released the right mouse button and caused an event. 
	///
	pub fn mouse_r_button_down(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_clicked(self.id,false,true,true) } }


	/// [**mouse_wheel_moved()**] — Returns true if the mouse wheel was moved.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_wheel_moved()
	///			mouse_wheel_moved_peek()
	///
	/// <u><i>**mouse_wheel_moved_peek()**</i></u>
	///
	/// - When this form is used, this will return the mouse-wheel-moved status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_wheel_moved()**</i> is used without peeking, the mouse-wheel-moved status is reset until the next time the user moves the mouse wheel.
	///
	/// **Notes**
	///
	/// - See <i>**get_mouse_wheel_move()**</i> to get the value of the mouse wheel movement (e.g. how much the mouse wheel moved and in what direction)
	///
	pub fn mouse_wheel_moved(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_wheel_moved(self.id,false,false) != 0 } }

	/// [**mouse_wheel_moved()**] — Returns true if the mouse wheel was moved.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			mouse_wheel_moved()
	///			mouse_wheel_moved_peek()
	///
	/// <u><i>**mouse_wheel_moved_peek()**</i></u>
	///
	/// - When this form is used, this will return the mouse-wheel-moved status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**mouse_wheel_moved()**</i> is used without peeking, the mouse-wheel-moved status is reset until the next time the user moves the mouse wheel.
	///
	/// **Notes**
	///
	/// - See <i>**get_mouse_wheel_move()**</i> to get the value of the mouse wheel movement (e.g. how much the mouse wheel moved and in what direction)
	///
	pub fn mouse_wheel_moved_peek(&self) -> bool { unsafe { ext_func::sage_rust_window_mouse_wheel_moved(self.id,false,true) != 0 } }
	
	/// [**get_mouse_wheel_move()**] — Returns the value of the last mouse wheel movement. 
	///
	/// - When the mouse wheel moves, it can have a positive or negative direction.
	///   - The value returned will be positive for forward movement, and negative for backward movement.
	/// - The value returned can be 1, 2, 3 .. etc.   This is effectively how many mouse wheel clicks occured.
	/// - 0 is returned if no mouse wheel after the first call, and until the mousewheel moves again.
	///
	/// - See <i>**mouse_wheel_moved()**</i> to determine when a mouse-wheel-moved event occurred to signal checking the value with <i>**get_mouse_wheel_move()**</i>
	///
	pub fn get_mouse_wheel_move(&self) -> i32 { unsafe { ext_func::sage_rust_window_mouse_wheel_moved(self.id,true,false) } }

	/// [**get_mouse_pos()**] - Returns the current Mouse Position. 
	///
	/// **Example:**
	///
	/// 	if win.mouse_clicked()
	/// 	{ 
	/// 		let mouse_pos = win.get_mouse_pos(); 
	///			println!("The Mouse was clicked at position {},{}",mouse_pos.x,mouse_pos.y);
	/// 	}
	pub fn get_mouse_pos<'a>(&self) -> Point<i32>
	{
		Point::<i32>::fromi32(unsafe { *ext_func::sage_rust_window_mouse_get_pos(self.id) })
	}

	/// [**show()**] -> Shows (makes visible) or hides the window. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			show()       // Make the window visible if it was hidden.
	///			show_s(show_window : bool)  // Makes the window visible if true
	///                                     // hides the window if false.
	///
	/// - When new windows are created, they default to a visible status.  Using <i>**show(false)**</i> may be used to hide the window.
	/// - if the <i>**kw::hidden()**</i> keyword is used when creating the window, the window will be created as hidden/invisible.
	///   - using <i>**show()**</i> or <i>**show_s(true)**</i> will make the window visible on the screen.
	///
 	pub fn show_s(&self,show_window : bool)
	{
		unsafe { ext_func::sage_rust_window_show_window(self.id,show_window) }
	}

	/// [**show()**] -> Shows (makes visible) or hides the window. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			show()       // Make the window visible if it was hidden.
	///			show_s(show_window : bool)  // Makes the window visible if true
	///                                     // hides the window if false.
	///
	/// - When new windows are created, they default to a visible status.  Using <i>**show(false)**</i> may be used to hide the window.
	/// - if the <i>**kw::hidden()**</i> keyword is used when creating the window, the window will be created as hidden/invisible.
	///   - using <i>**show()**</i> or <i>
	///
	pub fn show(&self)
	{
		self.show_s(true);
	}

	/// [**get_window_bitmap()**] - Returns a bitmap with a copy of the contents of the window. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			get_window_bitmap()
	///			get_window_bitmap_s(upper_left : (i32,i32), size : (i32,i32))
	///			get_window_bitmap_s(upper_left : Point<i32>, size : Point<i32>)
	///                                     
	/// **Parameters:**
    /// 
    /// - **`upper_left`**  - Upper-left coordinate of the window where the bitmap should start.
    /// - **`size`** - size of the bitmap top copy. 
	///
	/// **Notes:**
	///
	/// - using <i>**get_window_bitmap()**</i> (i.e. with no parameters()) copies the entire window area.
	///
	///  **Example:**
	///
	///			win.get_window_bitmap(); 		// Copy the entire contents of the window
	///			win.get_window_bitmap((100,200),(400,400)); // Copy a 400x400 bitmap starting at location (100,200)
	///
	pub fn get_window_bitmap(&self) -> Bitmap
	{
		unsafe { Bitmap::create(ext_func::sage_rust_window_get_window_bitmap(self.id,-1,-1,-1,-1)) }
	}
	fn _get_window_bitmap_s(&self,upper_left : (i32,i32), size : (i32,i32)) -> Bitmap
	{
		unsafe { Bitmap::create(ext_func::sage_rust_window_get_window_bitmap(self.id,upper_left.0,upper_left.1,size.0,size.1)) }
	}

	/// [**close_button_pressed()**] - Returns a **`true`** if the user pressed the 'X' button on the window to close it, or pressed F4 to close the window.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	/// 
	///  **Example:**
	///
	///			if win.close_button_pressed()
	/// 		{
	///				win.show(false); 	// Hide the window
	///             println!("The window was closed by the user.")
	///			}
	///
	pub fn close_button_pressed(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_close_button_pressed(self.id,false) }
	}
	pub fn close_button_pressed_peek(&self) -> bool
	{
		unsafe { ext_func::sage_rust_window_close_button_pressed(self.id,true) }
	}

	/// [**event_pending()**] — Returns true if an event has occurred in any window or control since the last time called. 
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	/// 
	/// **Function Parameter Forms:** 
	///	
	///			event_pending()
	///			event_pending_peek()
	///
	/// **Usefulness of <i>event_pending()</i>**
	///
	/// - In tight, real-time loops, it can be a little time-consuming (i.e. 500us-1ms, depending on how many controls are checked)
	///   - More time if values needed to be converted or translated by code to check specific events. 
	/// - calling **<i>event_pending()</i>** will tell you if there is a reason to check events. 
	/// - If **<i>event_pending()</i>** returns false, there are no events that will return a true status, and, therefore, no reason to check any events.
	/// - When **<i>event_pending()</i>** returns true, this means some event occured, such as a mouse movement, mouse click, button press, or other control activity. 
	///
	/// <u><i>**event_pending_peek()**</i></u>
	///
	/// - When this form is used, this will return the event-pending status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**event_pending()**</i> is used without peeking, event-pending status is reset until the next time the user moves the mouse wheel.
	///
	/// **Example**
	///
	///			while win.vsync_wait()
	///			{
	///				// No reason to spend time looking at events if there is nothing to check
	///			
	///				if win.event_pending()
	///				{
	///				   	if win.mouse_clicked() { handle_mouse_click(); }
	///				   	if win.mouse_moved()) { handle_mouse_moved()); }
	///				   	if win.strength_slider.moved()) { handle_slider_movement(); }
	///				}
	///				render_frame();
	///			}
	///
	pub fn event_pending(&self) -> bool { unsafe { ext_func::sage_rust_window_get_event(self.id,true,false,false) } }


	/// [**event_pending()**] — Returns true if an event has occurred in any window or control since the last time called. 
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	/// 
	/// **Function Parameter Forms:** 
	///	
	///			event_pending()
	///			event_pending_peek()
	///
	/// **Usefulness of <i>event_pending()</i>**
	///
	/// - In tight, real-time loops, it can be a little time-consuming (i.e. 500us-1ms, depending on how many controls are checked)
	///   - More time if values needed to be converted or translated by code to check specific events. 
	/// - calling **<i>event_pending()</i>** will tell you if there is a reason to check events. 
	/// - If **<i>event_pending()</i>** returns false, there are no events that will return a true status, and, therefore, no reason to check any events.
	/// - When **<i>event_pending()</i>** returns true, this means some event occured, such as a mouse movement, mouse click, button press, or other control activity. 
	///
	/// <u><i>**event_pending_peek()**</i></u>
	///
	/// - When this form is used, this will return the event-pending status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**event_pending()**</i> is used without peeking, event-pending status is reset until the next time the user moves the mouse wheel.
	///
	/// **Example**
	///
	///			while win.vsync_wait()
	///			{
	///				// No reason to spend time looking at events if there is nothing to check
	///			
	///				if win.event_pending()
	///				{
	///				   	if win.mouse_clicked() { handle_mouse_click(); }
	///				   	if win.mouse_moved()) { handle_mouse_moved()); }
	///				   	if win.strength_slider.moved()) { handle_slider_movement(); }
	///				}
	///				render_frame();
	///			}
	///
	pub fn event_pending_peek(&self) -> bool { unsafe { ext_func::sage_rust_window_get_event(self.id,true,true,false) } }

	// Controls


   	/// [**text_widget()**] - Creates a text widget in the Window. 
   	/// 
   	/// - Returns a <i>TextWidget</i> object where you can control and write to the text widget.
   	/// - The created text widget can be used to create static or dynamic text of any font size in the window.
	/// - Text Widget Objects returned (<i>TextWidget</i> type) does not need to be kept of the text widget will never be updated once the text widget is created.  Otherwise,
	/// the returned <i>TextWidget</i> object can be used to change the text or other aspects of the text widget.
	///
	/// **Function Parameter Forms:**
	///
	///		text_widget(location,title : &str)              // Creates a text widget with a a title
	///		text_widget_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the text widget characteristics
	///
	/// **Some Available Keywords with new_text_widget()**
	/// 
	/// Keyword | Action
	///---- | ----
   	/// text  | - Sets the text of the widget.  This can be set later with <i>text_widget.write()</i>. When text is entered, the text widget is created to the width of the text.  Use the width() parameter to set a width or pad the text with spaces to reserve width.
   	/// text_color  | - Sets the text color of the widget (default is current window text color).  Same as <i>kw::fg_color()</i>
   	/// font       | - Sets the font of the text in the text widget
   	/// text_center  | - Centers the text inside of the widget (which can be longer than the text itself). - Use <i>text_center_x()</i> and <i>center_x()</i> together to make sure text is centered in the window. This is only needed if the Width of the   	///                         - Text Widget and the text have been specificed separately.
   	/// just_center_x (or <i>center_x<i>) | - Centers the widget in horizontally at the Y position provided (as opposed to justifying the text within the text widget).
	/// just_top_center() | - This and other justification keywords can be used to place the widget around the window.
	///
	/// **Notes**
	///
	/// - The text widget is placed at the location provided, unless justification keywords are used
	/// - When justifications are used, 0 may be used in place of the affected values,
	///   - e.g. win.text_widget_s(0,200,"This is a widget",kw::center_x()), which discards the X value provided to center the text widget in the window horizontally, keeping the Y position.
	/// - Use pad_x() and pad_y() to refine the positon of the text widget when using justification-based keywords.
	/// - The returned TextWidget object can be disgregarded if no changes to the widget will occur after the original call. 
	///   - The text widget will be destroyed when the window is closed.
	///
   	/// Examples:
	///
   	///		let widget = win.text_widget("This is a text Widget"); // Create a basic text widget with text.  Return value does not need to be kept.
   	///		let widget = win.text_widget("This is a text Widget",kw::font(15) + kw::fg_color("red")) // Create a text widget with text.  That has a font size of 15 and a text color of red.
	///
	pub fn text_widget_s(&self,location : (i32,i32), initial_text : &str,keyw : kw) -> TextWidget
	{
		TextWidget { id : unsafe { ext_func::sage_rust_window_new_text_widget(self.id, location.0, location.1,initial_text.as_ptr(),initial_text.len(),keyw.pointer ) } }
	}

 	/// [**text_widget()**] - Creates a text widget in the Window. 
   	/// 
   	/// - Returns a <i>TextWidget</i> object where you can control and write to the text widget.
   	/// - The created text widget can be used to create static or dynamic text of any font size in the window.
	/// - Text Widget Objects returned (<i>TextWidget</i> type) does not need to be kept of the text widget will never be updated once the text widget is created.  Otherwise,
	/// the returned <i>TextWidget</i> object can be used to change the text or other aspects of the text widget.
	///
	/// **Function Parameter Forms:**
	///
	///		text_widget(location,title : &str)              // Creates a text widget with a a title
	///		text_widget_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the text widget characteristics
	///
	/// **Some Available Keywords with new_text_widget()**
	/// 
	/// Keyword | Action
	///---- | ----
   	/// text  | - Sets the text of the widget.  This can be set later with <i>text_widget.write()</i>. When text is entered, the text widget is created to the width of the text.  Use the width() parameter to set a width or pad the text with spaces to reserve width.
   	/// text_color  | - Sets the text color of the widget (default is current window text color).  Same as <i>kw::fg_color()</i>
   	/// font       | - Sets the font of the text in the text widget
   	/// text_center  | - Centers the text inside of the widget (which can be longer than the text itself). - Use <i>text_center_x()</i> and <i>center_x()</i> together to make sure text is centered in the window. This is only needed if the Width of the   	///                         - Text Widget and the text have been specificed separately.
   	/// just_center_x (or <i>center_x<i>) | - Centers the widget in horizontally at the Y position provided (as opposed to justifying the text within the text widget).
	/// just_top_center() | - This and other justification keywords can be used to place the widget around the window.
	///
	/// **Notes**
	///
	/// - The text widget is placed at the location provided, unless justification keywords are used
	/// - When justifications are used, 0 may be used in place of the affected values,
	///   - e.g. win.text_widget_s(0,200,"This is a widget",kw::center_x()), which discards the X value provided to center the text widget in the window horizontally, keeping the Y position.
	/// - Use pad_x() and pad_y() to refine the positon of the text widget when using justification-based keywords.
	/// - The returned TextWidget object can be disgregarded if no changes to the widget will occur after the original call. 
	///   - The text widget will be destroyed when the window is closed.
	///
   	/// Examples:
	///
   	///		let widget = win.text_widget("This is a text Widget"); // Create a basic text widget with text.  Return value does not need to be kept.
   	///		let widget = win.text_widget("This is a text Widget",kw::font(15) + kw::fg_color("red")) // Create a text widget with text.  That has a font size of 15 and a text color of red.
	///
	pub fn text_widget(&self,location : (i32,i32), initial_text : &str) -> TextWidget
	{
		self.text_widget_s(location,initial_text,KW_NONE)
	}

	/// [**new_button()**] — Create a button in the window
	/// 
	/// **Function Parameter Forms:**
	///
	///		new_button(location,title : &str)              // Creates a button with the title specified.
	///		new_button_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the button characteristics
	///
	/// - Buttons are automatically sized based on the text title and font (supplied font or default)
	/// - For buttons with short words, e.g. "stop", spaces may be added to make buttons longer, such as "    Stop    ".
	///   - Also see the **<i>width()</>** keyword below
	///
	/// **Some Keywords Available with Buttons**
	///
	/// - size() 	 - Set the size of the button
	/// - width()    - Set the width of the button (height is calculated automatically)
	/// - font() 	 - Set the font used in the button
	/// - hidden()   - Set the button as initially hidden
	/// - disabled() - Set the button as initially disabled
	/// - — There are many more keywords that will be documented in the next release.
	///
	/// **A note about the <i>kw::width()</> keyword**
	/// 
	/// - Using the **<i>kw::width()</>** keyword has a couple great uses:
	///
	///   - When defining multiple buttons, this can create uniform length for buttons, making them look more organized, also allowing lining them up together
	///   - When using the **<i>button.set_text()</>** function to change the text, this can reserve space for the longest text that may later be set in the button.
	///
	/// **[Special Note]**
	///
	/// - Buttons may take a wide variety of shapes and forms, with many keywords to affect their behavior.
	///  - This documentation is still being written and the interface still in-progress.
	/// - More will be added in the next release, with more button types and behavior controls.
	///
	/// **Examples**
	///
	/// 		win.new_button((100,200),"This is a button");                   // Create a button at 100,200
	/// 		win.new_button_s((100,200),"This is a button",kw::font(25));    // Create a button with a font size of 25
	/// 		win.new_button_s((100,200),"Stop",kw::Width(100))				// Create a wider button, since the "stop" title will create a small one if automatic.
	///
	pub fn new_button_s(&self,location : (i32,i32), title : &str,keyw : kw) -> Button
	{
		Button { id : unsafe { ext_func::sage_rust_window_new_button(self.id, location.0, location.1,title.as_ptr(),title.len(),keyw.pointer,false ) }, checkbox : false }
	}

	/// [**new_button()**] — Create a button in the window
	/// 
	/// **Function Parameter Forms:**
	///
	///		new_button(location,title : &str)              // Creates a button with the title specified.
	///		new_button_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the button characteristics
	///
	/// - Buttons are automatically sized based on the text title and font (supplied font or default)
	/// - For buttons with short words, e.g. "stop", spaces may be added to make buttons longer, such as "    Stop    ".
	///   - Also see the **<i>width()</>** keyword below
	///
	/// **Some Keywords Available with Buttons**
	///
	/// - size() 	 - Set the size of the button
	/// - width()    - Set the width of the button (height is calculated automatically)
	/// - font() 	 - Set the font used in the button
	/// - hidden()   - Set the button as initially hidden
	/// - disabled() - Set the button as initially disabled
	/// - — There are many more keywords that will be documented in the next release.
	///
	/// **A note about the <i>kw::width()</> keyword**
	/// 
	/// - Using the **<i>kw::width()</>** keyword has a couple great uses:
	///
	///   - When defining multiple buttons, this can create uniform length for buttons, making them look more organized, also allowing lining them up together
	///   - When using the **<i>button.set_text()</>** function to change the text, this can reserve space for the longest text that may later be set in the button.
	///
	/// **[Special Note]**
	///
	/// - Buttons may take a wide variety of shapes and forms, with many keywords to affect their behavior.
	///  - This documentation is still being written and the interface still in-progress.
	/// - More will be added in the next release, with more button types and behavior controls.
	///
	/// **Examples**
	///
	/// 		win.new_button((100,200),"This is a button");                   // Create a button at 100,200
	/// 		win.new_button_s((100,200),"This is a button",kw::font(25));    // Create a button with a font size of 25
	/// 		win.new_button_s((100,200),"Stop",kw::Width(100))				// Create a wider button, since the "stop" title will create a small one if automatic.
	///
	pub fn new_button(&self,location : (i32,i32), title : &str) -> Button
	{
		self.new_button_s(location,title,KW_NONE)
	}

    /// [**new_checkbox()**] — Create a checkbox in the Window.   
    /// 
	/// - The checkbox is unchecked by default. 
	/// - The input title is the text displayed directly to the right of the square checkbox.
	///
	/// **Function Parameter Forms:**
	///
	///		new_checkbox(location,title : &str)             // Creates a checkbox with the title displayed to the right of the checkbox
	///		new_checkbox_s(location,title : &str,keyw: kw)  // Used when entering keywords to shape checkbox characteristics
	///
	/// **Some Available Keywords with new_checkbox() (partial list)**
	/// 
    /// - **default** - <i>default(true)</i> sets the check in the checkbox.  <i>default(false)</i> leaves the checkbox unchecked.
    /// - **text_color** - Set the text of the checkbox.  This is the same keyword as <i>fg_color()</i>
    /// - **font** - Set the font of the checkbox.
	///
    /// Examples:
	///
    ///		let checkbox = win.new_checkbox((100,200),"This is a checkbox");                           // Create an unchecked checkbox
    ///		let checkbox = win.new_checkbox_s((100,200),"This is a checkbox",kw::default(true));       // Create a checked checkbox
    ///		let checkbox = win.new_checkbox_s((100,200),"This is a checkbox",kw::text_color("green")); // Create a checkbox with green text.
	///
	pub fn new_checkbox_s(&self,location : (i32,i32), initial_text : &str,keyw : kw) -> Button
	{
		Button { id : unsafe { ext_func::sage_rust_window_new_button(self.id, location.0, location.1,initial_text.as_ptr(),initial_text.len(),keyw.pointer,true ) }, checkbox : true }
	}

    /// [**new_checkbox()**] — Create a checkbox in the Window.   
    /// 
	/// - The checkbox is unchecked by default. 
	/// - The input title is the text displayed directly to the right of the square checkbox.
	///
	/// **Function Parameter Forms:**
	///
	///		new_checkbox(location,title : &str)             // Creates a checkbox with the title displayed to the right of the checkbox
	///		new_checkbox_s(location,title : &str,keyw: kw)  // Used when entering keywords to shape checkbox characteristics
	///
	/// **Some Available Keywords with new_checkbox() (partial list)**
	/// 
    /// - **default** - <i>default(true)</i> sets the check in the checkbox.  <i>default(false)</i> leaves the checkbox unchecked.
    /// - **text_color** - Set the text of the checkbox.  This is the same keyword as <i>fg_color()</i>
    /// - **font** - Set the font of the checkbox.
	///
    /// Examples:
	///
    ///		let checkbox = win.new_checkbox((100,200),"This is a checkbox");                           // Create an unchecked checkbox
    ///		let checkbox = win.new_checkbox_s((100,200),"This is a checkbox",kw::default(true));       // Create a checked checkbox
    ///		let checkbox = win.new_checkbox_s((100,200),"This is a checkbox",kw::text_color("green")); // Create a checkbox with green text.
	///
	pub fn new_checkbox(&self,location : (i32,i32), initial_text : &str) -> Button
	{
		self.new_checkbox_s(location,initial_text,KW_NONE)
	}

    /// [**draw_simple_doc_file()**] — Translates a simple .HTML file and displayed it in a newly opened modal window.
	///
	/// **Function Parameter Forms:**
	///
	///		draw_simple_doc_file(file_path : &str)             // Reads the .PGR file from a file path on disk
	///		draw_simple_doc_file_mem(file_mem : &Vec<u8>)      // Reads the .PGR file located in a vector within memory.
	///
	/// - [Note] — This function is still being defined.
	/// - The main purpose behind **<i>draw_simple_doc_file()</i>** is to display informational and About Windows.
	/// - This function takes a PGR file that contains a simple HTML file and a background image. 
	///   - optional additional images may be included
	/// - A new window is created and the HTML file is displayed on the image backrgound provided.
	/// - An 'Ok' button is placed a the bottom of the window for the user to dismiss the window.
	/// - This window is modal, meaning all other windows are blocked from activity until this window is dmissed by the user.
	///
	/// **Returns**
	///
	/// - **`true`** is returned if the file was translated and displayed properly.
	/// - **`false`** is returned if the file was not found, the .PGR file was corrupt, or the HTML file could not be translated.
	///
	pub fn draw_simple_doc_file(&self,file_path : &str) -> bool
	{
		unsafe { let result = ext_func::sage_rust_window_simple_doc_file_draw(self.id,file_path.as_ptr(),file_path.len()); 
		// Error codes to be used later to differentiated between not found, empty string, corrupted file, etc. 

		result == 0		// For now, just return good or bad result, with the overall assumption the .PGR file was not found or was not a PGR file
		 }
	}

    /// [**draw_simple_doc_file()**] — Translates a simple .HTML file and displayed it in a newly opened modal window.
	///
	/// **Function Parameter Forms:**
	///
	///		draw_simple_doc_file(file_path : &str)             // Reads the .PGR file from a file path on disk
	///		draw_simple_doc_file_mem(file_mem : &Vec<u8>)      // Reads the .PGR file located in a vector within memory.
	///
	/// - [Note] — This function is still being defined.
	/// - The main purpose behind **<i>draw_simple_doc_file()</i>** is to display informational and About Windows.
	/// - This function takes a PGR file that contains a simple HTML file and a background image. 
	///   - optional additional images may be included
	/// - A new window is created and the HTML file is displayed on the image backrgound provided.
	/// - An 'Ok' button is placed a the bottom of the window for the user to dismiss the window.
	/// - This window is modal, meaning all other windows are blocked from activity until this window is dmissed by the user.
	///
	/// **Returns**
	///
	/// - **`true`** is returned if the file was translated and displayed properly.
	/// - **`false`** is returned if the file was not found, the .PGR file was corrupt, or the HTML file could not be translated.
	///
	pub fn draw_simple_doc_file_mem(&self,file_mem : &Vec<u8>) -> bool
	{
		unsafe { let result = ext_func::sage_rust_window_simple_doc_filemem_draw(self.id,file_mem.as_ptr(),file_mem.len()); 
		// Error codes to be used later to differentiated between not found, empty string, corrupted file, etc. 

		result == 0		// For now, just return good or bad result, with the overall assumption the .PGR file was not found or was not a PGR file
		 }
	}

    /// [**child_window()**] — Create an embedded child window inside of the parent window
	///
	/// - This function creates a child window that resides within the parent window. 
	/// - The child window is embedded, but can be set as a popup window bound inside the main window (see keyword usage below)
	/// - The child window is borderless, unless the kw::border() keyword is used. 
	///
	/// **Function Parameter Forms:**
	///
	///		child_window(location : (i32,i32), size : (i32,i32))         
	///		child_window_s(location : (i32,i32), size : (i32,i32),keyw : kw)    
	///
	/// **Notes**
	///
	/// - When the window is first created, it inherits the same background color and foreground color of the parent window. 
	///   - If the current backrgound if the main window is a gradient, a middle color is chosen.
	/// - The child window inherits the current Update Type of the parent window. 
	///   - For example, to create a real-time window inside of a default window that does not update, set the update type for the child window with **<i>win.set_auto_update()</i>**.
	///
	/// **Return Value**
	/// 
	/// - **<i>child_window()</i>** returns a Window structure that can be used just the same as any other window.
	///
	/// **Some Useful Keywords**
	///
	/// - **transparent** - Causes the window to initially copy the contents of the parent window in the same location
	///    - The window will appear invisible since it has copied the parent's background
	/// - **hidden** - Creates the window as hidden.  The window can be shown with **<i>win.show()</i>**
	/// - **border()** - Adds a sunken border to the child window, making it appear sunken into the parent window.
	///   - **note:** - Other border types will appear in the interface in an upcoming release. 
	/// - **fg_color()** - Sets the initial foreground color
	/// - **bg_color()** - Sets the initial background color
	/// - **popup()** - Sets the child window as a popup window bound to and inside of the parent window. 
	/// - — <i>More keywords will be documented in an upcoming release.</i>
	///
	/// **Example**
	///
	/// 		let my_child_window = win.child_window((50,50); 	// Create a child window at location (50,50) in the parent window.
	///
	pub fn child_window_s(&self,location : (i32,i32), size : (i32,i32),keyw : kw) -> Window
	{ 
		unsafe { let win_id = ext_func::sage_rust_window_child_window(self.id,location.0,location.1,size.0,size.1,keyw.pointer); 
		Window { id : win_id, gdi : Gdi::new(win_id) } }
	}

   	/// [**child_window()**] — Create an embedded child window inside of the parent window
	///
	/// - This function creates a child window that resides within the parent window. 
	/// - The child window is embedded, but can be set as a popup window bound inside the main window (see keyword usage below)
	/// - The child window is borderless, unless the kw::border() keyword is used. 
	///
	/// **Function Parameter Forms:**
	///
	///		child_window(location : (i32,i32), size : (i32,i32))         
	///		child_window_s(location : (i32,i32), size : (i32,i32),keyw : kw)    
	///
	/// **Notes**
	///
	/// - When the window is first created, it inherits the same background color and foreground color of the parent window. 
	///   - If the current backrgound if the main window is a gradient, a middle color is chosen.
	/// - The child window inherits the current Update Type of the parent window. 
	///   - For example, to create a real-time window inside of a default window that does not update, set the update type for the child window with **<i>win.set_auto_update()</i>**.
	///
	/// **Return Value**
	/// 
	/// - **<i>child_window()</i>** returns a Window structure that can be used just the same as any other window.
	///
	/// **Some Useful Keywords**
	///
	/// - **transparent** - Causes the window to initially copy the contents of the parent window in the same location
	///    - The window will appear invisible since it has copied the parent's background
	/// - **hidden** - Creates the window as hidden.  The window can be shown with **<i>win.show()</i>**
	/// - **border()** - Adds a sunken border to the child window, making it appear sunken into the parent window.
	///   - **note:** - Other border types will appear in the interface in an upcoming release. 
	/// - **fg_color()** - Sets the initial foreground color
	/// - **bg_color()** - Sets the initial background color
	/// - **popup()** - Sets the child window as a popup window bound to and inside of the parent window. 
	/// - — <i>More keywords will be documented in an upcoming release.</i>
	///
	/// **Example**
	///
	/// 		let my_child_window = win.child_window((50,50); 	// Create a child window at location (50,50) in the parent window.
	///	
	pub fn child_window(&self,location : (i32,i32), size : (i32,i32)) -> Window { self.child_window_s(location,size,KW_NONE) }

	/// [**child_window()**] — Sets the auto update type for the window
	///
	/// - By default, windows have an update type of OFF, which means the window must update the window when
	///   it wants to ensure the last information written to it is displayed in the window
	///   - Note that many Sagebox functions will update the window when blocking functions are called, such as input routines. 
	///
	/// **Parameters**
	///
	/// - **`update_type`** - a value of **<i>AutoUpdateType</i>** with the window auto update type. 
	///
	/// [Note]
	///
	/// - Window updates (i.e. the window bitmap physically drawn to the window on the screen) are only performed if there have been graphics, text, or other data output to the window since the last udpate.
	///   - Otherwise, automatic updates do not occur. In this case, calling the **<i>update()</i>** function will simply return without updating the window.
	///
	/// **AutoUpdate Values**
	///
	/// - **Off** - **<i>win.update()</i>** must be used to update the window.
	/// - **On** - Updates every 10-20 millisconds when any Sagebox output, **<i>get_event()</i>** or other window-related function is called.
	///         - In most cases, this will update the window without the need for calling **<i>update()</i>**
	///         - After drawing sessions, if **<i>get_event()</i>** or another Sagebox is not called, use **<i>update()</i>** to make sure the window is updated.
	/// 	    - In general, use an extra **<i>update()</i>** at the end of all drawing and other output routines to make sure it is updated.
	///      - This mode is useful for general output to manage the window output. 
	///         - With drawing routines in loops or real-time routines, this may cause a flicker, as it will update
	///           within the loop, in the middle of a drawing routine.
	/// - **OnTime** - Updates the screen every 10-20 milliseconds regularly as new data is output to the window
	///          - As with all update modes, the window is only updated if there is new data in the window that has not been previously displayed.
	///          - When the OnTime mode is active, the program never needs to update the window — it is performed automatically.
	/// - **Immediate** - Updates after any action that outputs to the screen, no matter how small. 
	///            - note: **Immediate Mode**  can slow down your program.  Useful for diagnostics, status windows or other places where you don't want to do an **<i>update()</i>**, but 
	///              also don't care about it performing an **<i>update()</i>** at every step
	///
	/// **Notes**
	///
	/// - See the **<i>set_realtime()</i>** function to set the window as a window that uses real-time graphics
	///   - This is usually set when the window is created with the **<i>kw::realtime()</i>** keyword. 
	///
	pub fn set_auto_update(&self,update_type : AutoUpdateType) { unsafe { ext_func::sage_rust_window_set_auto_update_type(self.id,update_type as i32); } }

	/// [**set_drag_window()**] — Allows a Window to be dragged around by pressing the mouse anywhere in the window, not just the title bar area
	///
	/// - By default, the window can only be moved around the screen by grabbing the small title-bar area at the top of the window.
	/// - **<i>set_drag_window()</i>** allows the user to click the mouse anywhere in the window and drag it and place it around the screen.
	/// - This can be useful for windows with few or no controls, such as windows displaying bitmaps or mostly data.
	/// - Note: The window must be grabbed in areas that are part of the top-level window itself
	///   - grabbing the window where controls or child windows exist will not work. 
	///
	/// **Notes**
	/// 
	/// - Use **<i>set_drag_window(false)</i>** to turn off the ability to drag the window (i.e. default behavior)
	///
	pub fn set_drag_window(&self,allow_drag : bool) { unsafe { ext_func::sage_rust_window_generic_bool(self.id,0,allow_drag) } }
}
impl Clone for Window {
    fn clone(&self) -> Window {
	//	println!("cloned dial widget");
	//	unsafe { ext_func::rust_sage_widget_dial_clone(self.id); }
	unsafe { Window { id: ext_func::sage_rust_window_clone(self.id), gdi : self.gdi.clone() } }
        
    }
}
impl SageWinPairTypei32<(i32,i32)> for Window
{
	fn get_window_bitmap_s(&self,upper_left : (i32,i32), size : (i32,i32)) -> Bitmap { self._get_window_bitmap_s(upper_left,size) }
	fn set_write_pos(&self,write_pos : (i32,i32)) { self._set_write_pos(write_pos); }
}

impl SageWinPairTypei32<Point<i32>> for Window
{
	fn get_window_bitmap_s(&self,upper_left : Point<i32>, size : Point<i32>) -> Bitmap { self._get_window_bitmap_s(upper_left.i32(),size.i32()) }
	fn set_write_pos(&self,write_pos : Point<i32>) { self._set_write_pos(write_pos.i32()); }
}

impl WindowFontInputType<i32> for Window
{
	fn set_font(&self,font_type : i32) { self._set_font_i32(font_type); }
}

impl WindowFontInputType<&str> for Window
{
	fn set_font(&self,font_type : &str) { self._set_font_str(font_type); }
}
impl WindowFontInputType<&String> for Window
{
	fn set_font(&self,font_type : &String) { self._set_font_str(font_type.as_str()); }
}

pub struct QuickForm
{
	id : i64,
	pub win : Window,
	pub dev_win : DevWindow
}

impl QuickForm
{
 	pub fn new( _id : i64, _win : Window, _dev_win : DevWindow) -> QuickForm
	{
		QuickForm { id: _id, win: _win, dev_win : _dev_win }
	}

	pub const fn default() -> QuickForm
	{
		QuickForm { id: 0, win: Window::default(), dev_win : DevWindow::default() }
	}

	/// [**get_window()**] — Returns a clone of the main window (right-side window) in the quick form. 
	///
	/// - This can be stored for easier access to the quick form window directoy.
	/// - In multi-facted programs, this can be saved as a generic window so that the
	///**quick_form** aspect of the window is abstracted away to a general Window-type window.
	///
	pub fn get_window(&self) -> Window
	{
		let win : i64;
		unsafe { win = ext_func::rust_quickform_get_window(self.id); }
		Window::new(win)
	}	

	/// [**get_dev_window()**] — Returns a clone of the dev window (left-side window) in the quick form. 
	///
	/// - This can be stored for easier access to the quick form dev window directoy.
	/// - In multi-facted programs, this can be saved as a generic dev window so that the
	///**quick_form** aspect of the dev window is abstracted away to a general DevWindow object.
	///
	pub fn get_dev_window(&self) -> DevWindow
	{
		let dev_win : i64;
		unsafe { dev_win = ext_func::rust_quickform_get_dev_window(self.id); }
		DevWindow::new(dev_win)
	}	
}


//pub fn draw2_triangle_s(&self,p1  : T , p2 : T, p3 : T,color : RgbColor,keyw : kw) -> bool;

// ******** draw_triangle **********

impl SageDrawFuncTypeTriangle<(f32,f32),RgbColor> for Window
{ 
	fn draw_triangle_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor,keyw : kw) -> bool { self._draw_triangle_s(p1,p2,p3,color,keyw) }
	fn draw_triangle(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor) -> bool { self._draw_triangle(p1,p2,p3,color) }
	fn fill_triangle_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor,keyw : kw) -> bool { self._fill_triangle_s(p1,p2,p3,color,keyw) }
	fn fill_triangle(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : RgbColor) -> bool { self._fill_triangle(p1,p2,p3,color) }	
}

impl SageDrawFuncTypeTriangle<(f32,f32),&str> for Window
{ 
	fn draw_triangle_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str,keyw : kw) -> bool { self._draw_triangle_str_s(p1,p2,p3,color,keyw) }
	fn draw_triangle(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str) -> bool { self._draw_triangle_str(p1,p2,p3,color) }
	fn fill_triangle_s(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str,keyw : kw) -> bool { self._fill_triangle_str_s(p1,p2,p3,color,keyw) }
	fn fill_triangle(&self,p1  : (f32,f32) , p2 : (f32,f32), p3 : (f32,f32),color : &str) -> bool { self._fill_triangle_str(p1,p2,p3,color) }
}

impl SageDrawFuncTypeTriangle<Point<f32>,RgbColor> for Window
{ 
	fn draw_triangle_s(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : RgbColor,keyw : kw) -> bool 
		{ self._draw_triangle_s((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color,keyw) }
		
	fn draw_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : RgbColor) -> bool 
	{ self._draw_triangle((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color) }

	fn fill_triangle_s(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : RgbColor,keyw : kw) -> bool 
		{ self._fill_triangle_s((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color,keyw) }
		
	fn fill_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : RgbColor) -> bool 
	{ self._fill_triangle((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color) }

}

impl SageDrawFuncTypeTriangle<Point<f32>,&str> for Window
{ 
	fn draw_triangle_s(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : &str,keyw : kw) -> bool 
		{ self._draw_triangle_str_s((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color,keyw) }
		
	fn draw_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : &str) -> bool 
	{ self._draw_triangle_str((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color) }

	fn fill_triangle_s(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : &str,keyw : kw) -> bool 
		{ self._fill_triangle_str_s((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color,keyw) }
		
	fn fill_triangle(&self,p1  : Point<f32> , p2 : Point<f32>, p3 : Point<f32>,color : &str) -> bool 
	{ self._fill_triangle_str((p1.x,p1.y) ,(p2.x,p2.y), (p3.x,p3.y),color) }

}

impl SageDrawFuncTypeTriangle<Point<i32>,RgbColor> for Window
{ 
	fn draw_triangle_s(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : RgbColor,keyw : kw) -> bool 
		{ self._draw_triangle_s((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color,keyw) }
		
	fn draw_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : RgbColor) -> bool 
	{ self._draw_triangle((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color) }

	fn fill_triangle_s(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : RgbColor,keyw : kw) -> bool 
		{ self._fill_triangle_s((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color,keyw) }
		
	fn fill_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : RgbColor) -> bool 
	{ self._fill_triangle((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color) }

}

impl SageDrawFuncTypeTriangle<Point<i32>,&str> for Window
{ 
	fn draw_triangle_s(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : &str,keyw : kw) -> bool 
		{ self._draw_triangle_str_s((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color,keyw) }
		
	fn draw_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : &str) -> bool 
	{ self._draw_triangle_str((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color) }

	fn fill_triangle_s(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : &str,keyw : kw) -> bool 
		{ self._fill_triangle_str_s((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color,keyw) }
		
	fn fill_triangle(&self,p1  : Point<i32> , p2 : Point<i32>, p3 : Point<i32>,color : &str) -> bool 
	{ self._fill_triangle_str((p1.x as f32,p1.y as f32) ,(p2.x as f32,p2.y as f32), (p3.x as f32,p3.y as f32),color) }

}

// ** write, writeln

	impl SageDrawFuncWrite<&str> for Window
	{
		fn write(&self,msg : &str) -> bool { self._write(msg) }
		fn write_s(&self,msg : &str,keyw : kw ) -> bool { self._write_s(msg,keyw) }

		fn writeln(&self,msg : &str) -> bool { self._writeln(msg) }
		fn writeln_s(&self,msg : &str,keyw : kw ) -> bool { self._writeln_s(msg,keyw) }

		fn write_xy(&self,location : (i32,i32), msg : &str) -> bool { self._write_xy(location,msg) }
		fn write_xy_s(&self,location : (i32,i32), msg : &str,keyw : kw ) -> bool { self._write_xy_s(location,msg,keyw) }
	}

	impl SageDrawFuncWrite<&String> for Window
	{
		fn write(&self,msg : &String) -> bool { self._write_str(msg) }
		fn write_s(&self,msg : &String,keyw : kw ) -> bool { self._write_str_s(msg,keyw) }

		fn writeln(&self,msg : &String) -> bool { self._writeln_str(msg) }
		fn writeln_s(&self,msg : &String,keyw : kw ) -> bool { self._writeln_str_s(msg,keyw) }

		fn write_xy(&self,location : (i32,i32), msg : &String) -> bool { self._write_xy_str(location,msg) }
		fn write_xy_s(&self,location : (i32,i32), msg : &String,keyw : kw ) -> bool { self._write_xy_str_s(location,msg,keyw) }
	}
	impl SageDrawFuncWrite<String> for Window
	{
		fn write(&self,msg : String) -> bool { self._write_str(&msg) }
		fn write_s(&self,msg : String,keyw : kw ) -> bool { self._write_str_s(&msg,keyw) }

		fn writeln(&self,msg : String) -> bool { self._writeln_str(&msg) }
		fn writeln_s(&self,msg : String,keyw : kw ) -> bool { self._writeln_str_s(&msg,keyw) }
	
		fn write_xy(&self,location : (i32,i32), msg : String) -> bool { self._write_xy_str(location,&msg) }
		fn write_xy_s(&self,location : (i32,i32), msg : String,keyw : kw ) -> bool { self._write_xy_str_s(location,&msg,keyw) }
	}
// ** draw_pixel

	impl SageDrawFuncSetPixelRgb<RgbColor> for Window
	{
		fn draw_pixel_rgb(&self,x : i32, y: i32,color : RgbColor) -> bool { self._draw_pixel_rgb(x,y,color) }
		fn set_pixel_rgb(&self,x : i32, y: i32,color : RgbColor) -> bool { self._draw_pixel_rgb(x,y,color) }
	}

	impl SageDrawFuncSetPixelRgb<(i32,i32,i32)> for Window
	{
		fn draw_pixel_rgb(&self,x : i32, y: i32,color : (i32,i32,i32)) -> bool { self._draw_pixel_rgbi(x,y,color) }
		fn set_pixel_rgb(&self,x : i32, y: i32,color : (i32,i32,i32)) -> bool { self._draw_pixel_rgbi(x,y,color) }
	}


// *** Cls ****


impl SageDrawFuncCls<RgbColor> for Window
{
	fn cls_s(&self,color : RgbColor) -> bool { self._cls_s(color)  }
	fn cls_grad(&self,color : RgbColor,color2 : RgbColor) -> bool { self._cls_grad(color,color2) }
	fn set_fg_color(&self,color : RgbColor) { self._set_fg_color(color); }
	fn set_bg_color(&self,color : RgbColor) { self._set_bg_color(color); }
}

impl SageDrawFuncCls<&str> for Window
{
	fn cls_s(&self,color : &str) -> bool { self._cls_str(color)  }
	#[allow(unused_variables)]
	fn cls_grad(&self,color : &str,color2 : &str) -> bool { println!("\n-- >cls_grad_str() not yet implemented."); false }
	fn set_fg_color(&self,color : &str) { self._set_fg_color_str(color); }
	fn set_bg_color(&self,color : &str) { self._set_bg_color_str(color); }
}
impl SageDrawFuncCls<&String> for Window
{
	fn cls_s(&self,color : &String) -> bool { self._cls_str(color.as_str())  }
	#[allow(unused_variables)]
	fn cls_grad(&self,color : &String,color2 : &String) -> bool { println!("\n-- >cls_grad_str() not yet implemented."); false }
	fn set_fg_color(&self,color : &String) { self._set_fg_color_str(color.as_str()); }
	fn set_bg_color(&self,color : &String) { self._set_bg_color_str(color.as_str()); }
}
impl SageDrawFuncCls<(i32,i32,i32)> for Window
{
	fn cls_s(&self,color : (i32,i32,i32)) -> bool { self._cls_s(RgbColor::new(color.0,color.1,color.2))  }
	#[allow(unused_variables)]
	fn cls_grad(&self,color : (i32,i32,i32), color2 : (i32,i32,i32)) -> bool 
		{ self._cls_grad(RgbColor::new(color.0,color.1,color.2),RgbColor::new(color2.0, color2.1,color2.2))  }

	fn set_fg_color(&self,color : (i32,i32,i32)) { self._set_fg_color(RgbColor::new(color.0, color.1,color.2)); }
	fn set_bg_color(&self,color : (i32,i32,i32)) { self._set_bg_color(RgbColor::new(color.0, color.1,color.2)); }

}

/// ppp


impl SageDrawFuncClsRadial<RgbColor> for Window { fn cls_radial(&self,color : RgbColor,color2 : RgbColor) -> bool { self._cls_radial(color,color2) } }
impl SageDrawFuncClsRadial<(i32,i32,i32)> for Window { fn cls_radial(&self,color : (i32,i32,i32),color2 : (i32,i32,i32)) -> bool { self._cls_radial(RgbColor::fromi32(color),RgbColor::fromi32(color2)) } }
//impl SageDrawFuncClsRadial<&str>for Window	    { fn cls_radial(&self,color : &str) { self._cls_str(color); } }
//impl SageDrawFuncClsRadial<&String>for Window	    { fn cls_radial(&self,color : &String) { self._cls_str(color.as_str()); } }
//impl SageDrawFuncClsRadial<(i32,i32,i32)> for Window { }


impl SageDrawFuncTypes<(f32,f32),f32,RgbColor> for Window
{
	fn fill_circle(&self,center : (f32,f32), radius : f32,color : RgbColor) -> bool { self._fill_circle(center.0,center.1,radius,color) }
	fn fill_circle_s(&self,center : (f32,f32), radius : f32,color : RgbColor,keyw: kw) -> bool { self._fill_circle_s(center.0,center.1,radius,color,keyw) }
	
	fn draw_circle(&self,center : (f32,f32), radius : f32,color : RgbColor) -> bool { self._draw_circle(center.0,center.1,radius,color) }
	fn draw_circle_s(&self,center : (f32,f32), radius : f32,color : RgbColor,keyw: kw) -> bool { self._draw_circle_s(center.0,center.1,radius,color,keyw) }

	fn draw_line(&self,point1 : (f32,f32),point2 : (f32,f32),color : RgbColor) -> bool { self._draw_line(point1.0,point1.1,point2.0,point2.1,color) }
	fn draw_line_s(&self,point1 : (f32,f32),point2 : (f32,f32),color : RgbColor,keyw: kw) -> bool { self._draw_line_s(point1.0,point1.1,point2.0,point2.1,color,keyw) }

	fn draw_rectangle(&self,location : (f32,f32), size : (f32,f32),color : RgbColor) -> bool { self._draw_rectangle(location,size,color) }
	fn draw_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColor,keyw : kw) -> bool { self._draw_rectangle_s(location,size,color,keyw) }
	
	fn fill_rectangle(&self,location : (f32,f32), size : (f32,f32),color : RgbColor) -> bool { self._fill_rectangle(location,size,color) }
	fn fill_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColor,keyw : kw) -> bool { self._fill_rectangle_s(location,size,color,keyw) }
	
	fn draw_line_to(&self, point1 : (f32,f32), color : RgbColor) -> bool { self._draw_line_to_s(point1.0,point1.1,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : (f32,f32), color : RgbColor,keyw : kw) -> bool { self._draw_line_to_s(point1.0,point1.1,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : (f32,f32), color : RgbColor) -> bool { self._draw_line_to_ex_s(is_first_point,point.0,point.1,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : (f32,f32), color : RgbColor,keyw : kw) -> bool { self._draw_line_to_ex_s(is_first_point,point.0,point.1,color,keyw) }
}


impl SageDrawFuncTypes<(f32,f32),f32,RgbColorA> for Window
{
	fn fill_circle(&self,center : (f32,f32), radius : f32,color : RgbColorA) -> bool { self._fill_circle_a(center.0,center.1,radius,color) }
	fn fill_circle_s(&self,center : (f32,f32), radius : f32,color : RgbColorA,keyw: kw) -> bool { self._fill_circle_a_s(center.0,center.1,radius,color,keyw) }

	fn draw_circle(&self,center : (f32,f32), radius : f32,color : RgbColorA) -> bool { self._draw_circle_a(center.0,center.1,radius,color) }
	fn draw_circle_s(&self,center : (f32,f32), radius : f32,color : RgbColorA,keyw: kw) -> bool { self._draw_circle_a_s(center.0,center.1,radius,color,keyw) }

	fn draw_line(&self,point1 : (f32,f32),point2 : (f32,f32),color : RgbColorA) -> bool { self._draw_line_a(point1.0,point1.1,point2.0,point2.1,color) }
	fn draw_line_s(&self,point1 : (f32,f32),point2 : (f32,f32),color : RgbColorA,keyw: kw) -> bool { self._draw_line_a_s(point1.0,point1.1,point2.0,point2.1,color,keyw) }

	fn draw_rectangle(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA) -> bool { self._draw_rectangle_a(location,size,color) }
	fn draw_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA,keyw : kw) -> bool { self._draw_rectangle_a_s(location,size,color,keyw) }
	
	fn fill_rectangle(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA) -> bool { self._fill_rectangle_a(location,size,color) }
	fn fill_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : RgbColorA,keyw : kw) -> bool { self._fill_rectangle_a_s(location,size,color,keyw) }
	
	fn draw_line_to(&self, point1 : (f32,f32), color : RgbColorA) -> bool { self._draw_line_to_a_s(point1.0,point1.1,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : (f32,f32), color : RgbColorA,keyw : kw) -> bool { self._draw_line_to_a_s(point1.0,point1.1,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : (f32,f32), color : RgbColorA) -> bool { self._draw_line_to_ex_a_s(is_first_point,point.0,point.1,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : (f32,f32), color : RgbColorA,keyw : kw) -> bool { self._draw_line_to_ex_a_s(is_first_point,point.0,point.1,color,keyw) }
}

impl SageDrawFuncTypes<(i32,i32),i32,RgbColor> for Window
{
	fn fill_circle(&self,center : (i32,i32), radius : i32,color : RgbColor) -> bool { self._fill_circle(center.0 as f32,center.1 as f32,radius as f32,color) }
	fn fill_circle_s(&self,center : (i32,i32), radius : i32,color : RgbColor,keyw: kw) -> bool { self._fill_circle_s(center.0 as f32,center.1 as f32,radius as f32,color,keyw) }

	fn draw_circle(&self,center : (i32,i32), radius : i32,color : RgbColor) -> bool { self._draw_circle(center.0 as f32,center.1 as f32,radius as f32,color) }
	fn draw_circle_s(&self,center : (i32,i32), radius : i32,color : RgbColor,keyw: kw) -> bool { self._draw_circle_s(center.0 as f32,center.1 as f32,radius as f32,color,keyw) }

	fn draw_line(&self,point1 : (i32,i32),point2 : (i32,i32),color : RgbColor) -> bool { self._draw_line(point1.0 as f32,point1.1 as f32,point2.0 as f32,point2.1 as f32,color) }
	fn draw_line_s(&self,point1 : (i32,i32),point2 : (i32,i32),color : RgbColor,keyw: kw) -> bool { self._draw_line_s(point1.0 as f32,point1.1 as f32,point2.0 as f32,point2.1 as f32,color,keyw) }

	fn draw_rectangle(&self,location : (i32,i32), size : (i32,i32),color : RgbColor) -> bool { self._draw_rectangle((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color) }
	fn draw_rectangle_s(&self,location : (i32,i32), size : (i32,i32),color : RgbColor,keyw : kw) -> bool { self._draw_rectangle_s((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color,keyw) }
	
	fn fill_rectangle(&self,location : (i32,i32), size : (i32,i32),color : RgbColor) -> bool { self._fill_rectangle((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color) }
	fn fill_rectangle_s(&self,location : (i32,i32), size : (i32,i32),color : RgbColor,keyw : kw) -> bool { self._fill_rectangle_s((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color,keyw) }
	
	fn draw_line_to(&self, point1 : (i32,i32), color : RgbColor) -> bool { self._draw_line_to_s(point1.0 as f32,point1.1 as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : (i32,i32), color : RgbColor,keyw : kw) -> bool { self._draw_line_to_s(point1.0 as f32,point1.1 as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : (i32,i32), color : RgbColor) -> bool { self._draw_line_to_ex_s(is_first_point,point.0 as f32,point.1 as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : (i32,i32), color : RgbColor,keyw : kw) -> bool { self._draw_line_to_ex_s(is_first_point,point.0 as f32,point.1 as f32,color,keyw) }
}

impl SageDrawFuncTypes<(f32,f32),f32,&str> for Window
{
	fn fill_circle(&self,center : (f32,f32), radius : f32,color : &str) -> bool { self._fill_circle_str(center.0,center.1,radius,color) }
	fn fill_circle_s(&self,center : (f32,f32), radius : f32,color : &str,keyw: kw) -> bool { self._fill_circle_str_s(center.0,center.1,radius,color,keyw) }
	
	fn draw_circle(&self,center : (f32,f32), radius : f32,color : &str) -> bool { self._draw_circle_str(center.0,center.1,radius,color) }
	fn draw_circle_s(&self,center : (f32,f32), radius : f32,color : &str,keyw: kw) -> bool { self._draw_circle_str_s(center.0,center.1,radius,color,keyw) }
	
	fn draw_line(&self,point1 : (f32,f32),point2 : (f32,f32),color : &str) -> bool { self._draw_line_str(point1.0,point1.1,point2.0,point2.1,color) }
	fn draw_line_s(&self,point1 : (f32,f32),point2 : (f32,f32),color : &str,keyw: kw) -> bool { self._draw_line_str_s(point1.0,point1.1,point2.0,point2.1,color,keyw) }

	fn draw_rectangle(&self,location : (f32,f32), size : (f32,f32),color : &str) -> bool { self._draw_rectangle_str(location,size,color) }
	fn draw_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : &str,keyw : kw) -> bool { self._draw_rectangle_str_s(location,size,color,keyw) }
	
	fn fill_rectangle(&self,location : (f32,f32), size : (f32,f32),color : &str) -> bool { self._fill_rectangle_str(location,size,color) }
	fn fill_rectangle_s(&self,location : (f32,f32), size : (f32,f32),color : &str,keyw : kw) -> bool { self._fill_rectangle_str_s(location,size,color,keyw) }

	fn draw_line_to(&self, point1 : (f32,f32), color : &str) -> bool { self._draw_line_to_str_s(point1.0,point1.1,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : (f32,f32), color : &str,keyw : kw) -> bool { self._draw_line_to_str_s(point1.0,point1.1,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : (f32,f32), color : &str) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.0,point.1,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : (f32,f32), color : &str,keyw : kw) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.0,point.1,color,keyw) }

}

impl SageDrawFuncTypes<(i32,i32),i32,&str> for Window
{
	fn fill_circle(&self,center : (i32,i32), radius : i32,color : &str) -> bool { self._fill_circle_str(center.0 as f32,center.1 as f32,radius as f32,color) }
	fn fill_circle_s(&self,center : (i32,i32), radius : i32,color : &str,keyw: kw) -> bool { self._fill_circle_str_s(center.0 as f32,center.1 as f32,radius as f32,color,keyw) }

	fn draw_circle(&self,center : (i32,i32), radius : i32,color : &str) -> bool { self._draw_circle_str(center.0 as f32,center.1 as f32,radius as f32,color) }
	fn draw_circle_s(&self,center : (i32,i32), radius : i32,color : &str,keyw: kw) -> bool { self._draw_circle_str_s(center.0 as f32,center.1 as f32,radius as f32,color,keyw) }

	fn draw_line(&self,point1 : (i32,i32),point2 : (i32,i32),color : &str) -> bool { self._draw_line_str(point1.0 as f32,point1.1 as f32,point2.0 as f32,point2.1 as f32,color) }
	fn draw_line_s(&self,point1 : (i32,i32),point2 : (i32,i32),color : &str,keyw: kw) -> bool { self._draw_line_str_s(point1.0 as f32,point1.1 as f32,point2.0 as f32,point2.1 as f32,color,keyw) }


	fn draw_rectangle(&self,location : (i32,i32), size : (i32,i32),color : &str) -> bool { self._draw_rectangle_str((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color) }
	fn draw_rectangle_s(&self,location : (i32,i32), size : (i32,i32),color : &str,keyw : kw) -> bool { self._draw_rectangle_str_s((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color,keyw) }
	
	fn fill_rectangle(&self,location : (i32,i32), size : (i32,i32),color : &str) -> bool { self._fill_rectangle_str((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color) }
	fn fill_rectangle_s(&self,location : (i32,i32), size : (i32,i32),color : &str,keyw : kw) -> bool { self._fill_rectangle_str_s((location.0 as f32,location.1 as f32),(size.0 as f32,size.1 as f32),color,keyw) }


	fn draw_line_to(&self, point1 : (i32,i32), color : &str) -> bool { self._draw_line_to_str_s(point1.0 as f32,point1.1 as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : (i32,i32), color : &str,keyw : kw) -> bool { self._draw_line_to_str_s(point1.0 as f32,point1.1 as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : (i32,i32), color : &str) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.0 as f32,point.1 as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : (i32,i32), color : &str,keyw : kw) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.0 as f32,point.1 as f32,color,keyw) }


}

// Point Types (Point<i32> and Point<f32>)

impl SageDrawFuncTypes<Point<f32>,f32,RgbColor> for Window
{
	fn fill_circle(&self,center : Point<f32>, radius : f32,color : RgbColor) -> bool { self._fill_circle(center.x,center.y,radius,color) }
	fn fill_circle_s(&self,center : Point<f32>, radius : f32,color : RgbColor,keyw: kw) -> bool { self._fill_circle_s(center.x,center.y,radius,color,keyw) }

	fn draw_circle(&self,center : Point<f32>, radius : f32,color : RgbColor) -> bool { self._draw_circle(center.x,center.y,radius,color) }
	fn draw_circle_s(&self,center : Point<f32>, radius : f32,color : RgbColor,keyw: kw) -> bool { self._draw_circle_s(center.x,center.y,radius,color,keyw) }

	fn draw_line(&self,point1 : Point<f32>,point2 : Point<f32>,color : RgbColor) -> bool { self._draw_line(point1.x,point1.y,point2.x,point2.y,color) }
	fn draw_line_s(&self,point1 : Point<f32>,point2 : Point<f32>,color : RgbColor,keyw: kw) -> bool { self._draw_line_s(point1.x,point1.y,point2.x,point2.y,color,keyw) }

	fn draw_rectangle(&self,location : Point<f32>, size : Point<f32>,color : RgbColor) -> bool { self._draw_rectangle(location.f32(),size.f32(),color) }
	fn draw_rectangle_s(&self,location : Point<f32>, size : Point<f32>,color : RgbColor,keyw : kw) -> bool { self._draw_rectangle_s(location.f32(),size.f32(),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<f32>, size : Point<f32>,color : RgbColor) -> bool { self._fill_rectangle(location.f32(),size.f32(),color) }
	fn fill_rectangle_s(&self,location :Point<f32>, size : Point<f32>,color : RgbColor,keyw : kw) -> bool { self._fill_rectangle_s(location.f32(),size.f32(),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<f32>, color : RgbColor) -> bool { self._draw_line_to_s(point1.x,point1.y,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<f32>, color : RgbColor,keyw : kw) -> bool { self._draw_line_to_s(point1.x,point1.y,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<f32>, color : RgbColor) -> bool { self._draw_line_to_ex_s(is_first_point,point.x,point.y,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<f32>, color : RgbColor,keyw : kw) -> bool { self._draw_line_to_ex_s(is_first_point,point.x,point.y,color,keyw) }
}

impl SageDrawFuncTypes<Point<f32>,f32,RgbColorA> for Window
{
	fn fill_circle(&self,center : Point<f32>, radius : f32,color : RgbColorA) -> bool { self._fill_circle_a(center.x,center.y,radius,color) }
	fn fill_circle_s(&self,center : Point<f32>, radius : f32,color : RgbColorA,keyw: kw) -> bool { self._fill_circle_a_s(center.x,center.y,radius,color,keyw) }

	fn draw_circle(&self,center : Point<f32>, radius : f32,color : RgbColorA) -> bool { self._draw_circle_a(center.x,center.y,radius,color) }
	fn draw_circle_s(&self,center : Point<f32>, radius : f32,color : RgbColorA,keyw: kw) -> bool { self._draw_circle_a_s(center.x,center.y,radius,color,keyw) }

	fn draw_line(&self,point1 : Point<f32>,point2 : Point<f32>,color : RgbColorA) -> bool { self._draw_line_a(point1.x,point1.y,point2.x,point2.y,color) }
	fn draw_line_s(&self,point1 : Point<f32>,point2 : Point<f32>,color : RgbColorA,keyw: kw) -> bool { self._draw_line_a_s(point1.x,point1.y,point2.x,point2.y,color,keyw) }

	fn draw_rectangle(&self,location : Point<f32>, size : Point<f32>,color : RgbColorA) -> bool { self._draw_rectangle_a(location.f32(),size.f32(),color) }
	fn draw_rectangle_s(&self,location : Point<f32>, size : Point<f32>,color : RgbColorA,keyw : kw) -> bool { self._draw_rectangle_a_s(location.f32(),size.f32(),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<f32>, size : Point<f32>,color : RgbColorA) -> bool { self._fill_rectangle_a(location.f32(),size.f32(),color) }
	fn fill_rectangle_s(&self,location : Point<f32>, size : Point<f32>,color : RgbColorA,keyw : kw) -> bool { self._fill_rectangle_a_s(location.f32(),size.f32(),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<f32>, color : RgbColorA) -> bool { self._draw_line_to_a_s(point1.x,point1.y,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<f32>, color : RgbColorA,keyw : kw) -> bool { self._draw_line_to_a_s(point1.x,point1.y,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<f32>, color : RgbColorA) -> bool { self._draw_line_to_ex_a_s(is_first_point,point.x,point.y,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<f32>, color : RgbColorA,keyw : kw) -> bool { self._draw_line_to_ex_a_s(is_first_point,point.x,point.y,color,keyw) }
}

impl SageDrawFuncTypes<Point<i32>,i32,RgbColor> for Window
{
	fn fill_circle(&self,center : Point<i32>, radius : i32,color : RgbColor) -> bool { self._fill_circle(center.x as f32,center.y as f32,radius as f32,color) }
	fn fill_circle_s(&self,center : Point<i32>, radius : i32,color : RgbColor,keyw: kw) -> bool { self._fill_circle_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_circle(&self,center : Point<i32>, radius : i32,color : RgbColor) -> bool { self._draw_circle(center.x as f32,center.y as f32,radius as f32,color) }
	fn draw_circle_s(&self,center : Point<i32>, radius : i32,color : RgbColor,keyw: kw) -> bool { self._draw_circle_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_line(&self,point1 : Point<i32>,point2 : Point<i32>,color : RgbColor) -> bool { self._draw_line(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color) }
	fn draw_line_s(&self,point1 : Point<i32>,point2 : Point<i32>,color : RgbColor,keyw: kw) -> bool { self._draw_line_s(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color,keyw) }

	fn draw_rectangle(&self,location : Point<i32>, size : Point<i32>,color : RgbColor) -> bool { self._draw_rectangle((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color) }
	fn draw_rectangle_s(&self,location : Point<i32>, size : Point<i32>,color : RgbColor,keyw : kw) -> bool { self._draw_rectangle_s((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<i32>, size : Point<i32>,color : RgbColor) -> bool { self._fill_rectangle((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color) }
	fn fill_rectangle_s(&self,location : Point<i32>, size : Point<i32>,color : RgbColor,keyw : kw) -> bool { self._fill_rectangle_s((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<i32>, color : RgbColor) -> bool { self._draw_line_to_s(point1.x as f32,point1.y as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<i32>, color : RgbColor,keyw : kw) -> bool { self._draw_line_to_s(point1.x as f32,point1.y as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<i32>, color : RgbColor) -> bool { self._draw_line_to_ex_s(is_first_point,point.x as f32,point.y as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<i32>, color : RgbColor,keyw : kw) -> bool { self._draw_line_to_ex_s(is_first_point,point.x as f32,point.y as f32,color,keyw) }
}

impl SageDrawFuncTypes<Point<f32>,f32,&str> for Window
{
	fn fill_circle(&self,center : Point<f32>, radius : f32,color : &str) -> bool { self._fill_circle_str(center.x,center.y,radius,color) }
	fn fill_circle_s(&self,center : Point<f32>, radius : f32,color : &str,keyw: kw) -> bool { self._fill_circle_str_s(center.x,center.y,radius,color,keyw) }
	
	fn draw_circle(&self,center : Point<f32>, radius : f32,color : &str) -> bool { self._draw_circle_str(center.x,center.y,radius,color) }
	fn draw_circle_s(&self,center : Point<f32>, radius : f32,color : &str,keyw: kw) -> bool { self._draw_circle_str_s(center.x,center.y,radius,color,keyw) }
	
	fn draw_line(&self,point1 : Point<f32>,point2 : Point<f32>,color : &str) -> bool { self._draw_line_str(point1.x,point1.y,point2.x,point2.y,color) }
	fn draw_line_s(&self,point1 : Point<f32>,point2 : Point<f32>,color : &str,keyw: kw) -> bool { self._draw_line_str_s(point1.x,point1.y,point2.x,point2.y,color,keyw) }

	fn draw_rectangle(&self,location : Point<f32>, size : Point<f32>,color : &str) -> bool { self._draw_rectangle_str(location.f32(),size.f32(),color) }
	fn draw_rectangle_s(&self,location : Point<f32>, size : Point<f32>,color : &str,keyw : kw) -> bool { self._draw_rectangle_str_s(location.f32(),size.f32(),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<f32>, size : Point<f32>,color : &str) -> bool { self._fill_rectangle_str(location.f32(),size.f32(),color) }
	fn fill_rectangle_s(&self,location : Point<f32>, size : Point<f32>,color : &str,keyw : kw) -> bool { self._fill_rectangle_str_s(location.f32(),size.f32(),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<f32>, color : &str) -> bool { self._draw_line_to_str_s(point1.x,point1.y,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<f32>, color : &str,keyw : kw) -> bool { self._draw_line_to_str_s(point1.x,point1.y,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<f32>, color : &str) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.x,point.y,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<f32>, color : &str,keyw : kw) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.x,point.y,color,keyw) }
}

impl SageDrawFuncTypes<Point<f64>,f64,RgbColor> for Window
{
	fn fill_circle(&self,center : Point<f64>, radius : f64,color : RgbColor) -> bool { self._fill_circle(center.x as f32,center.y as f32,radius as f32,color) }
	fn fill_circle_s(&self,center : Point<f64>, radius : f64,color : RgbColor,keyw: kw) -> bool { self._fill_circle_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_circle(&self,center : Point<f64>, radius : f64,color : RgbColor) -> bool { self._draw_circle(center.x as f32,center.y as f32,radius as f32,color) }
	fn draw_circle_s(&self,center : Point<f64>, radius : f64,color : RgbColor,keyw: kw) -> bool { self._draw_circle_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_line(&self,point1 : Point<f64>,point2 : Point<f64>,color : RgbColor) -> bool { self._draw_line(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color) }
	fn draw_line_s(&self,point1 : Point<f64>,point2 : Point<f64>,color : RgbColor,keyw: kw) -> bool { self._draw_line_s(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color,keyw) }

	fn draw_rectangle(&self,location : Point<f64>, size : Point<f64>,color : RgbColor) -> bool { self._draw_rectangle(location.f32(),size.f32(),color) }
	fn draw_rectangle_s(&self,location : Point<f64>, size : Point<f64>,color : RgbColor,keyw : kw) -> bool { self._draw_rectangle_s(location.f32(),size.f32(),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<f64>, size : Point<f64>,color : RgbColor) -> bool { self._fill_rectangle(location.f32(),size.f32(),color) }
	fn fill_rectangle_s(&self,location :Point<f64>, size : Point<f64>,color : RgbColor,keyw : kw) -> bool { self._fill_rectangle_s(location.f32(),size.f32(),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<f64>, color : RgbColor) -> bool { self._draw_line_to_s(point1.x as f32,point1.y as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<f64>, color : RgbColor,keyw : kw) -> bool { self._draw_line_to_s(point1.x as f32,point1.y as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<f64>, color : RgbColor) -> bool { self._draw_line_to_ex_s(is_first_point,point.x as f32,point.y as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<f64>, color : RgbColor,keyw : kw) -> bool { self._draw_line_to_ex_s(is_first_point,point.x as f32,point.y as f32,color,keyw) }
}

impl SageDrawFuncTypes<Point<f64>,f64,RgbColorA> for Window
{
	fn fill_circle(&self,center : Point<f64>, radius : f64,color : RgbColorA) -> bool { self._fill_circle_a(center.x as f32,center.y as f32,radius as f32,color) }
	fn fill_circle_s(&self,center : Point<f64>, radius : f64,color : RgbColorA,keyw: kw) -> bool { self._fill_circle_a_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_circle(&self,center : Point<f64>, radius : f64,color : RgbColorA) -> bool { self._draw_circle_a(center.x as f32,center.y as f32,radius as f32,color) }
	fn draw_circle_s(&self,center : Point<f64>, radius : f64,color : RgbColorA,keyw: kw) -> bool { self._draw_circle_a_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_line(&self,point1 : Point<f64>,point2 : Point<f64>,color : RgbColorA) -> bool { self._draw_line_a(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color) }
	fn draw_line_s(&self,point1 : Point<f64>,point2 : Point<f64>,color : RgbColorA,keyw: kw) -> bool { self._draw_line_a_s(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color,keyw) }

	fn draw_rectangle(&self,location : Point<f64>, size : Point<f64>,color : RgbColorA) -> bool { self._draw_rectangle_a(location.f32(),size.f32(),color) }
	fn draw_rectangle_s(&self,location : Point<f64>, size : Point<f64>,color : RgbColorA,keyw : kw) -> bool { self._draw_rectangle_a_s(location.f32(),size.f32(),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<f64>, size : Point<f64>,color : RgbColorA) -> bool { self._fill_rectangle_a(location.f32(),size.f32(),color) }
	fn fill_rectangle_s(&self,location :Point<f64>, size : Point<f64>,color : RgbColorA,keyw : kw) -> bool { self._fill_rectangle_a_s(location.f32(),size.f32(),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<f64>, color : RgbColorA) -> bool { self._draw_line_to_a_s(point1.x as f32,point1.y as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<f64>, color : RgbColorA,keyw : kw) -> bool { self._draw_line_to_a_s(point1.x as f32,point1.y as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<f64>, color : RgbColorA) -> bool { self._draw_line_to_ex_a_s(is_first_point,point.x as f32,point.y as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<f64>, color : RgbColorA,keyw : kw) -> bool { self._draw_line_to_ex_a_s(is_first_point,point.x as f32,point.y as f32,color,keyw) }
}
impl SageDrawFuncTypes<Point<f64>,f64,&str> for Window
{
	fn fill_circle(&self,center : Point<f64>, radius : f64,color : &str) -> bool { self._fill_circle_str(center.x as f32,center.y as f32,radius as f32,color) }
	fn fill_circle_s(&self,center : Point<f64>, radius : f64,color : &str,keyw: kw) -> bool { self._fill_circle_str_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }
	
	fn draw_circle(&self,center : Point<f64>, radius : f64,color : &str) -> bool { self._draw_circle_str(center.x as f32,center.y as f32,radius as f32,color) }
	fn draw_circle_s(&self,center : Point<f64>, radius : f64,color : &str,keyw: kw) -> bool { self._draw_circle_str_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }
	
	fn draw_line(&self,point1 : Point<f64>,point2 : Point<f64>,color : &str) -> bool { self._draw_line_str(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color) }
	fn draw_line_s(&self,point1 : Point<f64>,point2 : Point<f64>,color : &str,keyw: kw) -> bool { self._draw_line_str_s(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color,keyw) }

	fn draw_rectangle(&self,location : Point<f64>, size : Point<f64>,color : &str) -> bool { self._draw_rectangle_str(location.f32(),size.f32(),color) }
	fn draw_rectangle_s(&self,location : Point<f64>, size : Point<f64>,color : &str,keyw : kw) -> bool { self._draw_rectangle_str_s(location.f32(),size.f32(),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<f64>, size : Point<f64>,color : &str) -> bool { self._fill_rectangle_str(location.f32(),size.f32(),color) }
	fn fill_rectangle_s(&self,location : Point<f64>, size : Point<f64>,color : &str,keyw : kw) -> bool { self._fill_rectangle_str_s(location.f32(),size.f32(),color,keyw) }
	
	fn draw_line_to(&self, point1 : Point<f64>, color : &str) -> bool { self._draw_line_to_str_s(point1.x as f32,point1.y as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<f64>, color : &str,keyw : kw) -> bool { self._draw_line_to_str_s(point1.x as f32,point1.y as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<f64>, color : &str) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.x as f32,point.y as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<f64>, color : &str,keyw : kw) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.x as f32,point.y as f32,color,keyw) }
}

impl SageDrawFuncTypes<Point<i32>,i32,&str> for Window
{
	fn fill_circle(&self,center : Point<i32>, radius : i32,color : &str) -> bool { self._fill_circle_str(center.x as f32,center.y as f32,radius as f32,color) }
	fn fill_circle_s(&self,center :Point<i32>, radius : i32,color : &str,keyw: kw) -> bool { self._fill_circle_str_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_circle(&self,center : Point<i32>, radius : i32,color : &str) -> bool { self._draw_circle_str(center.x as f32,center.y as f32,radius as f32,color) }
	fn draw_circle_s(&self,center :Point<i32>, radius : i32,color : &str,keyw: kw) -> bool { self._draw_circle_str_s(center.x as f32,center.y as f32,radius as f32,color,keyw) }

	fn draw_line(&self,point1 : Point<i32>,point2 : Point<i32>,color : &str) -> bool { self._draw_line_str(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color) }
	fn draw_line_s(&self,point1 : Point<i32>,point2 : Point<i32>,color : &str,keyw: kw) -> bool { self._draw_line_str_s(point1.x as f32,point1.y as f32,point2.x as f32,point2.y as f32,color,keyw) }


	fn draw_rectangle(&self,location : Point<i32>, size : Point<i32>,color : &str) -> bool { self._draw_rectangle_str((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color) }
	fn draw_rectangle_s(&self,location : Point<i32>, size : Point<i32>,color : &str,keyw : kw) -> bool { self._draw_rectangle_str_s((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color,keyw) }
	
	fn fill_rectangle(&self,location : Point<i32>, size : Point<i32>,color : &str) -> bool { self._fill_rectangle_str((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color) }
	fn fill_rectangle_s(&self,location : Point<i32>, size :Point<i32>,color : &str,keyw : kw) -> bool { self._fill_rectangle_str_s((location.x as f32,location.y as f32),(size.x as f32,size.y as f32),color,keyw) }

	
	fn draw_line_to(&self, point1 : Point<i32>, color : &str) -> bool { self._draw_line_to_str_s(point1.x as f32,point1.y as f32,color,KW_NONE) }
	fn draw_line_to_s(&self, point1 : Point<i32>, color : &str,keyw : kw) -> bool { self._draw_line_to_str_s(point1.x as f32,point1.y as f32,color,keyw) }

	fn draw_line_to_ex(&self, is_first_point : bool, point : Point<i32>, color : &str) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.x as f32,point.y as f32,color,KW_NONE) }
	fn draw_line_to_ex_s(&self, is_first_point : bool, point : Point<i32>, color : &str,keyw : kw) -> bool { self._draw_line_to_ex_str_s(is_first_point,point.x as f32,point.y as f32,color,keyw) }

}

impl Drop for Window {
    fn drop(&mut self) {

	unsafe { ext_func::sage_rust_window_delete(self.id); }

    }
}


pub struct ComboBox
{
	id : i64,
}

impl ComboBox
{
    pub fn new( _id : i64) -> ComboBox
	{
		ComboBox { id: _id }
	}	
	/// [**default()**] — returns an empty Combox that can be used for initial storage and then replaced with a created ComboBox
	///
	pub const fn default() -> ComboBox
	{
		ComboBox { id: 0 }
	}

	/// [**item_selected()**] — returns true if an item was been selected in the Combobox
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			item_selected()
	///			item_selected_peek()
	///
	/// <u><i>**item_selected_peek()**</i></u>
	///
	/// - When this form is used, this will return the Combo Box item-selected, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**item_selected()**</i> is used without peeking, the Combo Box item-selected status is reset until the next time the user selects another item.
	///
	pub fn item_selected(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_combobox_item_selected(self.id,false); }
		ret_val
	}

	/// [**item_selected()**] — returns true if an item was been selected in the Combobox
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			item_selected()
	///			item_selected_peek()
	///
	/// <u><i>**item_selected_peek()**</i></u>
	///
	/// - When this form is used, this will return the Combo Box item-selected, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**item_selected()**</i> is used without peeking, the Combo Box item-selected status is reset until the next time the user selects another item.
	///
	pub fn item_selected_peek(&self,peek : bool) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_combobox_item_selected(self.id,peek); }
		ret_val
	}
	/// [**get_item_selected()**] — returns the last item index selected in the Combobox. 
	///
	/// - If no item has ever been selected, or there are no items in the Combo Box, this function returns 0 (i.e. first item in the list)
	/// - **<i>See item_selected()</i>** to determine when a user has selected an item from the Combo Box
	/// 
	/// **Example**
	///
	/// 	if my_combo_box.item_selected() { item_index = combo_box.get_item_selected(); } 
	///
	///		// Get item selected, but only if someone selected something and caused an event.
	///
	pub fn get_item_selected(&self) -> i32
	{
		let ret_val : i32;
		unsafe { ret_val = ext_func::sage_rust_combobox_get_item_selected(self.id); }
		ret_val
	}
}

pub struct Button
{
	id : i64,
	checkbox : bool,
}
impl Button
{
    pub fn new( _id : i64, is_checkbox : bool) -> Button
	{
		Button { id: _id, checkbox : is_checkbox }
	}	

	/// [**default()**] — returns an empty Button that can be used for initial storage and then replaced with a created Button
	///
	pub const fn default() -> Button
	{
		Button { id: 0, checkbox : false }
	}	

	/// [**set_tooltip()**] — Set a tooltip that displays over the Button when the mouse is held over the control for about 1 second.
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_tooltip(tooltip : &str)
	///			set_tooltip_str(tooltip : &String)
	///
	pub fn set_tooltip(&self,tooltip : &str) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_checkbox_set_tooltip(tooltip.as_ptr(),tooltip.len(),self.id); }
		ret_val	
	}

	/// [**pressed()**] — returns true if the button was pressed.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			pressed()
	///			pressed_peek()
	///
	/// <u><i>**pressed_peek()**</i></u>
	///
	/// - When this form is used, this will return the Button pressed status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**pressed()**</i> is used without peeking, the Button pressed status status is reset until the next time the user presses the button again.
	///
	pub fn pressed(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_button_pressed(self.id,false); }
		ret_val	
	}

	/// [**pressed()**] — returns true if the button was pressed.
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			pressed()
	///			pressed_peek()
	///
	/// <u><i>**pressed_peek()**</i></u>
	///
	/// - When this form is used, this will return the Button pressed status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**pressed()**</i> is used without peeking, the Button pressed status status is reset until the next time the user presses the button again.
	///
	pub fn pressed_peek(&self,peek : bool) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_button_pressed(self.id,peek); }
		ret_val	
	}

	/// [**pressed()**] — Waits until the button is pressed before continuing program execution
	///
	/// - The program is set to a modal state, where the program execution is paused until the button is pressed.
	///   - The program thread is stopped during this time (i.e. until the button is pressed)
	/// - This is a useful debug, development, or otherwise quick way to stop execution until a button is pressed.
	/// - See: **<i>wait_for_unpress()</i>**, which can be used to wait for the user to unpress the button once it has been pressed, pause and
	///resume program execution only when the button is physically pressed. 
	///
	pub fn wait_for_press(&self)
	{
		unsafe { ext_func::sage_rust_button_wait_for_press(self.id,false); }
	}

	/// [**pressed()**] — Waits until the button is un-pressed (i.e. releases the mouse button while the button is already pressed) before continuing program execution
	///
	/// - The program is set to a modal state, where the program execution is paused until the button is unpressed.
	///   - The program thread is stopped during this time (i.e. until the button is unpressed)
	/// - This is a useful debug, development, or otherwise quick way to stop execution until a button is unpressed.
	/// - See: **<i>wait_for_press()</i>**, which can be used pause and resume program execution by pressing the button once to stop execution, and press it again to resume the program.
	///
	pub fn wait_for_unpress(&self)
	{
		unsafe { ext_func::sage_rust_button_wait_for_press(self.id,true); }
	}

	/// [**if_pressed()**] — Fills a boolean value if the button was pressed.
	///
	/// - This is a shortcut that can be used in place of querying **<i>button_pressed()</i>** and then assigning a value based on whether or not it was pressed.
	/// - This function works with checkboxes and buttons
	/// - The **`fill_value`** is only filled if the button or checkbox as been pressed.
	/// - **`true`** is returned if the button or checkbox was pressed.  **`false`** is returned if the button or checkbox was not pressed.
	/// - For buttons, The value filled will always be filled with **`true`**.
	/// - For checkboxes, the value is filled with the checkbox value — `true` if checked, `false` if unchecked.
	///
	/// **Returns**
	/// 
	/// - **`true`** is returned if the button or checkbox was checked (this is independent of the fill_value)
	/// - **`false`** is returned if the button or checkbox was not checked, and the **`fill_value`** is not touched.
	///
	pub fn if_pressed(&self, fill_value : &mut bool) -> bool
	{
		let return_val = self.pressed();
		if return_val
		{
			if self.checkbox  { *fill_value = self.checked(); }	// For Checkbox
			else { *fill_value = true; }							// For button
		}
		return_val
	}

	/// [**checked()**] — Returns **`true`** if the checkbox is currently checked.  **`false`** if the checkbox is not checked.
	///
	pub fn checked(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_button_checked(self.id); }
		ret_val	
	}

	/// [**enable()**] — Enables or disabled the button or checkbox
	///
	/// - For buttons, when disabled, the button text shows as a gray blending into the button to suggest it is disabled.
	///   - When disabled, the button will not react or move when pressed, and no events will be sent regarding the button.
	/// - For checkboxes, when disabled, the text is displayed in a middle gray to denote that the checkbox is disabled.
	///   - When disabled, the checkbox cannot be checked or unchecked.
	///
	pub fn enable(&self,enable_button : bool)
	{
		unsafe { ext_func::sage_rust_button_enable(self.id,enable_button); }
	}

	/// [**set_tooltip()**] — Set a tooltip that displays over the Button when the mouse is held over the control for about 1 second.
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_tooltip(tooltip : &str)
	///			set_tooltip_str(tooltip : &String)
	///
	pub fn set_tooltip_str(&self,tooltip : &String) -> bool { self.set_tooltip(tooltip.as_str()) }

	/// [**get_location()**] — Returns the location of the button's top-left corner in the parent window.
	///
	/// - This function returns a **`Point<i32>`** value.
	///   - to return an **`(i32,i32)`**, call the return values .i32() function
	///
	/// **Examples**
	///
	///		let location = my_button.get_location();       // Get location as Point<i32>
	///		let location = my_button.get_location().i32(); // Get location as (i32,i32)
	///
	pub fn get_location(&self) -> Point<i32>
	{
		unsafe { let loc = ext_func::sage_rust_button_get_location(self.id); 
		Point::<i32>::new(loc.0,loc.1) }
	}

	/// [**set_text()**] — Sets the text of the button.
	///
	/// - This can be used any time the button is active.
	/// - The width will change if the button does not have a defined width. 
	/// - Use the **<i>kw::width()</i>** keyword when creating the button to create a static width
	///   - This keeps the button size from changing when the **<i>set_text()</i>** function is used.
	///   - Reserve enough width for the longest title string the button will contain.
	///
	/// **Example**
	///
	///		let my_button = Sagebox::dev_button_s("Pause Output",kw::width(140));
	///	
	///		if my_button.pressed()
	///		{
	///			my_button.set_text("Continue Output"); // Change button text until pressed
	/// 		my_button.wait_for_press();            // Wait for button-press to continue
	///			my_button.set_text("Pause Output");    // Put original title back
	///		}
	///
	pub fn set_text(&self,text : &str) { unsafe { ext_func::sage_rust_button_set_text(self.id,text.as_ptr(),text.len()); } }

	/// [**set_style()**] — Sets the style of the button.
	///
	/// - [*** This function is still being implemented in the Rust interface and is incomplete ***]
	/// - **<i>set_style()</i>** allows the button to change styles, such as different colors. 
	///
	/// - This function will be documented in a next release. 
	/// - For now, basic colors can be used to set the background of the button
	///
	/// **Example**
	///
	/// 	my_button.set_style("red"); 	 // Sets the background of the button to red
	/// 	my_button.set_style("default");  // Sets the background of the button to it's default value
	/// 
	pub fn set_style(&self,style : &str) { unsafe { ext_func::sage_rust_button_set_style(self.id,style.as_ptr(),style.len()); } }

}

pub struct ButtonGroup
{
	id : i64,
}

impl ButtonGroup
{
    pub fn new( _id : i64) -> ButtonGroup
	{
		ButtonGroup { id: _id }
	}	

	/// [**default()**] — returns an empty ButtonGroup that can be used for initial storage and then replaced with a created ButtonGroup
	///
	pub const fn default() -> ButtonGroup
	{
		ButtonGroup { id: 0 }
	}
	// $$ This function is to be changed to fit with lifetime of buttons return vs. buttons in the radio buttnon group
	//
	pub fn get_button(&self,button_id : i32) -> Button
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_radio_button_get_button(self.id,button_id); }
		Button::new(ret_val,false)
	}	


	/// [**pressed()**] — Returns true if a button in the button group was pressed.
	///
	/// [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	/// - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			pressed()
	///			pressed_peek(peek : bool)
	///
	/// - When a radio button is pressed (i.e. filled in, removing that status from the currently-selected radio button), this event occurs. 
	/// - See **<i>get_checked_button()</i>** to get the index of the radio button that was pressed. 
	///
	/// <u><i>**pressed_peek()**</i></u>
	///
	/// - When this form is used, this will return the pressed status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**pressed()**</i> is used without peeking, the pressed status is reset until the next time a radio button
	///in the Button Group is pressed.
	/// 
	/// **Example**
	///
	///		if my_button_group_pressed() { button_index = my_button_group.get_checked_button }
	///
	pub fn pressed(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_radio_button_pressed(self.id,false); }
		ret_val	
	}

	/// [**pressed()**] — Returns true if a button in the button group was pressed.
	///
	/// [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	/// - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			pressed()
	///			pressed_peek(peek : bool)
	///
	/// - When a radio button is pressed (i.e. filled in, removing that status from the currently-selected radio button), this event occurs. 
	/// - See **<i>get_checked_button()</i>** to get the index of the radio button that was pressed. 
	///
	/// <u><i>**pressed_peek()**</i></u>
	///
	/// - When this form is used, this will return the pressed status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**pressed()**</i> is used without peeking, the pressed status is reset until the next time a radio button
	///in the Button Group is pressed.
	/// 
	/// **Example**
	///
	///		if my_button_group_pressed() { button_index = my_button_group.get_checked_button }
	///
	pub fn pressed_peek(&self,peek : bool) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_radio_button_pressed(self.id,peek); }
		ret_val	
	}

	/// [**get_checked_button()**] — Returns the currently checked (or filled in) button in a radio button group
	///
	/// - This is not an event-type function.  It can be called any time to get the currently highlighted/filled radio button.
	/// - Usually, this function is called after a **`pressed`** event was detected through the <i>**pressed()**</i> function, a callback, or a signal. 
	///
	/// **Example**
	///
	///		if my_button_group_pressed() { button_index = my_button_group.get_checked_button }
	///
	pub fn get_checked_button(&self) -> i32
	{
		let ret_val : i32;
		unsafe { ret_val = ext_func::sage_rust_radio_button_get_checked_button(self.id); }
		ret_val	
	}

}

pub trait TextWidgetStringType<StringType>
{
	fn write(&self,msg : StringType);
}
pub struct TextWidget
{
	id : i64,
}

impl TextWidget 
{
    pub fn new( _id : i64) -> TextWidget
	{
		TextWidget { id: _id }
	}
	pub const fn default() -> TextWidget
	{
		TextWidget { id: 0 }
	}
	fn _write(&self,message : &str)
	{
		unsafe { ext_func::sage_rust_text_widget_write(self.id,message.as_ptr(),message.len()); }
	}
}

impl TextWidgetStringType<&str> for TextWidget
{
	fn write(&self, message : &str) { self._write(message); }
}
impl TextWidgetStringType<&String> for TextWidget
{
	fn write(&self, message : &String) { self._write(message.as_str()); }
}
impl TextWidgetStringType<String> for TextWidget
{
	fn write(&self, message : String) { self._write(&message); }
}
pub struct Slider
{
	id : i64,
}

impl Slider 
{
    pub fn new( _id : i64) -> Slider
	{
		Slider { id: _id }
	}

	/// [**default()**] — returns an empty Slider that can be used for initial storage and then replaced with a created Slider
	///
	pub const fn default() -> Slider
	{
		Slider { id: 0 }
	}
	/// Returns the current position of the slider based on its range (as an i32 integer).
	/// 
	/// - Use get_pos() for default (integer) sliders, and get_pos_f() for floating-pointer sliders
	/// (floating-point sliders are created with new_slider_f() and dev_slider_f() forms).
	/// - get_pos_f() can be used with regular (integer) sliders and will simply return the position as an f32 value rather
	/// than an i32 integer.
    /// 
	/// The default range is (0-100), but this can be set through the kw::range() option when creating the slider.	
	/// 
	pub fn get_pos(&self) -> i32
	{
		let ret_val : i32;
		unsafe { ret_val = ext_func::sage_rust_slider_get_pos_integer(self.id); }
		ret_val
	}	

	/// Returns the current position of the slider based on its range as a float vlaue  (f32 float).
	/// 
	/// - Use get_pos() for default (integer) sliders, and get_pos_f() for floating-pointer sliders
	/// (floating-point sliders are created with new_slider_f() and dev_slider_f() forms).
	/// - get_pos_f() can be used with regular (integer) sliders and will simply return the position as an f32 value rather
	/// than an i32 integer.
    /// 
	/// The default range for floating-point sliders is (0.0-1.0), but this can be set through the kw::range() option when creating the slider.	
	///
	pub fn get_pos_f(&self) -> f32
	{
		let ret_val : f32;
		unsafe { ret_val = ext_func::sage_rust_slider_get_pos_float(self.id); }
		ret_val
	}	

	/// [**moved()**] — Returns true if a the slider was moved by the user
	///
	/// [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	/// - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Example**
	///
	///		if my_slider.moved { slider_pos = my_slider.get_pos() }
	///
	pub fn moved(&self) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_slider_moved(self.id); }
		ret_val
	}	

	/// [**set_tooltip()**] — Set a tooltip that displays over the Button when the mouse is held over the control for about 1 second.
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_tooltip(tooltip : &str)
	///			set_tooltip_str(tooltip : &String)
	///
	pub fn set_tooltip(&self,tooltip : &str) -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_slider_set_tooltip(tooltip.as_ptr(),tooltip.len(),self.id); }
		ret_val	
	}

	/// [**set_tooltip()**] — Set a tooltip that displays over the Button when the mouse is held over the control for about 1 second.
	///
	/// **Function Parameter Forms:** 
	///	
	///			set_tooltip(tooltip : &str)
	///			set_tooltip_str(tooltip : &String)
	///
	pub fn set_tooltip_str(&self,tooltip : &String) -> bool { self.set_tooltip(tooltip.as_str()) }

}


pub struct InputBox
{
	id : i64,
}

impl InputBox 
{
    pub fn new( _id : i64) -> InputBox
	{
		InputBox { id: _id }
	}

	/// [**default()**] — returns an empty InputBox that can be used for initial storage and then replaced with a created InputBox
	///
	pub fn default() -> InputBox { InputBox { id: 0 } }

	/// [**get_text()**] — Returns the text in the Input Box
	///
	/// - An empty string is returned if there is no text in the Input Box
	///
	pub fn get_text(&self) -> String
	{
		unsafe { 
			let ret = ext_func::sage_rust_inputbox_get_text(self.id);

			if ret.len > 0 { String::from_raw_parts(ret.data,ret.len,ret.len) } else { String::new() }
		} 
	}

	/// [**clear_text()**] — Clears the text in the Input Box
	///
	pub fn clear_text(&self) -> bool
	{
		let ret_val: bool;
		unsafe { ret_val = ext_func::sage_rust_inputbox_clear_text(self.id); };
		ret_val

	}

	/// [**get_i32()**] — Returns an integer translated from the text in the Input Box
	/// 
	/// - If there is no text in the box, 0 is returned.
	/// - If the value is not a number, 0 is returned. 
	///
	/// **note**
	///
	/// - **<i>get_i32()</i>** and **<i>get_integer()</i>** are the same function
	///   - **<i>get_integer()</i>** is provided for compability between platforms
	///
	///	**Example**
	///
	///		let value = my_input_box.get_i32() // If the input box reads "123", 123 will be returned. 
	///                                        // If there is nothing in the Input Box, 0 will be returned.
	///                                        // If there is "abcde" in the Input Box, 0 will be returned.
	///
	pub fn get_i32(&self) -> i32 { unsafe { ext_func::sage_rust_inputbox_get_integer(self.id) } }

	/// [**get_integer()**] — Returns an integer translated from the text in the Input Box
	/// 
	/// - If there is no text in the box, 0 is returned.
	/// - If the value is not a number, 0 is returned. 
	///
	/// **note**
	///
	/// - **<i>get_i32()</i>** and **<i>get_integer()</i>** are the same function
	///   - **<i>get_integer()</i>** is provided for compability between platforms
	///
	///	**Example**
	///
	///		let value = my_input_box.get_integer() // If the input box reads "123", 123 will be returned. 
	///                                            // If there is nothing in the Input Box, 0 will be returned.
	///                                            // If there is "abcde" in the Input Box, 0 will be returned.
	///
	pub fn get_integer(&self) -> i32 { unsafe { ext_func::sage_rust_inputbox_get_integer(self.id) } }

	/// [**get_f32()**] — Returns an f32, floating-point translated from the text in the Input Box
	/// 
	/// - If there is no text in the box, 0.0 is returned.
	/// - If the value is not a number, 0.0 is returned. 
	///
	/// **note**
	///
	/// - **<i>get_f32()</i>** and **<i>get_float()</i>** are the same function
	///   - **<i>get_float()</i>** is provided for compability between platforms
	///
	///	**Example**
	///
	///		let value = my_input_box.get_f32() // If the input box reads "123.567", 123.567 will be returned. 
	///                                        // If there is nothing in the Input Box, 0.0 will be returned.
	///                                        // If there is "abcde" in the Input Box, 0.0 will be returned.
	///	
	pub fn get_f32(&self) -> f32 { unsafe { ext_func::sage_rust_inputbox_get_float(self.id) } }

	/// [**get_float()**] — Returns an f32, floating-point translated from the text in the Input Box
	/// 
	/// - If there is no text in the box, 0.0 is returned.
	/// - If the value is not a number, 0.0 is returned. 
	///
	/// **note**
	///
	/// - **<i>get_f32()</i>** and **<i>get_float()</i>** are the same function
	///   - **<i>get_float()</i>** is provided for compability between platforms
	///
	///	**Example**
	///
	///		let value = my_input_box.get_float() // If the input box reads "123.567", 123.567 will be returned. 
	///                                          // If there is nothing in the Input Box, 0.0 will be returned.
	///                                          // If there is "abcde" in the Input Box, 0.0 will be returned.
	///	
	pub fn get_float(&self) -> f32 { unsafe { ext_func::sage_rust_inputbox_get_float(self.id) } }


	/// [**get_f64()**] — Returns an f64, floating-point translated from the text in the Input Box
	/// 
	/// - If there is no text in the box, 0.0 is returned.
	/// - If the value is not a number, 0.0 is returned. 
	///
	/// **note**
	///
	/// - **<i>get_f64()</i>** and **<i>get_double()</i>** are the same function
	///   - **<i>get_double()</i>** is provided for compability between platforms
	///
	///	**Example**
	///
	///		let value = my_input_box.get_f64() // If the input box reads "123.567", 123.567 will be returned. 
	///                                        // If there is nothing in the Input Box, 0.0 will be returned.
	///                                        // If there is "abcde" in the Input Box, 0.0 will be returned.
	///	
	pub fn get_f64(&self) -> f64{ unsafe { ext_func::sage_rust_inputbox_get_float(self.id) as f64 } }

	/// [**get_double()**] — Returns an f64, floating-point translated from the text in the Input Box
	/// 
	/// - If there is no text in the box, 0.0 is returned.
	/// - If the value is not a number, 0.0 is returned. 
	///
	/// **note**
	///
	/// - **<i>get_f64()</i>** and **<i>get_double()</i>** are the same function
	///   - **<i>get_double()</i>** is provided for compability between platforms
	///
	///	**Example**
	///
	///		let value = my_input_box.get_double() // If the input box reads "123.567", 123.567 will be returned. 
	///                                           // If there is nothing in the Input Box, 0.0 will be returned.
	///                                           // If there is "abcde" in the Input Box, 0.0 will be returned.
	///
	pub fn get_double(&self) -> f64{ unsafe { ext_func::sage_rust_inputbox_get_float(self.id) as f64 } }

	/// [**return_pressed()**] — Returns true if a the user pressed the return key in the input box
	///
	/// [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	/// - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// - This event can be used to determine when to read the input box (since the user has pressed return)
	///
	/// **Example**
	///
	///		if my_input_box.return_pressed() { return_string = my_input_box.get_text(); }
	///		if my_input_box.return_pressed() { return_i32 = my_input_box.get_i32(); }
	///
	pub fn return_pressed(&self) -> bool { unsafe { ext_func::sage_rust_inputbox_return_pressed(self.id) } 	}

}

pub struct ImageView
{
	id : i64, 
	is_ba : bool,	// Is before & after type
}

impl ImageView
{
	/// [**default()**] — returns an empty ImageView that can be used for initial storage and then replaced with a created ImageView
	///
	pub fn default() -> ImageView { ImageView{ id: 0, is_ba : false } }

	/// [**closed()**] — Returns true of the user closed the window
	///
	/// - When the user closes the window, the window is closed by Sagebox.
	/// - it is not required to call close() to complete the window closing. 
	/// - This function is informational in nature — it is not an event function.
	/// - See **<i>close_event()</i>** to react the specific close event created when the user closes the window.
	///
	pub fn closed(&self) -> bool  {  unsafe { ext_func::sage_rust_image_view_closed(self.id,self.is_ba) } }

	/// [**window_count()**] — Returns the number of Image View windows are currently open (either single-view or before-and-after-view)
	///
	/// **notes**
	///
	/// - This is a static function, accessed by calling **<i>ImageView::window_count()</i>** 
	///
	pub fn window_count() -> i32  {  unsafe { ext_func::sage_rust_image_view_window_count()  }}

	/// [**close_event()**] — Returns true if the user closed the window
	///
	/// [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	/// - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	pub fn close_event(&self) -> bool  { unsafe { ext_func::sage_rust_image_view_close_event(self.id,false,self.is_ba) }}

	/// [**wait_for_close()**] — Waits for the user to close the window
	///
	/// - This function stops program execution, waiting for the user to close the specific window attached to the calling object.
	/// - see **<i>wait_for_close_all()</i>** to wait for all Image View Windows to close
	/// - see **<i>wait_for_close_any()</i>** to wait for any Image View Window to close.
	///
	pub fn wait_for_close(&mut self) { unsafe { ext_func::sage_rust_image_view_wait_close(self.id, 0,self.is_ba ); } }


	/// [**wait_for_close_all()**] — Waits for the user to close all Image View windows
	///
	/// - This function stops program execution, waiting for the user to close all Image View Windows before continuing.
	/// - see **<i>wait_for_close()</i>** to wait for a specific Image View window to close
	/// - see **<i>wait_for_close_any()</i>** to wait for any Image View Window to close.
	///
	pub fn wait_for_close_all() { unsafe { ext_func::sage_rust_image_view_wait_close(0, 1, false); } }

	/// [**wait_for_close_any()**] — Waits for the user to any open Image View Window
	///
	/// - This function stops program execution, waiting for the user to close any single Image View Window before continuing.
	/// - see **<i>wait_for_close()</i>** to wait for a specific Image View window to close
	/// - see **<i>wait_for_close_all()</i>** to wait for all Image View Windows to close
	///
	pub fn wait_for_close_any() {  unsafe { ext_func::sage_rust_image_view_wait_close(0, 2, false); } } 

	/// [**close_all()**] — Closes all Image View Windows currently open
	///
	/// - This function closes the windows, and marks them as closed
	/// - It does not affect any Image View object that is in memory, which may go through the normal Drop process.
	///
	pub fn close_all() {  unsafe { ext_func::sage_rust_image_view_perform_task(0, 0, false ); } }


	/// [**show_instructions()**] — Opens a window with instructions on using the Image View Window and waits for the user to press an OK button
	///
	/// - The Image View windows can do a lot, either for just one window or multiple windows.
	///	  - Windows can be zoomed in and out, the contents moved around,
	///   - A Zoom Box/Navigator window can be brought up to help navigate through the image, 
	///   - The Zoom Box adjust to switching between open Image View Windows
	/// - This function brings up an informational window with an OK button at the bottom.
	/// - This function <u><i>does not stop program flow</i></u>, and just opens a window that dismisses when the OK button is pressed
	/// - The information window can be useful for users not familiar with the program.
	/// - The information window can also be brought up in the system menu of the window (the '-' in the window in the upper-left)
	///
	pub fn show_instructions() {  unsafe { ext_func::sage_rust_image_view_perform_task(0, 1, false ); } }

	/// [**close_window()**] — Closes the Image View window associated with the Image View (i.e. &self) object.
	///
	/// - This will close and remove the window from the screen. 
	///
	pub fn close_window(self) { unsafe { ext_func::sage_rust_image_view_perform_task(self.id, 2,self.is_ba ); } }

	/// [**is_valid()**] — Returns true of the Image View window is successfully displayed (or was, if it was closed) to the screen.
	///
	/// - If the Image View window was not displayed due to an error, **<i>is_valid()</i>** will return false.
	/// - **<i>is_valid()</i>** can be a good check to make sure the image was display, under certain circumstances.
	/// - The **Image View** display can fail to display the image in a window under the following circumstances:
	///   - With a **`Before and After window`**, the **`before`** image is not the same size as the **`after`** image. 
	///   - When displaying vectors, if the size given does not match the Vector memory size required for the image, the image will not be displayed.
	///
	pub fn is_valid(&self) -> bool { self.id != 0 }
}

impl Drop for ImageView { fn drop(&mut self) { unsafe { ext_func::sage_rust_image_view_destruct(self.id,self.is_ba); } } }

pub struct Sagebox
{
}
impl Sagebox
{

	pub fn get_conio() -> Conio
	{
		Conio::new()
	}
	
    /// Creates a new window on the desktop and returns a window object where you can perform window-based functions on that window.
	/// 
	/// Function Parameter Forms: 
	/// 
	/// 	- new_window();                 // Default Window
	/// 	- new_window_s(keywords : kw ); // Window with keyword options, such as title, size, color, position, etc.
	///
   	/// Keywords and Sagebox options can be included.  Some various options are as follows:
	/// 
	/// - title 		 - Sets the title of the window in the main window bar (otherwise no title is used), e.g. title("My Window")
	/// - pos 			 - Set position of window (default is used if not present)
	/// - size           - Sets the size of the window (otherwise default is used, typically 1200x800)
    /// - font           - Sets the default font for the window, i.e. "Font(40)" or "Font("Arial,40")"
    /// - bg_color       - Sets the background color of the window (i.e. "bgColor("Red")" or "bgColor(PanColor.ForestGreen())")
    /// - bg_gradient    - Sets a gradient background color of the window. Such as "bgGradient("black","blue").  You can also use Cls() to clear the window.
    /// - text_color     - Sets the text/foreground color of the window ("fg_color" also works)
    /// - no_auto_update - Tells the window to not update automatically when graphics are placed in the window.  The program must call Update().  This can prevent flashing in real-time drawing.
    /// - resize_ok      - Allows the window to be resized by the user. 
	/// 
    /// Examples:
	/// 
    ///     let my_window = Sagebox::new_window()
    ///     my_window.write("Hello World\\n")
	/// 
    /// 	let my_window = Sagebox::new_window_s(kw::title("This is the Title") + kw::Size(500,200) + kw::Pos(50,100));
	/// 	let my_window = Sagebox::new_window_s(kw::bg_color("Blue"));	
	///
	pub fn new_window_s(keyw: kw) -> Window
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_new_window(keyw.pointer); }
		Window::new(ret_val)
	//	x.auto() 	// Set auto-updates for now.  This may change.
	}
	
    /// Creates a new window on the desktop and returns a window object where you can perform window-based functions on that window.
	/// 
	/// Function Parameter Forms: 
	/// 
	/// 	- new_window();                 // Default Window
	/// 	- new_window_s(keywords : kw ); // Window with keyword options, such as title, size, color, position, etc.
	///
   	/// Keywords and Sagebox options can be included.  Some various options are as follows:
	/// 
	/// - title 		 - Sets the title of the window in the main window bar (otherwise no title is used), e.g. title("My Window")
	/// - pos 			 - Set position of window (default is used if not present)
	/// - size           - Sets the size of the window (otherwise default is used, typically 1200x800)
    /// - font           - Sets the default font for the window, i.e. "Font(40)" or "Font("Arial,40")"
    /// - bg_color       - Sets the background color of the window (i.e. "bgColor("Red")" or "bgColor(PanColor.ForestGreen())")
    /// - bg_gradient    - Sets a gradient background color of the window. Such as "bgGradient("black","blue").  You can also use Cls() to clear the window.
    /// - text_color     - Sets the text/foreground color of the window ("fg_color" also works)
    /// - no_auto_update - Tells the window to not update automatically when graphics are placed in the window.  The program must call Update().  This can prevent flashing in real-time drawing.
    /// - resize_ok      - Allows the window to be resized by the user. 
	/// 
    /// Examples:
	/// 
    ///     let my_window = Sagebox::new_window()
    ///     my_window.write("Hello World\\n")
	/// 
    /// 	let my_window = Sagebox::new_window_s(kw::title("This is the Title") + kw::Size(500,200) + kw::Pos(50,100));
	/// 	let my_window = Sagebox::new_window_s(kw::bg_color("Blue"));	
	///
	pub fn new_window() -> Window { Self::new_window_s(KW_NONE) }
	
	/// Creates a slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are integer.  see: dev_slider_f() to use floating-point values.
	///
	/// 	Forms: 
	/// 
	/// 	- dev_slider(title : str)                  -- Slider with no keywords added
	/// 	- dev_slider_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with dev_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - Sagebox::dev_slider("This is a slider")
	///                 - Sagebox::dev_slider_s("This is a slider",kw::range(100,500) + kw::default(200))
	///                 - Sagebox::dev_slider_s("This is a slider",kw::text_color("yellow") + kw::style="small"
	pub fn dev_slider_s(title : &str,keyw: kw) -> Slider
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_dev_slider(title.as_ptr(),title.len(),false,keyw.pointer); }
		Slider::new(ret_val)
	}
	
	/// Creates a slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are integer.  see: dev_sliderf() to use floating-point values.
	///
	/// 	Forms: 
	/// 
	/// 	- dev_slider(title : str)                  -- Slider with no keywords added
	/// 	- dev_slider_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with dev_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - Sagebox::dev_slider("This is a slider")
	///                 - Sagebox::dev_slider_s("This is a slider",kw::range(100,500) + kw::default(200))
	///                 - Sagebox::dev_slider_s("This is a slider",kw::text_color("yellow") + kw::style="small"
	pub fn dev_slider(title : &str) -> Slider { Self::dev_slider_s(title,KW_NONE) }

	///<summary> 
	/// Creates a floating-point slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are floating-point.  see: dev_slider() to use integer values values.
	///
	/// 	Forms: 
	/// 
	/// 	- dev_sliderf(title : str)                  -- Slider with no keywords added
	/// 	- dev_sliderf_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with dev_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - Sagebox::dev_sliderf("This is a slider")
	///                 - Sagebox::dev_sliderf_s("This is a slider",kw::range(-5.5,15.8) + kw::default(200))
	///                 - Sagebox::dev_sliderf_s("This is a slider",kw::text_color("yellow") + kw::style="small"
	/// </summary>
	pub fn dev_sliderf_s(title : &str,keyw: kw) -> Slider
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_dev_slider(title.as_ptr(),title.len(),true,keyw.pointer); }
		Slider::new(ret_val)
	}
	
	/// Creates a floating-point slider in the Dev Window. The slider is automatically placed.
	/// <br /><br />
	///  
	///     - A Slider class object is returned where the slider can be accessed and controlled. 
	///     - Values are floating-point.  see: dev_slider() to use integer values values.
	///
	/// 	Forms: 
	/// 
	/// 	- dev_sliderf(title : str)                  -- Slider with no keywords added
	/// 	- dev_sliderf_s(title : str,keywords : kw ) -- Slider with no keyword directives
	/// 
	///    Parameters:
	/// 
	///    - title                  -- Sets the title of the Slider
	///    - [optional keywords]    -- Add various kw:: keywords with dev_slider_s()
	/// 
	///    kw:: box options can be included.  Some various options are as follows:
	///
	///    - range             -- Set the range of the slider.  The default range is (1,100)          
	///    - default           -- Set the default value of the slider.  The default is (0)
	///    - textColor         -- Sets the color of the label of the slider.
	///    - style("small")    -- Sets a smaller slider handle
	///
	/// 
	///    Examples:    - Sagebox::dev_sliderf("This is a slider")
	///                 - Sagebox::dev_sliderf_s("This is a slider",kw::range(-5.5,15.8) + kw::default(200))
	///                 - Sagebox::dev_sliderf_s("This is a slider",kw::text_color("yellow") + kw::style="small"
	pub fn dev_sliderf(title : &str) -> Slider { Self::dev_sliderf_s(title,KW_NONE) }

	/// dev_inputbox() - Creates a new Input Box where you can enter text data as simple as a few characters are entire paragraphs. 
	/// 
	/// The Input box is placed in automaitcally in the Dev Window.
	///
	/// - Input boxes can be limited to only numbers, either integer or float values
	/// - Input boxes that accept only numbers can use the range() keyword to set the valid range. 
	/// - Input boxes can be set to automatically bring up an 'invalid' window for empty text or numbers out of range.
	/// - Input boxes can be set to readonly, and can also be set to display '*' for text when entering passwords.
	/// 
	/// **Function Parameter Forms:**
	///
	///		dev_inputbox(title : &str)              // Creates in Input box with a a title
	///		dev_inputbox_s(title : &str,keyw: kw)   // Used when entering keywords to shape the input box characteristics
	///
	/// **Available Keywords with dev_inputbox()**
	/// 
	/// Keyword | Action
	///---- | ----
	/// **width**  		| -Sets the width of the input box in pixels.  The default is 250 pixels.
	/// **height**        | -Sets the height of the input box in pixels.  The default is set to the default Font for 1 line of text. 
	/// **text**          | -This sets the starting text for the input box.  Otheriwse the input box is left blank at first. 
	/// **font**         | - Sets the font for the input box.  The default is the current font of the window
	/// **numbersonly**  | - Causes the input box to only accept numbers. 
	/// **readonly**     | - Sets the input box as read only so it can be used as a way to place a large amount of text that can be copied.
	/// **textcolor**    | - Sets the color of the text in the input box
	/// **bgcolor**      | - Sets the background color of the text in the input box
	/// **label**        | - Sets a label to the right of the input box. LabelRight, LabelLeft, LabelBottom, and LabelTop can be used to set the location of the label.
	/// **label_color**  | - Sets the color of the label (i.e. opt.label_color("Red"))
	/// **multiline**    | - Sets the input box as a multi-line input box.  This allows more than one line to be entered. A button or some method to end input must be used unless "WantReturn" is specified
	/// **wantreturn**   | - For multi-line boxes, this sends a "return pressed" message when the return key is pressed.
	/// **password**     | - Causes the input box to display '*' for all text.
	/// **noborder**     | - Causes the input box to not use a border so it will blend into the window more seamlessly.
	/// **thickborder,recessed** 	| - These are two different border styles that can be used.
	/// **vscroll, hscroll**      | - Adds a vertical scrollbar (useful for multi-line boxes) and horizontal scroll bar, respectively
	/// **wincolors**  	| - Sets the background input box color and text color to the current window color instead of the default white-and-black colors. 
	/// 
	/// Examples: 
	///
	///     Sagebox::dev_inputbox();                   // Create input box
	///     Sagebox::dev_inputbox_s(kw::height(500));  // Create an input box of height 500 pixels
	///     Sagebox::dev_inputbox_s(kw::font(20) + kw:label("Enter Data"));
	///     Sagebox::dev_inputbox_s(kw::text("This is the default text")); 		// Create an input box and set initial text.
	///
	pub fn dev_inputbox(title : &str) -> InputBox { Self::dev_inputbox_s(title,KW_NONE) }

	/// dev_inputbox() - Creates a new Input Box where you can enter text data as simple as a few characters are entire paragraphs. 
	/// 
	/// The Input box is placed in automaitcally in the Dev Window.
	///
	/// - Input boxes can be limited to only numbers, either integer or float values
	/// - Input boxes that accept only numbers can use the range() keyword to set the valid range. 
	/// - Input boxes can be set to automatically bring up an 'invalid' window for empty text or numbers out of range.
	/// - Input boxes can be set to readonly, and can also be set to display '*' for text when entering passwords.
	/// 
	/// **Function Parameter Forms:**
	///
	///		dev_inputbox(title : &str)              // Creates in Input box with a a title
	///		dev_inputbox_s(title : &str,keyw: kw)   // Used when entering keywords to shape the input box characteristics
	///
	/// **Available Keywords with dev_inputbox()**
	/// 
	/// Keyword | Action
	///---- | ----
	/// **width**  		| -Sets the width of the input box in pixels.  The default is 250 pixels.
	/// **height**        | -Sets the height of the input box in pixels.  The default is set to the default Font for 1 line of text. 
	/// **text**          | -This sets the starting text for the input box.  Otheriwse the input box is left blank at first. 
	/// **font**         | - Sets the font for the input box.  The default is the current font of the window
	/// **numbersonly**  | - Causes the input box to only accept numbers. 
	/// **readonly**     | - Sets the input box as read only so it can be used as a way to place a large amount of text that can be copied.
	/// **textcolor**    | - Sets the color of the text in the input box
	/// **bgcolor**      | - Sets the background color of the text in the input box
	/// **label**        | - Sets a label to the right of the input box. LabelRight, LabelLeft, LabelBottom, and LabelTop can be used to set the location of the label.
	/// **label_color**  | - Sets the color of the label (i.e. opt.label_color("Red"))
	/// **multiline**    | - Sets the input box as a multi-line input box.  This allows more than one line to be entered. A button or some method to end input must be used unless "WantReturn" is specified
	/// **wantreturn**   | - For multi-line boxes, this sends a "return pressed" message when the return key is pressed.
	/// **password**     | - Causes the input box to display '*' for all text.
	/// **noborder**     | - Causes the input box to not use a border so it will blend into the window more seamlessly.
	/// **thickborder,recessed** 	| - These are two different border styles that can be used.
	/// **vscroll, hscroll**      | - Adds a vertical scrollbar (useful for multi-line boxes) and horizontal scroll bar, respectively
	/// **wincolors**  	| - Sets the background input box color and text color to the current window color instead of the default white-and-black colors. 
	/// 
	/// Examples: 
	///
	///     Sagebox::dev_inputbox();                   // Create input box
	///     Sagebox::dev_inputbox_s(kw::height(500));  // Create an input box of height 500 pixels
	///     Sagebox::dev_inputbox_s(kw::font(20) + kw:label("Enter Data"));
	///     Sagebox::dev_inputbox_s(kw::text("This is the default text")); 		// Create an input box and set initial text.
	///
	pub fn dev_inputbox_s(title : &str,keyw: kw) -> InputBox
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_dev_inputbox(title.as_ptr(),title.len(),keyw.pointer); }
		InputBox::new(ret_val)
	}


   	/// dev_text_widget - Creates a text widget in the Dev Window. 
   	/// 
   	/// - <i>dev_text()</i> returns a <i>TextWidget</i> object where you can control and write to the text widget.
   	/// - The created text widget can be used to create static or dynamic text of any font size in the Dev Window.
	/// - Text Widget Objects returned (<i>TextWidget</i> type) does not need to be kept of the text widget will never be updated once the text widget is created.  Otherwise,
	/// the returned <i>TextWidget</i> object can be used to change the text or other aspects of the text widget.
	///
	/// **Function Parameter Forms:**
	///
	///		dev_text_widget(title : &str)              // Creates a text widget with a a title
	///		dev_text_widget_s(title : &str,keyw: kw)   // Used when entering keywords to shape the text widget characteristics
	///
	/// **Available Keywords with dev_text_widget()**
	/// 
	/// Keyword | Action
	///---- | ----
   	/// text  | - Sets the text of the widget.  This can be set later with <i>text_widget.write()</i>. When text is entered, the text widget is created to the width of the text.  Use the width() parameter to set a width or pad the text with spaces to reserve width.
   	/// textColor  | - Sets the text color of the widget (default is current window text color).  Same as <i>kw::fg_color()</i>
   	/// font       | - Sets the font of the text in the text widget
   	/// textCenter  | - Centers the text inside of the widget (which can be longer than the text itself). - Use <i>text_center_x()</i> and <i>center_x()</i> together to make sure text is centered in the window. This is only needed if the Width of the   	///                         - Text Widget and the text have been specificed separately.
   	/// 
   	/// Examples:
	///
   	///		let widget = Sagebox::dev_text_widget("This is a text Widget"); // Create a basic text widget with text.  Return value does not need to be kept.
	///
   	///		let widget = Sagebox::dev_text_widget("This is a text Widget",kw::font(15) + kw::fg_color("red")) // Create a text widget with text.  That has a font size of 15 and a text color of red.
	///
	pub fn dev_text_widget_s(title : &str,keyw: kw) -> Slider
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_text_widget(title.as_ptr(),title.len(),-1234,keyw.pointer); }
		Slider::new(ret_val)
	}

   	/// dev_text_widget - Creates a text widget in the Dev Window. 
   	/// 
   	/// - <i>dev_text()</i> returns a <i>TextWidget</i> object where you can control and write to the text widget.
   	/// - The created text widget can be used to create static or dynamic text of any font size in the Dev Window.
	/// - Text Widget Objects returned (<i>TextWidget</i> type) does not need to be kept of the text widget will never be updated once the text widget is created.  Otherwise,
	/// the returned <i>TextWidget</i> object can be used to change the text or other aspects of the text widget.
	///
	/// **Function Parameter Forms:**
	///
	///		dev_text_widget(title : &str)              // Creates a text widget with a a title
	///		dev_text_widget_s(title : &str,keyw: kw)   // Used when entering keywords to shape the text widget characteristics
	///
	/// **Available Keywords with dev_text_widget()**
	/// 
	/// Keyword | Action
	///---- | ----
   	/// text  | - Sets the text of the widget.  This can be set later with <i>text_widget.write()</i>. When text is entered, the text widget is created to the width of the text.  Use the width() parameter to set a width or pad the text with spaces to reserve width.
   	/// textColor  | - Sets the text color of the widget (default is current window text color).  Same as <i>kw::fg_color()</i>
   	/// font       | - Sets the font of the text in the text widget
   	/// textCenter  | - Centers the text inside of the widget (which can be longer than the text itself). - Use <i>text_center_x()</i> and <i>center_x()</i> together to make sure text is centered in the window. This is only needed if the Width of the   	///                         - Text Widget and the text have been specificed separately.
   	/// 
   	/// Examples:
	///
   	///		let widget = Sagebox::dev_text_widget("This is a text Widget"); // Create a basic text widget with text.  Return value does not need to be kept.
	///
   	///		let widget = Sagebox::dev_text_widget("This is a text Widget",kw::font(15) + kw::fg_color("red")) // Create a text widget with text.  That has a font size of 15 and a text color of red.
	///
	pub fn dev_text_widget(title : &str) -> Slider
	{
		Sagebox::dev_text_widget_s(title,KW_NONE)
	}

    /// ### dev_text() - This function is TBD
	///
	pub fn dev_text_s(title : &str,keyw: kw) -> Window
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_text(title.as_ptr(),title.len(),-1234,keyw.pointer); }
		Window::new(ret_val)
	}

    /// ### dev_text() - This function is TBD
	///
	pub fn dev_text(title : &str) -> Window
	{
		Sagebox::dev_text_s(title,KW_NONE)
	}


    /// dev_checkbox() - Create a checkbox in the Dev Window.  The Checkbox is automatically placed. 
    /// 
	/// - The checkbox is unchecked by default. 
	/// - The input title is the text displayed directly to the right of the square checkbox.
	///
	/// **Function Parameter Forms:**
	///
	///		dev_checkbox(title : &str)             // Creates a checkbox with the title displayed to the right of the checkbox
	///		dev_checkbox_s(title : &str,keyw: kw)  // Used when entering keywords to shape checkbox characteristics
	///
	/// **Some Available Keywords with dev_checkbox() (partial list)**
	/// 
    /// - **default** - <i>default(true)</i> sets the check in the checkbox.  <i>default(false)</i> leaves the checkbox unchecked.
    /// - **text_color** - Set the text of the checkbox.  This is the same keyword as <i>fg_color()</i>
    /// - **font** - Set the font of the checkbox.
	///
    /// Examples:
	///
    ///		let checkbox = Sagebox::dev_checkbox("This is a checkbox");                           // Create an unchecked checkbox
    ///		let checkbox = Sagebox::dev_checkbox_s("This is a checkbox",kw::default(true));       // Create a checked checkbox
    ///		let checkbox = Sagebox::dev_checkbox_s("This is a checkbox",kw::text_color("green")); // Create a checkbox with green text.
	///
	pub fn dev_checkbox_s(title : &str,keyw: kw) -> Button
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_checkbox(title.as_ptr(),title.len(),-1234,keyw.pointer); }
		Button::new(ret_val,true)
	}

    /// dev_checkbox() - Create a checkbox in the Dev Window.  The Checkbox is automatically placed. 
    /// 
	/// - The checkbox is unchecked by default. 
	/// - The input title is the text displayed directly to the right of the square checkbox.
	///
	/// **Function Parameter Forms:**
	///
	///		dev_checkbox(title : &str)             // Creates a checkbox with the title displayed to the right of the checkbox
	///		dev_checkbox_s(title : &str,keyw: kw)  // Used when entering keywords to shape checkbox characteristics
	///
	/// **Some Available Keywords with dev_checkbox() (partial list)**
	/// 
    /// - **default** - <i>default(true)</i> sets the check in the checkbox.  <i>default(false)</i> leaves the checkbox unchecked.
    /// - **text_color** - Set the text of the checkbox.  This is the same keyword as <i>fg_color()</i>
    /// - **font** - Set the font of the checkbox.
	///
    /// Examples:
	///
    ///		let checkbox = Sagebox::dev_checkbox("This is a checkbox");                           // Create an unchecked checkbox
    ///		let checkbox = Sagebox::dev_checkbox_s("This is a checkbox",kw::default(true));       // Create a checked checkbox
    ///		let checkbox = Sagebox::dev_checkbox_s("This is a checkbox",kw::text_color("green")); // Create a checkbox with green text.
	///
	pub fn dev_checkbox(title : &str) -> Button
	{
		Sagebox::dev_checkbox_s(title,KW_NONE)
	}

    /// [**dev_combobox()**] -- Creates a Combobox in the DevWindow.  The Combobox is automatically placed. 
    /// 
    /// - A combo box  is like a list box except that it consists of a single tab that expands when activated, 
    /// and rolls back up when released. 
    /// - This allows multiple listbox-style entries to take only the space of the height of one text line. 
    /// - <i>dev_combobox()</i> returns a Combobox type object so that items may be added and deleted, and user selections retrieved. 
    /// - When the <i>title()</i> keyword is used, the title appears above the combobox to the left. 
	/// - <i>title_right()</i> and <i>title_left()</i> can be used to place the title to the left or right of the combo box, respectively.
	///
 	/// **Function Parameter Forms:**
	///
	///		dev_combobox(title : &str)             // Creates a combo box with the title displayed to the right of the checkbox
	///		dev_combobox_s(title : &str,keyw: kw)  // Used when entering keywords to shape combobox characteristics
	///
	/// **Some Available Keywords with dev_combobox() (partial list)**
    /// 
    /// - **text** - Initial text in the combobox.  This text can be one line or multiple lines representing multiple entries.  See examples.
    /// - **title_cell** - Tells the combobox to display this string int the combobox tab when no item is selected.  Otherwise the first added item is displayed.
    /// - **default** - Default selection.  This is the index to the default selection (0 is the first selection, 1 the second, etc.)
    /// 
    /// **Examples:**
    /// 
    ///		let my_combobox = Sagebox::dev_combobox("First Item") 
    ///		let my_combobox = Sagebox::dev_combobox("First Item\\nSecond Item\\nThird Item") 
    ///		let my_combobox = Sagebox::dev_combobox(titlecell="This is a combobox") 
    ///		let my_combobox = Sagebox::dev_combobox_s("First Item\\nSecond Item\\nThird Item",kw::default(2)) 
    ///		let my_combobox = Sagebox::dev_combobox_s("First Item\\nSecond Item\\nThird Item",kw::title("This is a combobox"));
	///
	pub fn dev_combobox_s(title : &str,keyw: kw) -> ComboBox
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_combo_box(title.as_ptr(),title.len(),-1234,keyw.pointer); }
		ComboBox::new(ret_val)
	}

    /// [**dev_combobox()**] -- Creates a Combobox in the DevWindow.  The Combobox is automatically placed. 
    /// 
    /// - A combo box  is like a list box except that it consists of a single tab that expands when activated, 
    /// and rolls back up when released. 
    /// - This allows multiple listbox-style entries to take only the space of the height of one text line. 
    /// - <i>dev_combobox()</i> returns a Combobox type object so that items may be added and deleted, and user selections retrieved. 
    /// - When the <i>title()</i> keyword is used, the title appears above the combobox to the left. 
	/// - <i>title_right()</i> and <i>title_left()</i> can be used to place the title to the left or right of the combo box, respectively.
	///
 	/// **Function Parameter Forms:**
	///
	///		dev_combobox(title : &str)             // Creates a combo box with the title displayed to the right of the checkbox
	///		dev_combobox_s(title : &str,keyw: kw)  // Used when entering keywords to shape combobox characteristics
	///
	/// **Some Available Keywords with dev_combobox() (partial list)**
    /// 
    /// - **text** - Initial text in the combobox.  This text can be one line or multiple lines representing multiple entries.  See examples.
    /// - **title_cell** - Tells the combobox to display this string int the combobox tab when no item is selected.  Otherwise the first added item is displayed.
    /// - **default** - Default selection.  This is the index to the default selection (0 is the first selection, 1 the second, etc.)
    /// 
    /// **Examples:**
    /// 
    ///		let my_combobox = Sagebox::dev_combobox("First Item") 
    ///		let my_combobox = Sagebox::dev_combobox("First Item\\nSecond Item\\nThird Item") 
    ///		let my_combobox = Sagebox::dev_combobox(titlecell="This is a combobox") 
    ///		let my_combobox = Sagebox::dev_combobox_s("First Item\\nSecond Item\\nThird Item",kw::default(2)) 
    ///		let my_combobox = Sagebox::dev_combobox_s("First Item\\nSecond Item\\nThird Item",kw::title("This is a combobox"));
	///
	pub fn dev_combobox(title : &str) -> ComboBox
	{
		Sagebox::dev_combobox_s(title,KW_NONE)
	}

    /// [**dev_radio_buttons()**] - Creates a group of Radio Buttons with an optional outer border and label.  
    /// 
	/// - With radio buttons, only one can be selected at a time.  If one is selected, the currently selected radio button is unselected.
    /// - The Radio Button group is placed in the Dev Window automatically.
	/// - One radio button is selected at all times.  This defaults to the first (e.g. position 0) button, but can be changed with <i>radio_button.set_selection()</i>. 
    /// - <i>dev_radio_buttons()</i> returns a <i>ButtonGroup</i> object class where the buttons may be queried to see when pressed, and which button was pressed.
	/// - The radio buttons are outline with a thin line, with a title to the upper-left when the <i>title()</i> keyword is used.
	/// - Radio buttons are ordered vertically with one button per-line, unless the <i>horz()</i> and/or <i>columns()</i> keywords are used (see below)
	///
	/// **Function Parameter Forms:**
	///
	///		dev_radio_buttons(buttons_text : &str)             // Creates a set of radio buttons
	///		dev_radio_buttons_s(buttons_text : &str,keyw: kw)  // Used when entering keywords to shape the radio buttons characteristics
	///
	/// **Parameters:**
    /// 
    /// - button_text  - The radio button text for each radio button, separated by a new line ('\n') - see examples below. 
    ///
	/// **Some Available Keywords with dev_radio_buttons() (partial list)**
	///
    /// - **title** - The title/label of the radio button group.  A box is drawn around the radio buttons with the title name.
    /// - **default** - Sets the default index for the highlighted button.  There can only be one active radio button.  default() sets the index of the active/highlighted button (i.e. 0 = first button, 1 = second button, etc.)
    /// - **text_color** - Sets the text color for the radio buttons (same as fg_color())
	/// - **font** - Sets the font of the radio button text
	/// - **horz** - Makes the radio buttons appear one one horizontal area, rather than vertically
	/// - **columns** - Sets the number of radio buttons per-line, so that more than one radio button can appear on a single line, but the radio buttons can also use
	/// multiple lines when there are many buttons.
	///
    /// **Examples:**   
    /// 
    ///		let my_radio_box = Sagebox::dev_radio_buttons("This is a single radio button button")
    ///		let my_radio_box = Sagebox::dev_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list"))
    ///		let my_radio_box = Sagebox::dev_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list") + kw::default(1))
	///
	pub fn dev_radio_buttons_s(buttons_text : &str,keyw: kw) -> ButtonGroup
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_radio_buttons(buttons_text.as_ptr(),buttons_text.len(),-1234,keyw.pointer); }
		ButtonGroup::new(ret_val)
	}

    /// [**dev_radio_buttons()**] - Creates a group of Radio Buttons with an optional outer border and label.  
    /// 
	/// - With radio buttons, only one can be selected at a time.  If one is selected, the currently selected radio button is unselected.
    /// - The Radio Button group is placed in the Dev Window automatically.
	/// - One radio button is selected at all times.  This defaults to the first (e.g. position 0) button, but can be changed with <i>radio_button.set_selection()</i>. 
    /// - <i>dev_radio_buttons()</i> returns a <i>ButtonGroup</i> object class where the buttons may be queried to see when pressed, and which button was pressed.
	/// - The radio buttons are outline with a thin line, with a title to the upper-left when the <i>title()</i> keyword is used.
	/// - Radio buttons are ordered vertically with one button per-line, unless the <i>horz()</i> and/or <i>columns()</i> keywords are used (see below)
	///
	/// **Function Parameter Forms:**
	///
	///		dev_radio_buttons(buttons_text : &str)             // Creates a set of radio buttons
	///		dev_radio_buttons_s(buttons_text : &str,keyw: kw)  // Used when entering keywords to shape the radio buttons characteristics
	///
	/// **Parameters:**
    /// 
    /// - button_text  - The radio button text for each radio button, separated by a new line ('\n') - see examples below. 
    ///
	/// **Some Available Keywords with dev_radio_buttons() (partial list)**
	///
    /// - **title** - The title/label of the radio button group.  A box is drawn around the radio buttons with the title name.
    /// - **default** - Sets the default index for the highlighted button.  There can only be one active radio button.  default() sets the index of the active/highlighted button (i.e. 0 = first button, 1 = second button, etc.)
    /// - **text_color** - Sets the text color for the radio buttons (same as fg_color())
	/// - **font** - Sets the font of the radio button text
	/// - **horz** - Makes the radio buttons appear one one horizontal area, rather than vertically
	/// - **columns** - Sets the number of radio buttons per-line, so that more than one radio button can appear on a single line, but the radio buttons can also use
	/// multiple lines when there are many buttons.
	///
    /// **Examples:**   
    /// 
    ///		let my_radio_box = Sagebox::dev_radio_buttons("This is a single radio button button")
    ///		let my_radio_box = Sagebox::dev_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list"))
    ///		let my_radio_box = Sagebox::dev_radio_buttons_s("button 1\\nbutton2\\nbutton 3",kw::title("This a radio button list") + kw::default(1))
	///
	pub fn dev_radio_buttons(buttons_text : &str) -> ButtonGroup
	{
		Sagebox::dev_radio_buttons_s(buttons_text,KW_NONE)
	}

	/// [**dev_button()**] — Create a button in the Dev Window
	/// 
	/// **Function Parameter Forms:**
	///
	///		dev_button(location,title : &str)              // Creates a button with the title specified.
	///		dev_button_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the button characteristics
	///
	/// - Buttons are automatically sized based on the text title and font (supplied font or default)
	/// - For buttons with short words, e.g. "stop", spaces may be added to make buttons longer, such as "    Stop    ".
	///   - Also see the **<i>width()</>** keyword below
	///
	/// **Some Keywords Available with Buttons**
	///
	/// - size() 	 - Set the size of the button
	/// - width()    - Set the width of the button (height is calculated automatically)
	/// - font() 	 - Set the font used in the button
	/// - hidden()   - Set the button as initially hidden
	/// - disabled() - Set the button as initially disabled
	/// - — There are many more keywords that will be documented in the next release.
	///
	/// **A note about the <i>kw::width()</> keyword**
	/// 
	/// - Using the **<i>kw::width()</>** keyword has a couple great uses:
	///
	///   - When defining multiple buttons, this can create uniform length for buttons, making them look more organized, also allowing lining them up together
	///   - When using the **<i>button.set_text()</>** function to change the text, this can reserve space for the longest text that may later be set in the button.
	///
	/// **[Special Note]**
	///
	/// - Buttons may take a wide variety of shapes and forms, with many keywords to affect their behavior.
	///  - This documentation is still being written and the interface still in-progress.
	/// - More will be added in the next release, with more button types and behavior controls.
	///
	/// **Examples**
	///
	/// 		dev_button((100,200),"This is a button");                   // Create a button at 100,200
	/// 		dev_button_s((100,200),"This is a button",kw::font(25));    // Create a button with a font size of 25
	/// 		dev_button_s((100,200),"Stop",kw::Width(100))				// Create a wider button, since the "stop" title will create a small one if automatic.
	///
	pub fn dev_button_s(title : &str,keyw: kw) -> Button
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_add_button(title.as_ptr(),title.len(),-1234,keyw.pointer); }
		Button::new(ret_val,false)
	}

	/// [**dev_button()**] — Create a button in the Dev Window
	/// 
	/// **Function Parameter Forms:**
	///
	///		dev_button(location,title : &str)              // Creates a button with the title specified.
	///		dev_button_s(location,title : &str,keyw: kw)   // Used when entering keywords to shape the button characteristics
	///
	/// - Buttons are automatically sized based on the text title and font (supplied font or default)
	/// - For buttons with short words, e.g. "stop", spaces may be added to make buttons longer, such as "    Stop    ".
	///   - Also see the **<i>width()</>** keyword below
	///
	/// **Some Keywords Available with Buttons**
	///
	/// - size() 	 - Set the size of the button
	/// - width()    - Set the width of the button (height is calculated automatically)
	/// - font() 	 - Set the font used in the button
	/// - hidden()   - Set the button as initially hidden
	/// - disabled() - Set the button as initially disabled
	/// - — There are many more keywords that will be documented in the next release.
	///
	/// **A note about the <i>kw::width()</> keyword**
	/// 
	/// - Using the **<i>kw::width()</>** keyword has a couple great uses:
	///
	///   - When defining multiple buttons, this can create uniform length for buttons, making them look more organized, also allowing lining them up together
	///   - When using the **<i>button.set_text()</>** function to change the text, this can reserve space for the longest text that may later be set in the button.
	///
	/// **[Special Note]**
	///
	/// - Buttons may take a wide variety of shapes and forms, with many keywords to affect their behavior.
	///  - This documentation is still being written and the interface still in-progress.
	/// - More will be added in the next release, with more button types and behavior controls.
	///
	/// **Examples**
	///
	/// 		dev_button((100,200),"This is a button");                   // Create a button at 100,200
	/// 		dev_button_s((100,200),"This is a button",kw::font(25));    // Create a button with a font size of 25
	/// 		dev_button_s((100,200),"Stop",kw::Width(100))				// Create a wider button, since the "stop" title will create a small one if automatic.
	///
	pub fn dev_button(title : &str) -> Button
	{
		Sagebox::dev_button_s(title,KW_NONE)
	}



	/// exit_button() comes up with a message and an "OK" button. 
    /// 
	/// - exit_button() is quick way to let the user know that the program has ended
	/// 
	/// Forms: 
	/// 
	/// - exit_button()                  - exit_button() with default "program ended" wording
	/// - exit_button_str(msg : &str)    - Add your own message via string slice
	///  
    /// See win.exit_button() to put an exit button at the bottom of a window instead.
    /// 
	/// Parameters
    /// 
    /// - Msg       -- [optional].  Message to put in the window above the button.  If not specified, a default "program is finished" message is place in the window.
    /// 
    /// About exit_button() and Displays using Windows and Graphics
    /// 
    /// - The exit_button() gives a way to pause the program before it ends so that the Windows and Graphics displaying don't suddenly disappear. 
    /// 
    /// - The Console Window is not part of the running application.  It is a separate application to which the running program is a client.
    /// When the program closes down, this other application (a separate process entirely) simply lets you know the program has ended.
    /// 
    /// - When windows and graphics are displayed in your program, and the program ends, it all disappears suddenly.  In a program with a Console Mode box, this will show a "program has ended"
    /// message in the box.  In a Windows application, the program will just "disappear"
    /// 
    /// - You can use exit_button() as a nice, quick method to pause and let the user know the program is ending in a GUI/graphical method.
	pub fn exit_button_s(msg : &str) -> i64
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_exit_button(msg.as_ptr(),msg.len()); }		
		ret_val

	}	

	/// exit_button() comes up with a message and an "OK" button. 
    /// 
	/// - exit_button() is quick way to let the user know that the program has ended
	/// 
	/// Forms: 
	/// 
	/// - exit_button()                  - exit_button() with default "program ended" wording
	/// - exit_button_str(msg : &str)    - Add your own message via string slice
	///  
    /// See win.exit_button() to put an exit button at the bottom of a window instead.
    /// 
	/// Parameters
    /// 
    /// - Msg       -- [optional].  Message to put in the window above the button.  If not specified, a default "program is finished" message is place in the window.
    /// 
    /// About exit_button() and Displays using Windows and Graphics
    /// 
    /// - The exit_button() gives a way to pause the program before it ends so that the Windows and Graphics displaying don't suddenly disappear. 
    /// 
    /// - The Console Window is not part of the running application.  It is a separate application to which the running program is a client.
    /// When the program closes down, this other application (a separate process entirely) simply lets you know the program has ended.
    /// 
    /// - When windows and graphics are displayed in your program, and the program ends, it all disappears suddenly.  In a program with a Console Mode box, this will show a "program has ended"
    /// message in the box.  In a Windows application, the program will just "disappear"
    /// 
    /// - You can use exit_button() as a nice, quick method to pause and let the user know the program is ending in a GUI/graphical method.
	pub fn exit_button() -> i64
	{
		Self::exit_button_s("")
	}


	/// [**get_event**] - waits for an event to occur in your main body of code. 
	///
	/// - Events are any events such as a mouse move, click, keyboard press, etc. 
	/// - Events are also caused by any control change such as moving a slider, pressing a button, or pressing return in an input box. 
	/// - Events can also be a window closing, a window moving or changing size -- basically anything happening in the system sends a message
	///
	/// Until an event occurs, the program is sleeping and not using any CPU time.  Sagebox wakes up the program when an event happens. 
	///
	/// In the event loop, you can check for events.  
	///
	/// get_event() returns true until all main windows are closed. 
	///
	/// **Note**
	///	
	/// - **<i>wait_event()</i>** and **<i>get_event()</i>** are the same function.
	/// - **<i>get_event()</i>** is deprecated and is included to maintain compatibility between platforms.
	///
	/// **Example:**
	///
	///         let my_button = Sagebox::dev_button("Press Me");
	///         let my_slider = Sagebox::dev_slider();
	///
	///         while Sagebox::get_event()
	/// 		{
	///             if my_button.pressed() { println!("My Button was Pressed"); }
	///
	/// 			let pos = my_slider.get_pos();
	///             if my_slider.moved() { println!("Range Slider is now at position",pos); }
	///			}
	///
	/// In this example, the program sleeps until an event occurs, and then the program checks to see if an event occured with a button or a slider movement. 
	///
	/// Events are typically one-time:  They report "true" on the first call, and then fals afterwards until another event of the same type happens. 
	/// 
	/// You can also use a callback to retrieve events.  Though this is not recommended or useful for most programs, it can be useful for some specific purposes.
	/// See set_event_callback() for more information.
	/// 
	pub fn get_event() -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_get_event(); }
		ret_val
	}


	/// [**wait_event()**] -  waits for an event to occur in your main body of code. 
	///
	/// - Events are any events such as a mouse move, click, keyboard press, etc. 
	/// - Events are also caused by any control change such as moving a slider, pressing a button, or pressing return in an input box. 
	/// - Events can also be a window closing, a window moving or changing size -- basically anything happening in the system sends a message
	///
	/// Until an event occurs, the program is sleeping and not using any CPU time.  Sagebox wakes up the program when an event happens. 
	///
	/// In the event loop, you can check for events.  
	///
	/// wait_event() returns true until all main windows are closed. 
	///
	/// **Note**
	///	
	/// - **<i>wait_event()</i>** and **<i>get_event()</i>** are the same function.
	/// - **<i>get_event()</i>** is deprecated and is included to maintain compatibility between platforms.
	///
	/// **Example:**
	///
	///         let my_button = Sagebox::dev_button("Press Me");
	///         let my_slider = Sagebox::dev_slider();
	///
	///         while Sagebox::wait_event()
	/// 		{
	///             if my_button.pressed() { println!("My Button was Pressed"); }
	///
	/// 			let pos = my_slider.get_pos();
	///             if my_slider.moved() { println!("Range Slider is now at position",pos); }
	///			}
	///
	/// In this example, the program sleeps until an event occurs, and then the program checks to see if an event occured with a button or a slider movement. 
	///
	/// Events are typically one-time:  They report "true" on the first call, and then fals afterwards until another event of the same type happens. 
	/// 
	/// You can also use a callback to retrieve events.  Though this is not recommended or useful for most programs, it can be useful for some specific purposes.
	/// See set_event_callback() for more information.
	/// 
	pub fn wait_event() -> bool
	{
		let ret_val : bool;
		unsafe { ret_val = ext_func::sage_rust_get_event(); }
		ret_val
	}

	fn _console_write(msg : &str) -> bool
	{
		unsafe { ext_func::rust_sagebox_console_write(msg.as_ptr(),msg.len(),false,false) }
	}	
	fn _console_writeln(msg : &str) -> bool
	{
		unsafe { ext_func::rust_sagebox_console_write(msg.as_ptr(),msg.len(),true,false) }
	}	
	fn _debug_write(msg : &str) -> bool
	{
		unsafe { ext_func::rust_sagebox_console_write(msg.as_ptr(),msg.len(),false,true) }
	}	
	fn _debug_writeln(msg : &str) -> bool
	{
		unsafe { ext_func::rust_sagebox_console_write(msg.as_ptr(),msg.len(),true,true) }
	}


	/// [**rand()**] - Generates an integer (i32) random number between 0 and the range submiitted (up to 16,777,215)
	/// 
	/// - The <i>**Sagebox::rand()**<i/> function generates a fast random number.  This can be used in place of std::rand(), which
	///while extensive, can also be very slow for simple applications (in the range of 10x slower). 
	/// - Range values can be negative (e.g. -200,300) - the entire range should be within 16,777,215.
	///
	/// **Function Parameter Forms:**
	///
	///		rand(rand_range : i64)                         // Generates a random number from 0 to the input range (up to 16,777,215) 
	///		rand_range(min_value : i32,max_value : i32)    // Generates a random number from the start value to the max value. 
	///		rand_seed(random_seed : i32)                   // Seeds the random generator to give different results.
	///		rand_seed_time()                               // Generates a random seed based on the current time (guaranteeing it is different each time)
	///
	/// **Examples:**
	/// 
	///		let rand_value = Sagebox::rand(200);      // Generate a random number between 0 and 200
	///		let rand_value = Sagebox::rand(-200,300)  // Generate a random number between -200 and 299
	///		Sagebox::rand_seed(1234);                 // Seed the random number generator with the value 1234
	///		Sagebox::rand_seed_time();                // Seed the random number generator with the current time in ms.
	///
	pub fn rand(rand_range : i64) -> i32
	{
		let ret_val : i32;
		unsafe { ret_val = ext_func::rust_sagebox_rand(0 as i64,rand_range); }
		ret_val
	}

	/// [**rand_range()**] - Generates an integer (i32) random number between 0 and the range submiitted (up to 16,777,215)
	/// 
	/// - The <i>**Sagebox::rand()**<i/> function generates a fast random number.  This can be used in place of std::rand(), which
	///while extensive, can also be very slow for simple applications (in the range of 10x slower). 
	/// - Range values can be negative (e.g. -200,300) - the entire range should be within 16,777,215.
	///
	/// **Function Parameter Forms:**
	///
	///		rand(rand_range : i64)                         // Generates a random number from 0 to the input range (up to 16,777,215) 
	///		rand_range(min_value : i32,max_value : i32)    // Generates a random number from the start value to the max value. 
	///		rand_seed(random_seed : i32)                   // Seeds the random generator to give different results.
	///		rand_seed_time()                               // Generates a random seed based on the current time (guaranteeing it is different each time)
	///
	/// **Examples:**
	/// 
	///		let rand_value = Sagebox::rand(200);      // Generate a random number between 0 and 200
	///		let rand_value = Sagebox::rand(-200,300)  // Generate a random number between -200 and 299
	///		Sagebox::rand_seed(1234);                 // Seed the random number generator with the value 1234
	///		Sagebox::rand_seed_time();                // Seed the random number generator with the current time in ms.
	///	
	pub fn rand_range(min_value : i32,max_value : i32) -> i32
	{
		let ret_val : i32;
		unsafe { ret_val = ext_func::rust_sagebox_rand(min_value as i64,max_value as i64); }
		ret_val
	}

	/// [**rand_seed()**] - Generates an integer (i32) random number between 0 and the range submiitted (up to 16,777,215)
	/// 
	/// - The <i>**Sagebox::rand()**<i/> function generates a fast random number.  This can be used in place of std::rand(), which
	///while extensive, can also be very slow for simple applications (in the range of 10x slower). 
	/// - Range values can be negative (e.g. -200,300) - the entire range should be within 16,777,215.
	///
	/// **Function Parameter Forms:**
	///
	///		rand(rand_range : i64)                         // Generates a random number from 0 to the input range (up to 16,777,215) 
	///		rand_range(min_value : i32,max_value : i32)    // Generates a random number from the start value to the max value. 
	///		rand_seed(random_seed : i32)                   // Seeds the random generator to give different results.
	///		rand_seed_time()                               // Generates a random seed based on the current time (guaranteeing it is different each time)
	///
	/// **Examples:**
	/// 
	///		let rand_value = Sagebox::rand(200);      // Generate a random number between 0 and 200
	///		let rand_value = Sagebox::rand(-200,300)  // Generate a random number between -200 and 299
	///		Sagebox::rand_seed(1234);                 // Seed the random number generator with the value 1234
	///		Sagebox::rand_seed_time();                // Seed the random number generator with the current time in ms.
	///		
	pub fn rand_seed(random_seed : i32)
	{
		unsafe { ext_func::rust_sagebox_srand(random_seed); }
	}

	/// [**rand_seed_time()**] - Generates an integer (i32) random number between 0 and the range submiitted (up to 16,777,215)
	/// 
	/// - The <i>**Sagebox::rand()**<i/> function generates a fast random number.  This can be used in place of std::rand(), which
	///while extensive, can also be very slow for simple applications (in the range of 10x slower). 
	/// - Range values can be negative (e.g. -200,300) - the entire range should be within 16,777,215.
	///
	/// **Function Parameter Forms:**
	///
	///		rand(rand_range : i64)                         // Generates a random number from 0 to the input range (up to 16,777,215) 
	///		rand_range(min_value : i32,max_value : i32)    // Generates a random number from the start value to the max value. 
	///		rand_seed(random_seed : i32)                   // Seeds the random generator to give different results.
	///		rand_seed_time()                               // Generates a random seed based on the current time (guaranteeing it is different each time)
	///
	/// **Examples:**
	/// 
	///		let rand_value = Sagebox::rand(200);      // Generate a random number between 0 and 200
	///		let rand_value = Sagebox::rand(-200,300)  // Generate a random number between -200 and 299
	///		Sagebox::rand_seed(1234);                 // Seed the random number generator with the value 1234
	///		Sagebox::rand_seed_time();                // Seed the random number generator with the current time in ms.
	///		
	pub fn rand_seed_time()
	{
		unsafe { ext_func::rust_sagebox_srand(-1234567); }
	}	

    /// [**get_color()**] - Returns a n RgbColor type with the color string specified, e.g. "Blue")
    /// 
    /// - The color <i>**White**</i> is returned if the color is not found.
	/// - Searching for colors is slower than using symbolic colors or rgb colors directly, especially in intensive loops.  All colors can be found through the symbolic values in <i>**pan_color**</i> and <i>**sage_color**</i> structs, e.g. <i>**pan_color::Green**</i>
    /// - Specifications for <i>**pan_color**</i> and <i>**sage_color structs**</i> can be used, such as <i>**"pan_color:beige"**</i> or <i>**"sage_color:orange"**</i>, which return Rgb values equivalent to their preset
	/// symbolic colors,  <i>**pan_color::beige**</i>  and  <i>**sage_color::orange**</i>, respectively.
	///
	/// **Function Parameter Forms:**
	///
	///		get_color(color_str : &str) -> RgbColor
	///
    /// **Examples:**
    /// 
    /// 	let blue = Sagebox::get_color("blue")
    /// 	let my_color = Sagebox::get_color("forestgreen");
    /// 	let my_color = Sagebox::get_color("Pancolor:forestgreen")
    /// 
    /// **Pantone and Sagebox colors can also be used symblically, avoiding string parsing:**
    /// 
    /// 	let my_color = sage_color::SkyBlue;
    /// 	let my_color = pan_color::ForestGreen;
	///
	pub fn get_color(color_str : &str) -> RgbColor
	{
	
		let color 	: RgbColor =  RgbColor::new(0,0,0);
		unsafe { ext_func::sage_rust_get_color(color_str.as_ptr(),color_str.len(),&color.red,&color.green,&color.blue); }
		color 	
	}

	fn _quick_form_s(msg : &str,keyw : kw) -> QuickForm
	{
		let qform : i64;
		let win : i64;
		let dev_win : i64;
		unsafe { 
					qform 	= ext_func::rust_window_quick_form(msg.as_ptr(),msg.len(),keyw.pointer); 
					win 	= ext_func::rust_quickform_get_window(qform);
					dev_win = ext_func::rust_quickform_get_dev_window(qform);
		}
		QuickForm::new(qform,Window{ id: win, gdi: Gdi::new(win) },DevWindow{id : dev_win})
	}

	fn _quick_form(msg : &str) -> QuickForm { Sagebox::_quick_form_s(msg,KW_NONE) } // $$ make initial outer window not sizeable

    /// [**read_image_file()**] - Reads an image file and returns a 24-bit RGB bitmap in a Bitmap object. 
    /// 
    /// - Input files may be **.JPG, .BMP, or .PNG.** 
	/// - See <i>**read_image_file32()**</i> to load files with alpha channels or transparencies into a 32-bit bitmap, such as .PNG or 32-bit bitmaps 
    /// - Bitmaps are returned in reverse Y order unless the <i>**kw::reverse()**</i> keyword is used (<i>[not currently implemented]</i>)
	/// - 24-bit bitmaps have a stride (i.e. width of each line) that is divisible by 4, i.e. a bitmap with a width of 25 pixels (75 total RGB pixels) has a width of 76 with an overlap of 1 pixel to keep divisible by 4.
	/// - Use the bitmaps <i>**bitmap.get_width_bytes()**</i> to get the stride (width) of each bitmap line. 
	///   - The total size of the bitmap will be the <i>width_bytes*height</i>.
	/// 
	/// **Function Parameter Forms:**
	///
	///		read_image_file(bitmap_path : &str)		// Reads the file specified by the string slice
	///
    /// **Notes:**
    /// 
    /// - return type: <i>**read_image_file()**</i> returns a 24-Bit Bitmap object with the bitmap.  If the file was not found, the bitmap will be empty.
    /// - Use the Bitmap's <i>**is_valid()**</i> or <i>**bitmap.empty()**</i> function to determine if the bitmap was read ok.
	///
	/// **Example:**
	///
	///			let my_bitmap = Sagebox::read_image_file("myfile.jpg")		// Load a bitmap
	///			if my_bitmap.is_empty() { return err; }						// Form an error if the bitmap was not found
	///
	pub fn read_image_file(bitmap_path : &str) -> Bitmap
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_read_image_file_str(bitmap_path.as_ptr(),bitmap_path.len(),false); }
		Bitmap::create(ret_val)
	}

    /// [**read_image_file32()**] - Reads an image file and returns a 32-bit RGB bitmap in a Bitmap object. 
    /// 
    /// - Input files may be **.JPG, .BMP, or .PNG.** 
	/// - See <i>**read_image_file()**</i> to load files without alpha channels or transparencies into a 24-bit bitmap.
	///   - Most functions support 32-bitmaps directly, but there is some flexibility in using 24-bit bitmaps when there is no alpha-channel/mask in the image.
    /// - Bitmaps are returned in reverse Y order unless the <i>**kw::reverse()**</i> keyword is used (<i>[not currently implemented]</i>)
	/// - 32-bit bitmaps may be blended into the output window with the <i>**win.blend_bitmap()**</i> function.
	///
	/// **Function Parameter Forms:**
	///
	///		read_image_file32(bitmap_path : &str)		// Reads the file specified by the string slice
	///
    /// **Notes:**
    /// 
    /// - return type: <i>**read_image_file32()**</i> returns a 32-Bit Bitmap object with the bitmap.  If the file was not found, the bitmap will be empty.
    /// - Use the Bitmap's <i>**is_valid()**</i> or <i>**bitmap.empty()**</i> function to determine if the bitmap was read ok.	
	///
	/// **Example:**
	///
	///			let my_bitmap = Sagebox::read_image_file32("myfile.png")	// Load a 32-bitmap from a file with a mask
	///			if my_bitmap.is_empty() { return err; }						// Form an error if the bitmap was not found
	///
	pub fn read_image_file32(bitmap_path : &str) -> Bitmap32
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_read_image_file_str(bitmap_path.as_ptr(),bitmap_path.len(),true); }
		Bitmap32::create(ret_val)
	}


	/// [**read_pgr_file()**] — Reads a .PGR file and returns a **`Pgr`** object to later retreive the information in the .PGR file
	///
	/// **About PGR Files**
	///
	/// PGR is a Portable Graphics Resource File compiled from a source .txt file similiar to JSON format.
	///
	/// - The use of .pgr files has extended into general resource storage
	/// - PGR files start as .txt definition files and are convert to verified .PGR files to data integrity and security
	//  - Once a .PGR file is loaded, it is guaranteed to be accurate and parseable. 
	/// - PGR files can store files as well as variable and array definitions
	/// - PGR support different sections. 
	///
	/// **Memory- and File-Based PGR files**
	///
	/// - PGR files can be loaded from disk or memory
	/// - When loaded from disk, since the PGR file is verified before it is compiled and as it is reads in, a simple check for
	///PGR integrity can suffice to ensure the integrity of the loaded data
	/// - With memory-based Files, the PGR file is always guaranteed once it has been verified one time.
	///
	/// - note: This does not include program-based errors with filename or variable definitions, which are text based.
	///
	/// **Uses of PGR files**
	///
	/// - PGR files are used for general resources, usually graphics resources — also general data abstracted from hard-coded programming.
	/// - This can abstract and offload values from the program to an external resource. 
	/// - For example, inclusion of control graphics and their preferred locations can be stored in the PGR file, abstracting them from code. 
	///
	/// **Example of PGR text file**
	///
	/// 		[MySection]                              // Start of a new section (sections are not required)
	/// 		    my_image         = <file>MyBitmap
	/// 		    image_location   = 100,50            // Put it the image at 100,50 in the window 
	/// 		    background_color = Black             // Text string for bacground color
	///                                                 // We can also use RGB here, such as 0,255,0 for BLUE
	/// 		[files]
	/// 		MyBitmap   = "path/my_bitmap.jpg"
	///
	/// **Example of using the above .TXT file after compiled to .PGR file**
	///
	///  		let pgr = read_pgr_file("my_pgr_file.pgr");   // We can use is_valid() to check it's integrity. 
	///                                                      // However, we don't need this here, because
	///                                                      // things will just fallthrough passively if they aren't found.
	///
	///  		let my_bitmap       = pgr.read_image_file("MySection:my_image");
	///  		let bitmap_location = pgr.read_pair_i32("MySection:image_location");
	///  		let color_string    = pgr.read_text("MySection:background_color");
	///
	pub fn read_pgr_file(pgr_path : &str) -> Pgr
	{
		let ret_val : i64;
		unsafe { ret_val = ext_func::sage_rust_read_pgr_file_str(pgr_path.as_ptr(),pgr_path.len()); }
		Pgr::create(ret_val)
	}
	fn _info_window_s(msg : &str,keyw : kw)
	{
		unsafe { ext_func::sage_rust_info_window(msg.as_ptr(),msg.len(),keyw.pointer); }
	}
	fn _info_window(msg : &str)
	{
		Self::_info_window_s(msg,KW_NONE);
	}

	/// [**show_debug_window()**] — Shows (or hides) the built-in Sagebox Debug Window
	///
	/// - The **`Sagebox Debug Window`** is very useful for debugging output, in Console Mode, Terminal, or Graphic programs
	///   - Since it is a pre-built window, it is very easy share debug information to without needing to clutter the Console Window
	///   - only **<i>Sagebox::write()</i>** functions are needed to use the debug window with no setup.
	/// - The debug window can be shown or hidden easily, and can be used to terminate the program. 
	/// - The debug window prints line numbers for each entry to distinguish between lines when the same text appears repeatedly. 
	///
	/// **Function Parameter Forms:**
	///
	///		show_debug_window()                       // Shows debug window    
	///		show_debug_window_s(show_window : bool)   // Shows or Hides debug window depending on show_window value
	///
	/// **Parameters**
	///
	/// - **`show_window`** - When **`true`** the debug window is shown.  When **`false`** the debug window is hidden. 
	///
	/// **Notes**
	///
	/// - The `Debug Window` can be dismissed or shown by moving the mouse to the upper-right-hand corner of the window.
	///   - When hidden, just move the mouse to the upper-right for it to appear
	///   - When showing, moving the mouse to the upper-right hides it. 
	/// - The `Debug Window` has a Help button to show instructions on how to use it more effectively.
	/// - The `Debug Window` responds to embedded colors, such as `{b}` or `{blue}` to set a blue text color
	/// - When using the **`format!()`** function, `{}` values must be enlcosed in double `{{}}`
	///   - e.g. `{{blue}}` instead of `{blue}`
	///
	/// **Examples**
	///
	///			Sagebox::debug_writeln("I made it to my_function()");
	///			Sagebox::debug_writeln("I made it to {g}my_function()"); // prints "my_function()" in green
	/// 		Sagebox::debug_writeln(format!("position = {{cyan}}{},{}",pos.x,pos.y))); 
	///
	pub fn show_debug_window_s(show_window : bool)
	{
		unsafe { ext_func::sage_rust_show_debug_window(show_window); }
	}
	
	/// [**show_debug_window()**] — Shows (or hides) the built-in Sagebox Debug Window
	///
	/// - The **`Sagebox Debug Window`** is very useful for debugging output, in Console Mode, Terminal, or Graphic programs
	///   - Since it is a pre-built window, it is very easy share debug information to without needing to clutter the Console Window
	///   - only **<i>Sagebox::write()</i>** functions are needed to use the debug window with no setup.
	/// - The debug window can be shown or hidden easily, and can be used to terminate the program. 
	/// - The debug window prints line numbers for each entry to distinguish between lines when the same text appears repeatedly. 
	///
	/// **Function Parameter Forms:**
	///
	///		show_debug_window()                       // Shows debug window    
	///		show_debug_window_s(show_window : bool)   // Shows or Hides debug window depending on show_window value
	///
	/// **Parameters**
	///
	/// - **`show_window`** - When **`true`** the debug window is shown.  When **`false`** the debug window is hidden. 
	///
	/// **Notes**
	///
	/// - The `Debug Window` can be dismissed or shown by moving the mouse to the upper-right-hand corner of the window.
	///   - When hidden, just move the mouse to the upper-right for it to appear
	///   - When showing, moving the mouse to the upper-right hides it. 
	/// - The `Debug Window` has a Help button to show instructions on how to use it more effectively.
	/// - The `Debug Window` responds to embedded colors, such as `{b}` or `{blue}` to set a blue text color
	/// - When using the **`format!()`** function, `{}` values must be enlcosed in double `{{}}`
	///   - e.g. `{{blue}}` instead of `{blue}`
	///
	/// **Examples**
	///
	///			Sagebox::debug_writeln("I made it to my_function()");
	///			Sagebox::debug_writeln("I made it to {g}my_function()"); // prints "my_function()" in green
	/// 		Sagebox::debug_writeln(format!("position = {{cyan}}{},{}",pos.x,pos.y))); 
	///
	pub fn show_debug_window() { Self::show_debug_window_s(true); }

	/// [**get_open_filename()**] — Opens an OS-based dialog box for the user to select a filename from disk. 
	///
	/// - An OS-based Open-File dialog box is launched given a selection of system directories and files.
	///
	/// **Return Values**
	///
	/// - When selection, **`Some(String)`** is returned. 
	/// - If the user cancels the box or returns with no selection, an **`Empty String`** is returned.
	///
	/// **Function Parameter Forms:**
	///
	///			get_open_filename()                       
	///			get_open_filename_s(file_types : &str)   
	///
	/// **Parameters**
	///
	/// **file_types** 	- when this is used, this contains file type that restricts the type of files
	///                 - if not used, all files may be selected
	///				    - example **<i>"&ast;.jpg;&ast;.bmp"</i>** restricts the file types to JPEG and Bitmap images.
	///
	/// **Examples**
	///
	/// 	let filename = get_open_filename()  // Get a filename of any type
	///
	///		let filename = get_open_filename("*.jpg;*.bmp;*.png")  // restrict to image file types
	///		
	///			if filename.is_some()
	///			{
	///				/* process file here */
	///			}
	///
	///
	pub fn get_open_filename() -> Option<String>
	{
		Sagebox::get_open_filename_s("")
	}


	/// [**get_open_filename()**] — Opens an OS-based dialog box for the user to select a filename from disk. 
	///
	/// - An OS-based Open-File dialog box is launched given a selection of system directories and files.
	///
	/// **Return Values**
	///
	/// - When selection, **`Some(String)`** is returned. 
	/// - If the user cancels the box or returns with no selection, an **`Empty String`** is returned.
	///
	/// **Function Parameter Forms:**
	///
	///			get_open_filename()                       
	///			get_open_filename_s(file_types : &str)   
	///
	/// **Parameters**
	///
	/// **file_types** 	- when this is used, this contains file type that restricts the type of files
	///                 - if not used, all files may be selected
	///				    - example **<i>"&ast;.jpg;&ast;.bmp"</i>** restricts the file types to JPEG and Bitmap images.
	///
	/// **Examples**
	///
	/// 	let filename = get_open_filename()  // Get a filename of any type
	///
	///		let filename = get_open_filename("*.jpg;*.bmp;*.png")  // restrict to image file types
	///		
	///			if filename.is_some()
	///			{
	///				/* process file here */
	///			}
	///
	///
	pub fn get_open_filename_s(file_types : &str) -> Option<String> 
	{ 
		unsafe { let vector = ext_func::sage_rust_get_open_filename(file_types.as_ptr(),file_types.len(),false); 

		if vector.len <= 0 { return None; }

		Some(String::from_raw_parts(vector.data,vector.len,vector.len)) }

	}

	/// [**image_view()**] — Opens an Image View Window and display an image
	///
	/// - Multiple **`Image View`** windows may be open concurrently. 
	/// - The window may be placed in a modal state, waiting for input. 
	/// - Otherwise, program execution may continue with the window (or multiple **`Image View`** windows) displayed on the screen.
	///
	/// **Returns**
	///
	/// - An **`ImageView`** object is returned. 
	/// - This object may be ignored if it is not needed. 
	///   - Sagebox will close the window separately when the user closes it or the program ends. 
	/// - The returned ImageView object can be used to control the window, especially when multiple windows are opened. 
	/// - Program execution may continue when the window is opened — Sagebox takes over ownership of the window and creates as an indepent window
	///   - Sagebox will handle closing the window when the user dismisses it. 
	///   - The returned object may be used to have the program dismiss it manually. 
	///
	/// **Function Parameter Forms:**
	///
	///			image_view(bitmap : &Bitmap)                       
	///			image_view_s(bitmap : &Bitmap,keyw : kw)   
	///
	/// **Some Useful Keywords**
	///
	/// - **wait_for_close** - Waits for the user to close the window before returning. 
	/// - **zoom_box** - Shows the Zoom Box/Navigator window
	/// - **reverse** - States that the Bitmap is upside-down
	/// - **maximize** - Maximizes the display window
	/// - **size**  - Sets the size of the display window
	/// - **pos** - Sets the position of the display window
	///
	/// ### About ImageView Functions
	///
	/// Sagebox ImageView functions (e.g. image_view(), image_view_before_after()) allow an easy, but powerful way to 
	/// look at image data, which can be: 
	///
	///   - Loaded or derived Bitmaps
	///   - Any image in Bitmap or Vec<> format
	///     Vector formats can be u8, f32, i32, (u8,u8,u8), (f32,f32,f32), (i32,i32,i32), or
	///     32-bit equivalents, such as (u8,u8,u8,u8), where the last component is an alpha channel.
	///
	/// The Image View functions are meant to be generalized function supporting in-program created content (e.g. a Vec<> of just about any type), or a loaded
	/// (and possibly altereed) image.
	///
	/// ### Powerful Functions in one line of code
	///
	/// It just takes one line of code to activate and use an Image View Window, which can do the following: 
	///
	/// - The Image View functions include the ablity to resize the image, zoom in and out, and move the image about in the window.
	/// - A navigator (eka. Zoom Box) window can be called up within the window (or dispayed with a keyword in the image_view() call) to help 
	///   moving about in the window
	/// - Multiple Image View windows may be opened at one time.
	/// - WIndows are not required to be modal - the Sagebox internal system manages the windows independently and will close them when the user
	///   closes them or the program ends. 
	/// - An object is returned.  It is not required to save the object, as the Image View windows survive object deletion.
	/// - With the return object, more power is handed to the program to close, hide, update, or move the window.
	/// - Captions and titles may be added to the images in different font styles.
	///
	pub fn image_view_s(bitmap : &Bitmap,keyw : kw) -> ImageView
	{
		unsafe { ImageView { id: ext_func::sage_rust_image_view(bitmap.get_id(),0,keyw.pointer,false,false), is_ba : false } }
	}


	/// [**image_view()**] — Opens an Image View Window and display an image
	///
	/// - Multiple **`Image View`** windows may be open concurrently. 
	/// - The window may be placed in a modal state, waiting for input. 
	/// - Otherwise, program execution may continue with the window (or multiple **`Image View`** windows) displayed on the screen.
	///
	/// **Returns**
	///
	/// - An **`ImageView`** object is returned. 
	/// - This object may be ignored if it is not needed. 
	///   - Sagebox will close the window separately when the user closes it or the program ends. 
	/// - The returned ImageView object can be used to control the window, especially when multiple windows are opened. 
	/// - Program execution may continue when the window is opened — Sagebox takes over ownership of the window and creates as an indepent window
	///   - Sagebox will handle closing the window when the user dismisses it. 
	///   - The returned object may be used to have the program dismiss it manually. 
	///
	/// **Function Parameter Forms:**
	///
	///			image_view(bitmap : &Bitmap)                       
	///			image_view_s(bitmap : &Bitmap,keyw : kw)   
	///
	/// **Some Useful Keywords**
	///
	/// - **wait_for_close** - Waits for the user to close the window before returning. 
	/// - **zoom_box** - Shows the Zoom Box/Navigator window
	/// - **reverse** - States that the Bitmap is upside-down
	/// - **maximize** - Maximizes the display window
	/// - **size**  - Sets the size of the display window
	/// - **pos** - Sets the position of the display window
	///
	/// ### About ImageView Functions
	///
	/// Sagebox ImageView functions (e.g. image_view(), image_view_before_after()) allow an easy, but powerful way to 
	/// look at image data, which can be: 
	///
	///   - Loaded or derived Bitmaps
	///   - Any image in Bitmap or Vec<> format
	///     Vector formats can be u8, f32, i32, (u8,u8,u8), (f32,f32,f32), (i32,i32,i32), or
	///     32-bit equivalents, such as (u8,u8,u8,u8), where the last component is an alpha channel.
	///
	/// The Image View functions are meant to be generalized function supporting in-program created content (e.g. a Vec<> of just about any type), or a loaded
	/// (and possibly altereed) image.
	///
	/// ### Powerful Functions in one line of code
	///
	/// It just takes one line of code to activate and use an Image View Window, which can do the following: 
	///
	/// - The Image View functions include the ablity to resize the image, zoom in and out, and move the image about in the window.
	/// - A navigator (eka. Zoom Box) window can be called up within the window (or dispayed with a keyword in the image_view() call) to help 
	///   moving about in the window
	/// - Multiple Image View windows may be opened at one time.
	/// - WIndows are not required to be modal - the Sagebox internal system manages the windows independently and will close them when the user
	///   closes them or the program ends. 
	/// - An object is returned.  It is not required to save the object, as the Image View windows survive object deletion.
	/// - With the return object, more power is handed to the program to close, hide, update, or move the window.
	/// - Captions and titles may be added to the images in different font styles.
	///
	pub fn image_view(bitmap : &Bitmap) -> ImageView
	{
		Sagebox::image_view_s(bitmap,KW_NONE) 
	}

	/// [**image_view_before_after()**] — Opens an Image View Window and display two images: a **`Before`** and **`After`** image for comparison
	///
	/// - Multiple **`Image View`** windows may be open concurrently. 
	/// - This function will return an empty **`Image View Object`** if the before and after images are not the same size.
	///   - call **<i>is_valid()</i>** to ensure the images are displayed.
	///   - image_view_before_after() falls through passively when the images are mismatched.
	/// - The window may be placed in a modal state, waiting for input. 
	/// - Otherwise, program execution may continue with the window (or multiple **`Image View`** windows) displayed on the screen.
	///
	/// **Returns**
	///
	/// - An **`ImageView`** object is returned. 
	/// - This object may be ignored if it is not needed. 
	///   - Sagebox will close the window separately when the user closes it or the program ends. 
	/// - The returned ImageView object can be used to control the window, especially when multiple windows are opened. 
	/// - Program execution may continue when the window is opened — Sagebox takes over ownership of the window and creates as an indepent window
	///   - Sagebox will handle closing the window when the user dismisses it. 
	///   - The returned object may be used to have the program dismiss it manually. 
	///
	/// **Function Parameter Forms:**
	///
	///			image_view_before_after(bitmap : &Bitmap)                       
	///			image_view_before_after_s(bitmap : &Bitmap,keyw : kw)   
	///
	/// **Some Useful Keywords**
	///
	/// - **wait_for_close** - Waits for the user to close the window before returning. 
	/// - **zoom_box** - Shows the Zoom Box/Navigator window
	///                  - With Before and Afer Windows, the Zoom Box comes up automatically
	/// 				 - use zoom_box_s() to keep the Zoom Box from displaying.
	/// - **reverse** - States that the Bitmap is upside-down
	/// - **maximize** - Maximizes the display window
	/// - **size**  - Sets the size of the display window
	/// - **pos** - Sets the position of the display window
	///
	/// ### About ImageView Functions
	///
	/// Sagebox ImageView functions (e.g. <i>image_view(), image_view_before_after()) allow an easy, but powerful way to 
	/// look at image data, which can be: 
	///
	///   - Loaded or derived Bitmaps
	///   - Any image in Bitmap or Vec<> format
	///     Vector formats can be u8, f32, i32, (u8,u8,u8), (f32,f32,f32), (i32,i32,i32), or
	///     32-bit equivalents, such as (u8,u8,u8,u8), where the last component is an alpha channel.
	///
	/// The Image View functions are meant to be generalized function supporting in-program created content (e.g. a Vec<> of just about any type), or a loaded
	/// (and possibly altereed) image.
	///
	/// ### Powerful Functions in one line of code
	///
	/// It just takes one line of code to activate and use an Image View Window, which can do the following: 
	///
	/// - The Image View functions include the ablity to resize the image, zoom in and out, and move the image about in the window.
	/// - A navigator (eka. Zoom Box) window can be called up within the window (or dispayed with a keyword in the image_view() call) to help 
	///   moving about in the window
	/// - Multiple Image View windows may be opened at one time.
	/// - WIndows are not required to be modal - the Sagebox internal system manages the windows independently and will close them when the user
	///   closes them or the program ends. 
	/// - An object is returned.  It is not required to save the object, as the Image View windows survive object deletion.
	/// - With the return object, more power is handed to the program to close, hide, update, or move the window.
	/// - Captions and titles may be added to the images in different font styles.
	///
	pub fn image_view_before_after_s(before : &Bitmap,after : &Bitmap,keyw : kw) -> ImageView
	{
		unsafe { ImageView { id: ext_func::sage_rust_image_view(before.get_id(),after.get_id(),keyw.pointer,false,true), is_ba : true } }
	}

	/// [**image_view_before_after()**] — Opens an Image View Window and display two images: a **`Before`** and **`After`** image for comparison
	///
	/// - Multiple **`Image View`** windows may be open concurrently. 
	/// - This function will return an empty **`Image View Object`** if the before and after images are not the same size.
	///   - call **<i>is_valid()</i>** to ensure the images are displayed.
	///   - image_view_before_after() falls through passively when the images are mismatched.
	/// - The window may be placed in a modal state, waiting for input. 
	/// - Otherwise, program execution may continue with the window (or multiple **`Image View`** windows) displayed on the screen.
	///
	/// **Returns**
	///
	/// - An **`ImageView`** object is returned. 
	/// - This object may be ignored if it is not needed. 
	///   - Sagebox will close the window separately when the user closes it or the program ends. 
	/// - The returned ImageView object can be used to control the window, especially when multiple windows are opened. 
	/// - Program execution may continue when the window is opened — Sagebox takes over ownership of the window and creates as an indepent window
	///   - Sagebox will handle closing the window when the user dismisses it. 
	///   - The returned object may be used to have the program dismiss it manually. 
	///
	/// **Function Parameter Forms:**
	///
	///			image_view_before_after(bitmap : &Bitmap)                       
	///			image_view_before_after_s(bitmap : &Bitmap,keyw : kw)   
	///
	/// **Some Useful Keywords**
	///
	/// - **wait_for_close** - Waits for the user to close the window before returning. 
	/// - **zoom_box** - Shows the Zoom Box/Navigator window
	///                  - With Before and Afer Windows, the Zoom Box comes up automatically
	/// 				 - use zoom_box_s() to keep the Zoom Box from displaying.
	/// - **reverse** - States that the Bitmap is upside-down
	/// - **maximize** - Maximizes the display window
	/// - **size**  - Sets the size of the display window
	/// - **pos** - Sets the position of the display window
	///
	/// ### About ImageView Functions
	///
	/// Sagebox ImageView functions (e.g. <i>image_view(), image_view_before_after()) allow an easy, but powerful way to 
	/// look at image data, which can be: 
	///
	///   - Loaded or derived Bitmaps
	///   - Any image in Bitmap or Vec<> format
	///     Vector formats can be u8, f32, i32, (u8,u8,u8), (f32,f32,f32), (i32,i32,i32), or
	///     32-bit equivalents, such as (u8,u8,u8,u8), where the last component is an alpha channel.
	///
	/// The Image View functions are meant to be generalized function supporting in-program created content (e.g. a Vec<> of just about any type), or a loaded
	/// (and possibly altereed) image.
	///
	/// ### Powerful Functions in one line of code
	///
	/// It just takes one line of code to activate and use an Image View Window, which can do the following: 
	///
	/// - The Image View functions include the ablity to resize the image, zoom in and out, and move the image about in the window.
	/// - A navigator (eka. Zoom Box) window can be called up within the window (or dispayed with a keyword in the image_view() call) to help 
	///   moving about in the window
	/// - Multiple Image View windows may be opened at one time.
	/// - WIndows are not required to be modal - the Sagebox internal system manages the windows independently and will close them when the user
	///   closes them or the program ends. 
	/// - An object is returned.  It is not required to save the object, as the Image View windows survive object deletion.
	/// - With the return object, more power is handed to the program to close, hide, update, or move the window.
	/// - Captions and titles may be added to the images in different font styles.
	///
	pub fn image_view_before_after(before : &Bitmap,after : &Bitmap) -> ImageView
	{
		Sagebox::image_view_before_after_s(before,after,KW_NONE) 
	}

	pub fn __priv_get_index() -> SageDLLInit { unsafe { ext_func::sage_rust_get_sagebox_index() } }


}
impl SageStringFuncType<&str> for Sagebox
{
	fn console_write(msg : &str) -> bool { Sagebox::_console_write(msg) }
	fn console_writeln(msg : &str) -> bool { Sagebox::_console_writeln(msg) }
	fn debug_write(msg : &str) -> bool { Sagebox::_debug_write(msg) }
	fn debug_writeln(msg : &str) -> bool { Sagebox::_debug_writeln(msg) }
	fn info_window(msg : &str) { Sagebox::_info_window(msg) }
	fn info_window_s(msg : &str,keyw : kw) { Sagebox::_info_window_s(msg,keyw) }
}

impl SageStringFuncType<&String> for Sagebox
{
	fn console_write(msg : &String) -> bool { Sagebox::_console_write(msg.as_str()) }
	fn console_writeln(msg : &String) -> bool { Sagebox::_console_writeln(msg.as_str()) }
	fn debug_write(msg : &String) -> bool { Sagebox::_debug_write(msg.as_str()) }
	fn debug_writeln(msg : &String) -> bool { Sagebox::_debug_writeln(msg.as_str()) }
	fn info_window(msg : &String) { Sagebox::_info_window(msg.as_str()) }
	fn info_window_s(msg : &String,keyw : kw) { Sagebox::_info_window_s(msg.as_str(),keyw) }
}
impl SageStringFuncType<String> for Sagebox
{
	fn console_write(msg : String) -> bool { Sagebox::_console_write(msg.as_str()) }
	fn console_writeln(msg : String) -> bool { Sagebox::_console_writeln(msg.as_str()) }
	fn debug_write(msg : String) -> bool { Sagebox::_debug_write(msg.as_str()) }
	fn debug_writeln(msg : String) -> bool { Sagebox::_debug_writeln(msg.as_str()) }
	fn info_window(msg : String) { Sagebox::_info_window(msg.as_str()) }
	fn info_window_s(msg : String,keyw : kw) { Sagebox::_info_window_s(msg.as_str(),keyw) }
}


impl SageNewQuickForm<&str> for Sagebox
{
	fn quick_form(msg : &str) -> QuickForm { Sagebox::_quick_form(msg) }
	fn quick_form_s(msg : &str,keyw : kw ) -> QuickForm { Sagebox::_quick_form_s(msg,keyw) }
}


impl SageNewQuickForm<&String> for Sagebox
{
	fn quick_form(msg : &String) -> QuickForm { Sagebox::_quick_form(msg.as_str()) }
	fn quick_form_s(msg : &String,keyw : kw ) -> QuickForm { Sagebox::_quick_form_s(msg.as_str(),keyw) }
}

mod colors;
mod point;
mod keywords;
mod ext_func;

