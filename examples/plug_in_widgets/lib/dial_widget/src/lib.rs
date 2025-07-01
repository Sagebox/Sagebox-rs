

// --------------------------------------------------------------
// Sagebox Plug-in Widget Example - Dial Widget Library Interface 
// --------------------------------------------------------------
//
// This Dial Widget is an example of using plug-in widgets that anyone can write for Sagebox.
//
// This is the interface to the widget, which can be a .LIB or .DLL file, depending on the 'link-static' 
// feature set in the dependency in the calling cargo.toml file.
//
// The .LIB file is compiled as a separate library and then linked to and interface with this lib.rs file. 
//
// ---------------------------------
// A Good example of General Widgets
// ---------------------------------
//
// While the Dial Widget was written as a specific thermostate-like tool, it's also a good example of 
// writing widgets for any use, due to its usefulness in using a radial dial.
//
// This widget, for example, can be adapted to a generic dial-based slider with a central display that
// can be used for general purposes, rather specifically for emulating a hardware-based thermostat
//
// ---------------------
// About the Dial Widget
// ---------------------
//
// The Dial Widget is an example of an emulated interface to a physical hardware control such as a
// wall thermostat controlling the temperature to a heater or some other device.
//
// The Dial Widget is an example of a more complex widget that has a graphic display and changes based
// on mouse movements (setting the temperature) as well as button clicks. 
//
// The Dial Widget does the following: 
//
//	- Allows the mouse to set the temperture in a radial mannter by moving the small knob in the dial/thermostat.
//  - Allows the temperature to be set in increments by pressing the '<' and '> buttons with the mouse.
//  - Development Debug Modes - Allows the user to enter a debug mode by right-clicking the mouse: 
//    - ** Document modes here **
//
// A more extensive Dial widget in a real-world scenario would be able to send signals directly to the hardware, as well as receive feedback 
// about actual temperature vs. the temperature set on the dial. 
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
// Sagebox input functions using the mouse, mouse clicks (both left and right), as well as debug code is also used in this
// example.

#![allow(improper_ctypes)]	// So "not FFI-safe messages" do not appear
use sagebox::*;

	// Define external functions in the lirary (.lib) file 

	mod ext_func
	{
		#[link(name = "dial_widget", kind = "static")]	
		unsafe extern
		{
			pub unsafe fn rust_sage_widget_dial_init(dll_init : sagebox::SageDLLInit,window_id : i64,x : i32, y : i32) -> i64;
			pub unsafe fn rust_sage_widget_dial_delete(dial_widget_id : i64);
			pub unsafe fn rust_sage_widget_dial_get_raw_value(dial_id : i64) -> i32;
			pub unsafe fn rust_sage_widget_dial_get_raw_range(dial_id : i64) -> (i32,i32);
			pub unsafe fn rust_sage_widget_dial_get_value(dial_id : i64) -> f32;

			pub unsafe fn rust_sage_widget_dial_value_changed(dial_id : i64,peek : bool) -> bool;
			pub unsafe fn rust_sage_widget_dial_set_range(dial_id : i64,min : f32,max : f32);
			pub unsafe fn rust_sage_widget_dial_set_value_f32(dial_id : i64,value : f32);
			pub unsafe fn rust_sage_widget_dial_show_fraction(dial_id : i64,show_fraction : bool);
			pub unsafe fn rust_sage_widget_dial_set_location(dial_id : i64,x : i32, y : i32);
			pub unsafe fn rust_sage_widget_dial_get_window_size(dial_id : i64) -> (i32,i32);
			pub unsafe fn rust_sage_widget_dial_get_location(dial_id : i64) -> (i32,i32);
			pub unsafe fn rust_sage_widget_dial_clone(dial_id : i64);
		}
    }


pub struct DialWidget
{
	id : i64,
}

impl DialWidget
{

	/// [get_raw_value()] - Retreives the hardware's actual value vs. the range set for the dial
	///
	/// - The raw value represents a value being retreived from the heater's hardware, which is then converted into 
	///   the dial's set range. 
	///   - For example, a dial range might be 50-100, with in internal hardware (i.e. raw) range of 0-65535. 
	///
	pub fn get_raw_value(&self) -> i32 { unsafe { ext_func::rust_sage_widget_dial_get_raw_value(self.id) } }

	/// [get_raw_range()] - Retreives the hardware's actual value vs. the range set for the dial
	///
	/// - The raw range value represents a range being retreived from the heater's hardware, which is then converted into 
	///   the dial's set range. 
	///   - For example, a dial range might be 50-100, with in internal hardware (i.e. raw) range of 0-65535. 
	///
	pub fn get_raw_range(&self) -> (i32,i32) { unsafe { ext_func::rust_sage_widget_dial_get_raw_range(self.id) } }

	/// [get_value()] - Get the current value of the dial
	///
	/// - This returns the value based on the range set with the **`set_range()`** function (or default range)
	/// - The range is the dial's range.  
	/// - **`get_raw_range()`** and **`get_raw_value()`** to get values from the heater source itself. 
	/// 
	/// note: 
	///
	/// - This function returns a floating-point, f32 value. 
	/// - see **`get_value_i32()`** to return an integer, i32 value.
	///
	pub fn get_value(&self) -> f32 { unsafe { ext_func::rust_sage_widget_dial_get_value(self.id) } }



	/// [get_value()] - Get the current value of the dial
	///
	/// - This returns the value based on the range set with the **`set_range()`** function (or default range)
	/// - The range is the dial's range.  
	/// - **`get_raw_range()`** and **`get_raw_value()`** to get values from the heater source itself. 
	/// 
	/// note: 
	///
	/// - This function returns a integer, i32 value. 
	/// - see **`get_value()`** to return an floating-point, f32 value.
	pub fn get_value_i32(&self) -> i32 { unsafe { ext_func::rust_sage_widget_dial_get_value(self.id) as i32 } }


    /// [**value_changed()**] - Returns **`true`** if the dial value changed, either through moving the dial knob or clicking the '<' or '>' buttons
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			value_changed()
	///			value_changed_peek()
	///
	/// <u><i>**value_changed_peek()**</i></u>
	///
	/// - When this form is used, this will return the `value changed` status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**value_changed_peek()**</i> is used without peeking, the `value changed` status is reset until the next time the causes a
	/// change to the displayed value.
	///
	pub fn value_changed(&self) -> bool { unsafe { ext_func::rust_sage_widget_dial_value_changed(self.id,false) } }

    /// [**value_changed()**] - Returns **`true`** if the dial value changed, either through moving the dial knob or clicking the '<' or '>' buttons
	///
	///	- [**note:**] This function is an <u><i>event function</i></u>: This function will only return **`true`**  <i>one time per-event and subsquent call to this function.</i>
	///   - as an <i>event function</i>, it will return **`false`** after the first call until the event occurs again, unless a <i>peek form</i> is used to prevent resetting it's event status.. 
	///
	/// **Function Parameter Forms:** 
	///	
	///			value_changed()
	///			value_changed_peek()
	///
	/// <u><i>**value_changed_peek()**</i></u>
	///
	/// - When this form is used, this will return the `value changed` status, but will not reset it if it returns true. 
	///   - (future calls will still return true until called without peeking)
	/// - Otherwise, when <i>**value_changed_peek()**</i> is used without peeking, the `value changed` status is reset until the next time the causes a
	/// change to the displayed value.
	///
	pub fn value_changed_peek(&self) -> bool { unsafe { ext_func::rust_sage_widget_dial_value_changed(self.id,true) } }

	/// [set_range()] - Sets the range of dial display.
	///
	/// - This sets the range of the dial itself, which is then translated internally to the hardware's range
	///
	/// - See **`set_range_f32()`** to set a range using floating-point values
	///
	pub fn set_range(&self,range : (i32,i32)) { unsafe { ext_func::rust_sage_widget_dial_set_range(self.id,range.0 as f32,range.1 as f32); } }

	/// [set_range_f32()] - Sets the range of dial display.
	///
	/// - This sets the range of the dial itself, which is then translated internally to the hardware's range
	///
	/// - See **`set_range()`** to set a range using integer, i32 values.
	///
	pub fn set_range_f32(&self,range : (f32,f32)) { unsafe { ext_func::rust_sage_widget_dial_set_range(self.id,range.0,range.1); } }

	/// [set_value_f32()] - Sets the value of the dial, which is then changed on the display. 
	///
	/// - This sets the value of the dial within the dial's range, which is then translated to the
	///   hardware's raw value and sent to the external hardware (e.g. heater).
	///
	/// - See **`set_value`** to set the value using an integer, i32 input.
	///
	pub fn set_value_f32(&self,value : f32) { unsafe { ext_func::rust_sage_widget_dial_set_value_f32(self.id,value); } }

	/// [set_value()] - Sets the value of the dial, which is then changed on the display. 
	///
	/// - This sets the value of the dial within the dial's range, which is then translated to the
	///   hardware's raw value and sent to the external hardware (e.g. heater).
	///
	/// - See **`set_value_f32`** to set the value using an floaing-point, f32 input.
	///
	pub fn set_value(&self,value : i32) { unsafe { ext_func::rust_sage_widget_dial_set_value_f32(self.id,value as f32); } }

	/// [show_fraction()] - Shows the fractional component of the value in the dial display.
	///
	/// - By default, the value is displayed rounded down to the nearest integer. 
	/// - Setting `show_fration` to `true` will show the fractional component of the dial's value.
	/// - This does not affect the value calculated and sent to the hardware (e.g. heater component)
	///
	pub fn show_fraction(&self,show_fraction : bool) { unsafe { ext_func::rust_sage_widget_dial_show_fraction(self.id,show_fraction); } }

	/// [set_location()] - Sets the location of the dial window in the parent window
	/// 
	/// - The dial window places itself in the parent's window when it is created.
	/// - **`set_location` can be used to move the location of the dial in the parent window. 
	///	- see **`get_window_size()` to get the size of the dial window to help determine the location in the parent window
	///
	pub fn set_location(&self,location : (i32,i32)) { unsafe { ext_func::rust_sage_widget_dial_set_location(self.id, location.0,location.1); } }

	/// [get_window_size()] - Returns the size of the dial window in the Parent Window
	///
	/// - This function is useful placing the dial window in the parent window.
	///
	pub fn get_window_size(&self) -> (i32,i32) { unsafe { ext_func::rust_sage_widget_dial_get_window_size(self.id) } }

	/// [get_location()] - Returns the current location of the dial window in the Parent Window
	///
	pub fn get_location(&self) -> (i32,i32) { unsafe { ext_func::rust_sage_widget_dial_get_location(self.id) } }

	/// [default()] - returns an empty Dial Widget for use in intitial structures
	///             
	/// - Note that it calls the **'rust_sage_widget_dial_init()`**.
	///   - This is because the Dial DLL is invoked on the first instantiation of an Dial Widget (which is returned),
	///     and we want Sagebox to be initialized first. 
	///
	pub fn default() -> DialWidget {unsafe { DialWidget { id : ext_func::rust_sage_widget_dial_init(Sagebox::__priv_get_index(),0,0,0) } } }
}
impl Drop for DialWidget 
{
    fn drop(&mut self) 
	{

	//	if self.id == 0 { println!("dropping (null) dial widget"); }
	//	else { println!("dropping dial widget"); }

		unsafe { ext_func::rust_sage_widget_dial_delete(self.id); }
    }
}
impl Clone for DialWidget {
    fn clone(&self) -> DialWidget {
	//	println!("cloned dial widget");
		unsafe { ext_func::rust_sage_widget_dial_clone(self.id); }
        DialWidget { id: self.id }
    }
}

/// [new()] - Creates and Dial widget and displays it at `location` in the parent window.
///
/// - See **`set_range()`** and **`set_value()`** to set the initial dial display. 
///
pub fn new(win : &Window,location : (i32,i32)) -> DialWidget
{
    unsafe { DialWidget { id : ext_func::rust_sage_widget_dial_init(Sagebox::__priv_get_index(),win.get_id(),location.0,location.1) } }
}
