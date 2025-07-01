

// -------------------------------------------------------------
// Sagebox Plug-in Widget Example - LCD Widget Library Interface 
// -------------------------------------------------------------
//
// This LCD Widget is an example of using plug-in widgets that anyone can write for Sagebox.
//
// This is the interface to the widget, which can be a .LIB or .DLL file, depending on the 'link-static' 
// feature set in the dependency in the calling cargo.toml file.
//
// The .LIB file is compiled as a separate library and then linked to and interface with this lib.rs file. 
//
// --------------------
// About the LCD Widget
// --------------------
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

#![allow(improper_ctypes)]	// So "not FFI-safe messages" do not appear
use sagebox::*; 

	// Define the external library functions in the LCD Widget

	mod ext_func
	{
	#[link(name = "lcd_widget", kind = "static")]	
	unsafe extern
	{
		pub unsafe fn rust_sage_widget_lcd_init(dll_init : sagebox::SageDLLInit,window_id : i64,x : i32, y : i32,initial_value : i32) -> i64;
		pub unsafe fn rust_sage_widget_lcd_delete(lcd_widget_id : i64);
		pub unsafe fn rust_sage_widget_lcd_set_value(lcd_widget_id : i64, value : i32); 
		pub unsafe fn rust_sage_widget_lcd_get_window_size(lcd_widget_id : i64) -> (i32,i32); 
		pub unsafe fn rust_sage_widget_lcd_set_display_mode(lcd_widget_id : i64,blue_led : bool);
		pub unsafe fn rust_sage_widget_lcd_set_fast_mode(lcd_widget_id : i64,fast_mode : bool);
		pub unsafe fn rust_sage_widget_lcd_update_last(lcd_widget_id : i64);

    }
    }
#[derive(PartialEq, Eq)]
pub enum LcdMode 
{
    PlainLcd,
    BlueLcd,
}

pub struct LcdWidget
{
	id : i64,
}

impl LcdWidget
{
	/// [default()] - returns an empty LCD Widget for use in intitial structures
	///             
	/// - Note that it calls the rust_sage_widget_lcd_init().
	///   - This is because the LCD DLL is invoked on the first instantiation of an LCDWidget (which is returned),
	///     and we want Sagebox to be initialized first. 
	///
	pub fn default() -> LcdWidget { unsafe { LcdWidget { id : ext_func::rust_sage_widget_lcd_init(Sagebox::__priv_get_index(),0,0,0,0) } } }

	// [set_value()] - Sets the value displayed on the LCD 
	//
	pub fn set_value(&self,value : i32)
	{
		unsafe {  ext_func::rust_sage_widget_lcd_set_value(self.id,value) };
	}

	/// [get_window_size()] - returns the window size of the LCD Widget created. 
	///
	/// - This value is useful when placing text or other things around the LCD window, or placing the LCD window in the parent window.
	///
	pub fn get_window_size(&self) -> (i32, i32)
	{
		unsafe {  ext_func::rust_sage_widget_lcd_get_window_size(self.id) }	
	}

	/// [set_display_mode()] - Sets the display mode between a normal LCD display and a Blue LED display.
	///
	pub fn set_display_mode(&self,mode : LcdMode)
	{
		unsafe {  ext_func::rust_sage_widget_lcd_set_display_mode(self.id, mode == LcdMode::BlueLcd); }
	}

	/// [set_fast_mode()] - Sets a 'fast' mode, which does not update the LCD on every set_value() call. 
	///
	/// - The default mode updates the LCD on very **`set_value()`** call.
	///   - In high speed loops, this can slow the process down, e.g. taking ~10 seconds for 1,000,000 updates (depending on program mode, etc.)
	/// - in **fast mode** the LCD is only updated every 16ms (i.e. 60fps) or so to keep a fluid display without needing to update on every set_value() call.
	///	  - This allows the same loop of 1,000,000 calls to take place in a matter of a ~100us or so
	///
	/// note:
	///
	/// - **`update_last()`** should be called after any loop is finished when using **fast mode**.
	///   - Since the fast mode does not update on every `set_value()` call, and has no way to know when the loop is finished,
	///      it may not have updated the last value.  
	///	  - Using **`update_last()`** ensures the last known value is up-to-date on the LCD screen
	///
	pub fn set_fast_mode(&self,fast_mode : bool)
	{
		unsafe {  ext_func::rust_sage_widget_lcd_set_fast_mode(self.id, fast_mode); }
	}

	/// [update_last()] - Updates the LCD to the last known value.  This is used when <i>**fast mode**</i> is active.
	///
	/// - See notes in set_fast_mode(). 
	///
	/// - **`update_last()`** should be called after any loop is finished when using **fast mode**.
	///   - Since the fast mode does not update on every `set_value()` call, and has no way to know when the loop is finished,
	///      it may not have updated the last value.  
	///	  - Using **`update_last()`** ensures the last known value is up-to-date on the LCD screen
	///
	pub fn update_last(&self)
	{
		unsafe {  ext_func::rust_sage_widget_lcd_update_last(self.id); }
	}
}
impl Drop for LcdWidget 
{
    fn drop(&mut self) 
	{
		unsafe { ext_func::rust_sage_widget_lcd_delete(self.id); }
    }
}


/// [new_popup()] - Creates a new popup-style LCD Window
///
/// This creates a sample lCD in a new parent window with a circuit-board background.
///
/// - This can be used for demonstration purposes, since it does not require a parent window be created first. 
//  
pub fn new_popup(location : (i32,i32), initial_value : i32) -> LcdWidget
{
    unsafe { LcdWidget { id : ext_func::rust_sage_widget_lcd_init(Sagebox::__priv_get_index(),0,location.0,location.1,initial_value) } }
}

/// [new()] - Creates and LCD widget and displays it at `location` in the parent window.
///
/// **Parameters**
///
/// - **location** - The location to place the LCD Widget window in the parent window
/// - **initial_value** - The initial values to set in the LCD Widget display
///
pub fn new(win : &Window,location : (i32,i32), initial_value : i32) -> LcdWidget
{
    unsafe { LcdWidget { id : ext_func::rust_sage_widget_lcd_init(Sagebox::__priv_get_index(),win.get_id(),location.0,location.1,initial_value) } }
}
