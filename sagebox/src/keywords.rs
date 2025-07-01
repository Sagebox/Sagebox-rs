use super::{Point,ext_func,RgbColor,RgbColorA};


	pub trait SageKwSingleColor<ColorType>
	{
		/// bg_color() -- Sets the background color of the window, control or other object.
		/// 
		/// Function Parameter Forms: 
		///
		///		bg_color(color : RgbColor)          // RgbColor type value to set bg color, e.g. RgbColor(0,255,0), pan_color::Beige, etc.
		///		bg_color(color : (i32,i32,i32))     // Rgb Color value to set bg color, e.g. (0,255,0)
		///		bg_color(color : RgbColorA)         // RgbA Color value to set bg color, e.g. RgbColorA(0,255,0,128), pan_color_a::Beige(128), etc.
		///		bg_color(color : (i32,i32,i32,i32)) // RgbA Color value to set bg color
		///		bg_color(color : &str)              // &str value to set bg color, e.g. "cyan", "cyan(128)"", "red,blue", etc.
		///		bg_color(color : &String))          // &String value to set bg color (same form as &str) 
		///	
		/// Examples:
		///
		/// 	let win = Sagebox::new_window_s(kw::bg_color("green"));
		/// 	let win = Sagebox::new_window_s(kw::bg_color(pan_color::Green));
		/// 	let win = Sagebox::new_window_s(kw::bg_color(my_color_string));
		/// 	let text_widget = Sagebox::dev_text_widget_s("This is a Widget",kw::bg_color(0,255,0));
		/// 	let text_widget = Sagebox::dev_text_widget_s("This is a Widget",kw::font(25) + kw::bg_color(sage_color::Blue));
		///
		fn bg_color(_color : ColorType) -> kw;

//		fn fg_color(&self,_color : ColorType) -> kw;	--> To be implemented, along with a number of other similar keyword functions

		/// pen_color() -- Sets the pen color (i.e. line pixel width) when drawing shapes such as circles, polygons, rectangles, lines. etc.
		/// 
		/// Function Parameter Forms: 
		///
		///		pen_color(color : RgbColor)          // RgbColor type value for pen color, e.g. RgbColor(0,255,0), pan_color::Beige, etc.
		///		pen_color(color : (i32,i32,i32))     // Rgb Color value for pen color, e.g. (0,255,0)
		///		pen_color(color : RgbColorA)         // RgbA Color value for pen color, e.g. RgbColorA(0,255,0,128), pan_color_a::Beige(128), etc.
		///		pen_color(color : (i32,i32,i32,i32)) // RgbA Color value for pen color
		///		pen_color(color : &str)              // &str value for pen color, e.g. "cyan", "cyan(128)"", "red,blue", etc.
		///		pen_color(color : &String))          // &String value for pen color (same form as &str) 
		///	
		/// Examples:
		///
		///		win.draw_rectangle((x,y),(width,height),"green");              // Draws a rectangle with a green line color
		///		win.draw_rectangle((x,y),(width,height),pan_color::Green);     // Draws a rectangle with a pan_color::Green color
		///		win.draw_rectangle((x,y),(width,height),my_color_string);      // Draws a rectangle with the color in the &str or &String value
		///		win.draw_rectangle((x,y),(width,height),(0,255,0));            // Draws a rectangle with a Green RGB color
		///		win.draw_rectangle((x,y),(width,height),pan_color::Cyan(150)); // Draws a rectangle with a Cyan with opacity of 150 (out of 255)
 		///		win.draw_rectangle((x,y),(width,height),"Cyan(150)");          // Draws a rectangle with a Cyan with opacity of 150 (out of 255)
		///
		fn pen_color(_color : ColorType) -> kw;
	}
	pub trait SageKwDefault<T>
	{
		/// default() -- Sets the default for various types of controls, such as sliders, checkboxes, combo boxes, etc. 
		/// 
		/// Function Parameter Forms: 
		///
		///		default(default_value : i32)          // integer default value
		///		default(default_value : f32)          // floating-point default value
		///		default(default_value : bool)         // boolean default value
		///	
		/// Examples:
		///
		///		Sagebox::dev_Checkbox_s("This is a checkbox",kw::default(true));  // Creates a checkbox and sets the default to true
		///		Sagebox::dev_slider_s("This is a slider",kw::default(57));        // Creates a slider and sets the default value to 57
		///
		fn default(_default : T) -> kw;
	}

	pub trait SageKwRange<T>
	{
		/// range() -- Sets the range for various types of controls, such as sliders.
		/// 
		/// Function Parameter Forms: 
		///
		///		range(range_value : (i32,i32)       // integer pair range value
		///		range(range_value : (f32,f32)       // floating-point pair range value
		///		range(range_value : (Point<i32>)    // Point<i32> range value
		///		range(range_value : (Point<f32>)    // Point<f32> range value
		///	
		/// Examples:
		///
		///		Sagebox::dev_slider("This is a slider",kw::range(200,300));  // Creates a slider and sets the range to 200-300
		///		Sagebox::dev_slider_f("This is a slider",kw::range(.5,2.5)); // Creates a floating-point and sets the range to .5-2.5
		///
		fn range(range : T) -> kw;
	}
	pub trait SagekwPairTypei32<T>
	{
		/// size() -- Sets the size for various types of controls, such as windows, checkboxes, sliders, etc.
		/// 
		/// Function Parameter Forms: 
		///
		///		size(size_value : (i32,i32)       // integer pair size value
		///		size(size_value : (Point<i32>)    // Point<i32> size value
		///	
		/// Examples:
		///
		///		Sagebox::new_window_s(kw::size((1200,800));  // Creates a window and sets the size to 1200x800
		///
		fn size(win_size : T) -> kw;

		fn location(loc : T) -> kw;
		fn pos(loc : T) -> kw;
	}
	pub trait SageKwStringType<T>
	{
		fn set_caption(caption: T) -> kw;
		fn set_captions(caption: T) -> kw;
	}
	pub trait SageKwNumberType<T>
	{
		fn percent(_percent : T) -> kw;
	}
	#[allow(non_camel_case_types)]
	#[derive(Copy, Clone)]
	pub struct kw
	{
		pub (crate) pointer : i64,
	}

	impl kw
	{
		fn generic_bool(index : i32,value : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(index,value); }
			kw{pointer : ret_val}					
		}
		fn generic_string(index : i32,value : &str) -> kw
		{
			 unsafe { kw{ pointer : ext_func::rust_kw_generic_string(index,value.as_ptr(),value.len()) } }
		}

		fn generic_integer(index : i32,value : i32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_integer(index,value); }
			kw{pointer : ret_val}					
		}

		fn generic_float(index : i32,value : f32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_float(index,value); }
			kw{pointer : ret_val}					
		}

       	/// Sets the font for a text string.  If a number is used, it sets the font to the font size in the default font.
       	/// Otherwise, specific fonts may be given.
		/// 
		/// Function Parameter Forms:
		///
		///		font(font : i32)            // integer value to set font
		///		font(font: &str)            // string-slice/ascii string to set font
		///		font(font: &String)         // String to set font
		///			
		///	Examples:
		///
		/// 	kw::font(50);                // Set a 50-point font
       	///     kw::font("Arial,40");        // Set "arial,40" font
       	///     kw::font("Arial,35,italic"); // Set "Arial,35,italic" font
		///		kw::font("MyFont");          // Set a font previous registered with create_font() as "MyFont"
       	///
       	pub fn font(_font : i32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_integer(12,_font); }
			kw{pointer : ret_val}						
		}
		
       	/// Sets the font for a text string.  If a number is used, it sets the font to the font size in the default font.
       	/// Otherwise, specific fonts may be given.
		/// 
		/// Function Parameter Forms:
		///
		///		font(font : i32)            // integer value to set font
		///		font(font: &str)            // string-slice/ascii string to set font
		///		font(font: &String)         // String to set font
		///			
		///	Examples:
		///
		/// 	kw::font(50);                // Set a 50-point font
       	///     kw::font("Arial,40");        // Set "arial,40" font
       	///     kw::font("Arial,35,italic"); // Set "Arial,35,italic" font
		///		kw::font("MyFont");          // Set a font previous registered with create_font() as "MyFont"
       	///
		pub fn font_str(_font : &str) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_string(0,_font.as_ptr(),_font.len()); }
			kw{pointer : ret_val}						
		}
		
		/// angle() -- Sets the angle of an object such as a circle, square, ellipse, bitmap, etc. in degrees. 
		///
		/// - When objects are rotated, it is generally by the calculated center of the object. 
		/// - See the keyword <i>set_center()</i> to change the center of rotation. 
		/// - Transforms set ahead of the call can also be used, rather than using the keyword angle, to allow more flexibility in rotating an object
		///   or multiple objects with the same transform.
		///
		/// Function Parameter Forms:
		///
		///		angle(angle : f32)       // floating-point (f32) value to set angle
		///		angle_deg(angle : f32)   // same as angle(), just more specific to degrees, for clarity
		///		angle_rad(angle : f32)   // rotate by the angle in radians
		///			
		///	Example:
		///
		/// 	win.draw_rectangle((x,y,),(width,Height),"Green",kw::angle(45.0));	// Draw a green rectangle rotated at its center by 45 degrees 
       	///
		pub fn angle(angle : f32) -> kw { let ret_val : i64; unsafe { ret_val = ext_func::rust_kw_generic_float(5,angle); } kw{pointer : ret_val} }		

		/// angle_deg() -- Sets the angle of an object such as a circle, square, ellipse, bitmap, etc. in degrees.
		///
		/// - When objects are rotated, it is generally by the calculated center of the object. 
		/// - See the keyword <i>set_center()</i> to change the center of rotation. 
		/// - Transforms set ahead of the call can also be used, rather than using the keyword angle, to allow more flexibility in rotating an object
		///   or multiple objects with the same transform.
		///
		/// Function Parameter Forms:
		///
		///		angle_deg(angle : f32)   // floating-point (f32) value to set angle
		///		angle(angle : f32)       // same as angle_deg(), just more generic in its name.
		///		angle_rad(angle : f32)   // rotate by the angle in radians
		///			
		///	Example:
		///
		/// 	win.draw_rectangle((x,y,),(width,Height),"Green",kw::angle(45.0));	// Draw a green rectangle rotated at its center by 45 degrees 
       	///
		pub fn angle_deg(angle : f32) -> kw { let ret_val : i64; unsafe { ret_val = ext_func::rust_kw_generic_float(5,angle); } kw{pointer : ret_val} }		
		
		/// angle_rad() -- Sets the angle of an object such as a circle, square, ellipse, bitmap, etc. in radians.
		///
		/// - When objects are rotated, it is generally by the calculated center of the object. 
		/// - See the keyword <i>set_center()</i> to change the center of rotation. 
		/// - Transforms set ahead of the call can also be used, rather than using the keyword angle, to allow more flexibility in rotating an object
		///   or multiple objects with the same transform.
		///
		/// Function Parameter Forms:
		///
		///		angle_rad(angle : f32)   // rotate by the angle in radians
		///		angle(angle : f32)       // floating-point (f32) value to set angle
		///		angle_deg(angle : f32)   // same as angle(), just more specific to degrees, for clarity
		///			
		///	Example:
		///
		/// 	win.draw_rectangle((x,y,),(width,Height),"Green",kw::angle(45.0));	// Draw a green rectangle rotated at its center by 45 degrees 
       	///
		pub fn angle_rad(angle : f32) -> kw { let ret_val : i64; unsafe { ret_val = ext_func::rust_kw_generic_float(33,angle); } kw{pointer : ret_val} }		
		
		/// pen_size() -- Sets the pen size for a shape such as a circle, square, triangle, etc.  This is the thickness of the border around the shape.
		/// - When <i>pen_size()</i> is not used, the current default <i>pen size</i> setting is used (it defaults to 1, but can be changed via <i>win.set_pen_size()</i>)
		///
		/// Function Parameter Forms:
		///
		///		pen_size(pen_size : f32)  // Sets pen size using floating-point (f32) value
		///			
		///	Example:
		///
		/// 	win.draw_rectangle((x,y,),(width,Height),"Green",kw::pen_size(10.0));	// Draw a green rectangle with a width of 10 for the lines drawn (i.e. pen size of 10) 
       	///
		pub fn pen_size(pen_size : f32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_float(4,pen_size); }
			kw{pointer : ret_val}					
		}		

		/// opacity() -- Sets the opacity of an object or control, from 0 (invisible) - 255 (completely opaque).
		///
		/// - Drawn shapes such as rectangle, polygons, etc. may be semi-transparent by using the keyword <i>opacity()</i>
		///
		/// Function Parameter Forms:
		///
		///		opacity(opacity : i32)  // Sets opacity of a drawn object using floating-point (f32) value.
		///			
		///	Example:
		///
		/// 	win.draw_rectangle((x,y,),(width,Height),"Green",kw::opacity(64));	// Draw a green rectangle with a low opacity/high-transparency value. 
       	///
		pub fn opacity(opacity : i32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_integer(29,opacity); }
			kw{pointer : ret_val}					
		}	

		/// pad_y() -- Add (y) pixels to the size or placement of a control or other object.
		///
		/// - <i>pad_y()</i> can be used to change the location of justified items.
		/// - <i>pad_y()</i> can also be use to adjust the position of automaitcally placed controls.
		/// - Values may be positive or negative.
		///
		/// Function Parameter Forms:
		///
		///		pad_y(amount : i32)  // Sets the vertical pixel adjustment for control or object.
		///			
		///	Examples:
		///
		/// 	win.display_bitmap_s(0,0,my_bitmap,kw::top_center() + kw::pad_y(20))	// Displays bitmap at top-center of window, and also adds 20 pixels.
		///		let my_checkbox = Sagebox::dev_checkbox_s("this is a checkbox",kw::pad_y(-10)); // Creates a checkbox, with pad_y(-10) bringing up the automatic placement by 10 pixels.
       	///
		pub fn pad_y(amount : i32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_integer(7,amount); }
			kw{pointer : ret_val}		
		}

		pub fn pad_x(amount : i32) -> kw { unsafe { kw { pointer : ext_func::rust_kw_generic_integer(6,amount) } } }

		/// Sets the text or object in the center-top of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - center_x();                     // Set to horizontal center (true as default)
		///     - center_x_s(value : bool);       // Set to horizontal center if value is true; otherwise it is ignored.
		///     - just_center_x();                // Same as center_x()
		///     - just_center_x_s(value : bool);  // Same as center_x_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn center_x_s(_centerx : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(24,_centerx); }
			kw{pointer : ret_val}		
		}

		/// Sets the text or object in the center-top of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - center_x();                     // Set to horizontal center (true as default)
		///     - center_x_s(value : bool);       // Set to horizontal center if value is true; otherwise it is ignored.
		///     - just_center_x();                // Same as center_x()
		///     - just_center_x_s(value : bool);  // Same as center_x_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn center_x() -> kw { kw::center_x_s(true) }

		/// Sets the text or object in the center-top of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - center_x();                     // Set to horizontal center (true as default)
		///     - center_x_s(value : bool);       // Set to horizontal center if value is true; otherwise it is ignored.
		///     - just_center_x();                // Same as center_x()
		///     - just_center_x_s(value : bool);  // Same as center_x_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn just_center_x_s(_centerx : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(24,_centerx); }
			kw{pointer : ret_val}		
		}

		/// Sets the text or object in the center-top of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - center_x();                     // Set to horizontal center (true as default)
		///     - center_x_s(value : bool);       // Set to horizontal center if value is true; otherwise it is ignored.
		///     - just_center_x();                // Same as center_x()
		///     - just_center_x_s(value : bool);  // Same as center_x_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn just_center_x() -> kw { kw::center_x_s(true) }
	
		/// Sets the text or object in the center-top of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - top_center();                     // Set to top-center (true as default)
		///     - top_center_s(value : bool);       // Set to top-center if value is true; otherwise it is ignored.
		///     - just_top_center();                // Same as top_center()
		///     - just_top_center_s(value : bool);  // Same as top_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn just_top_center_s(_justtopcenter : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(20,_justtopcenter); }
			kw{pointer : ret_val}		
		}

		/// Sets the text or object in the center-top of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - top_center();                     // Set to top-center (true as default)
		///     - top_center_s(value : bool);       // Set to top-center if value is true; otherwise it is ignored.
		///     - just_top_center();                // Same as top_center()
		///     - just_top_center_s(value : bool);  // Same as top_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn just_top_center() -> kw { kw::just_top_center_s(true) }
	
		/// Sets the text or object in the top-center of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - top_center();                     // Set to top-center (true as default)
		///     - top_center_s(value : bool);       // Set to top-center if value is true; otherwise it is ignored.
		///     - just_top_center();                // Same as top_center()
		///     - just_top_center_s(value : bool);  // Same as top_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		pub fn top_center_s(_justtopcenter : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(20,_justtopcenter); }
			kw{pointer : ret_val}		
		}

		/// Sets the text or object in the top-center of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - top_center();                     // Set to top-center (true as default)
		///     - top_center_s(value : bool);       // Set to top-center if value is true; otherwise it is ignored.
		///     - just_top_center();                // Same as top_center()
		///     - just_top_center_s(value : bool);  // Same as top_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		///
		pub fn top_center() -> kw { kw::top_center_s(true) }

		/// Sets the text or object in the bottom-center of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - bottom_center();                     // Set to bottom-center (true as default)
		///     - bottom_center_s(value : bool);       // Set to bottom-center if value is true; otherwise it is ignored.
		///     - just_bottom_center();                // Same as bottom_center()
		///     - just_bottom_center_s(value : bool);  // Same as bottom_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		///
		pub fn just_bottom_center_s(_justtopcenter : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(21,_justtopcenter); }
			kw{pointer : ret_val}		
		}

		/// Sets the text or object in the bottom-center of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - bottom_center();                     // Set to bottom-center (true as default)
		///     - bottom_center_s(value : bool);       // Set to bottom-center if value is true; otherwise it is ignored.
		///     - just_bottom_center();                // Same as bottom_center()
		///     - just_bottom_center_s(value : bool);  // Same as bottom_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		///
		pub fn just_bottom_center() -> kw { kw::just_bottom_center_s(true) }
	
		/// Sets the text or object in the bottom-center of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - bottom_center();                     // Set to bottom-center (true as default)
		///     - bottom_center_s(value : bool);       // Set to bottom-center if value is true; otherwise it is ignored.
		///     - just_bottom_center();                // Same as bottom_center()
		///     - just_bottom_center_s(value : bool);  // Same as bottom_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		///
		pub fn bottom_center_s(_justtopcenter : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(21,_justtopcenter); }
			kw{pointer : ret_val}		
		}

		/// Sets the text or object in the bottom-center of the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - bottom_center();                     // Set to bottom-center (true as default)
		///     - bottom_center_s(value : bool);       // Set to bottom-center if value is true; otherwise it is ignored.
		///     - just_bottom_center();                // Same as bottom_center()
		///     - just_bottom_center_s(value : bool);  // Same as bottom_center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 
		///		
		pub fn bottom_center() -> kw { kw::bottom_center_s(true) }

		/// numbers_only() -- tells an edit box to only accept integer numbers.  
		///
		/// - By default, edit boxes accept any kind of text.  using <i>numbers_only()</i> limits the text to integer values.
		/// - Use <i>floats_only()</i> to allow the edit box to use floating-point numbers while also disallowing other text.
		///
		/// Function Parameter Forms: 
		/// 
		///     - numbers_only();                     // Set Edit Box to integer numbers only (true is default)
		///     - numbers_only_s(value : bool);       // Set Edit Box to integer numbers only if value is true (otherwise, it is ignored)
		/// 
		/// Example:
		///
		///      let my_edit_box = Sagebox::new_edit_box_s(kw::title("Numbers only Edit Box") + kw::numbers_only());
		///
		pub fn numbers_only_s(_numbers_only : bool) -> kw { Self::generic_bool(14,_numbers_only) }

		/// numbers_only() -- tells an edit box to only accept integer numbers.  
		///
		/// - By default, edit boxes accept any kind of text.  using <i>numbers_only()</i> limits the text to integer values.
		/// - Use <i>floats_only()</i> to allow the edit box to use floating-point numbers while also disallowing other text.
		///
		/// Function Parameter Forms: 
		/// 
		///     - numbers_only();                     // Set Edit Box to integer numbers only (true is default)
		///     - numbers_only_s(value : bool);       // Set Edit Box to integer numbers only if value is true (otherwise, it is ignored)
		/// 
		/// Example:
		///
		///      let my_edit_box = Sagebox::new_edit_box_s(kw::title("Numbers only Edit Box") + kw::numbers_only());
		///
		pub fn numbers_only() -> kw { Self::generic_bool(14,true) }
		
		/// Allows the window to be resized by the user by dragging the edge of the window and maximizing the window.
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - allow_resize();                // Allow Window to be resized (parameter is true as default)
		///     - allow_resize_s(value : bool);  // Allow Window to be resized if value is true; 
		///                                      // otherwise any resizing is turned off.
		///     - resizeable();                  // Same as allow_resize()
		///     - resizeable_s(value : bool);    // Same as allow_resize_s()
		///
		/// By default, once the window is created, the size cannot be changed by the user. 
		/// When bResizeable = true, make sure the window has the canvas size to support larger sizes.  The window will not
		/// be allowed to size greater than the original size of greater canvas set set in the program
		///
		/// Example:
		///	
		///		let win = new_window_s(kw::allow_resize());	// Allow resizing in the window.
		///
		///	Notes:
		/// - <i>allow_resize()</i> and <i>resizeable()</i> are the same function. These are duplicated for
		///compatibility across platforms.
		/// - the function <i>win.allow_resize()</i> can be used to set the resize on or off after the window is created.
		///
		pub fn allow_resize_s(_allow_resize : bool) -> kw { Self::generic_bool(23,_allow_resize) }
		
		/// Allows the window to be resized by the user by dragging the edge of the window and maximizing the window.
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - allow_resize();                // Allow Window to be resized (parameter is true as default)
		///     - allow_resize_s(value : bool);  // Allow Window to be resized if value is true; 
		///                                      // otherwise any resizing is turned off.
		///     - resizeable();                  // Same as allow_resize()
		///     - resizeable_s(value : bool);    // Same as allow_resize_s()
		///
		/// By default, once the window is created, the size cannot be changed by the user. 
		/// When bResizeable = true, make sure the window has the canvas size to support larger sizes.  The window will not
		/// be allowed to size greater than the original size of greater canvas set set in the program
		///
		/// Example:
		///	
		///		let win = new_window_s(kw::allow_resize());	// Allow resizing in the window.
		///
		///	Notes:
		/// - <i>allow_resize()</i> and <i>resizeable()</i> are the same function. These are duplicated for
		///compatibility across platforms.
		/// - the function <i>win.allow_resize()</i> can be used to set the resize on or off after the window is created.
		///
		pub fn allow_resize() -> kw { Self::generic_bool(23,true) }

		/// Allows the window to be resized by the user by dragging the edge of the window and maximizing the window.
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - allow_resize();                // Allow Window to be resized (parameter is true as default)
		///     - allow_resize_s(value : bool);  // Allow Window to be resized if value is true; 
		///                                      // otherwise any resizing is turned off.
		///     - resizeable();                  // Same as allow_resize()
		///     - resizeable_s(value : bool);    // Same as allow_resize_s()
		///
		/// By default, once the window is created, the size cannot be changed by the user. 
		/// When bResizeable = true, make sure the window has the canvas size to support larger sizes.  The window will not
		/// be allowed to size greater than the original size of greater canvas set set in the program
		///
		/// Example:
		///	
		///		let win = new_window_s(kw::allow_resize());	// Allow resizing in the window.
		///
		///	Notes:
		/// - <i>allow_resize()</i> and <i>resizeable()</i> are the same function. These are duplicated for
		///compatibility across platforms.
		/// - the function <i>win.allow_resize()</i> can be used to set the resize on or off after the window is created.
		///
		pub fn resizeable() -> kw { Self::generic_bool(23,true) }

		/// Sets the text or object in the center the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - center();                      // Set to center (true as default)
		///     - center_s(value : bool);        // Set to center if value is true; otherwise it is ignored.
		///     - just_center();                 // Same as center()
		///     - just_center_s(value : bool);   // Same as center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 		
		pub fn center_s(_center : bool) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_boolean(19,_center); }
			kw{pointer : ret_val}		
		}
		
		/// Sets the text or object in the center the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     - center();                      // Set to center (true as default)
		///     - center_s(value : bool);        // Set to center if value is true; otherwise it is ignored.
		///     - just_center();                 // Same as center()
		///     - just_center_s(value : bool);   // Same as center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 		
		pub fn center() -> kw { kw::center_s(true) }

		/// Sets the text or object in the center the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     center();                      // Set to center (true as default)
		///     center_s(value : bool);        // Set to center if value is true; otherwise it is ignored.
		///     just_center();                 // Same as center()
		///     just_center_s(value : bool);   // Same as center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 		
		pub fn just_center_s(_center : bool) -> kw { kw::center_s(_center) }
		
		/// Sets the text or object in the center the window (or region)
		/// 
		/// Function Parameter Forms: 
		/// 
		///     center();                      // Set to center (true as default)
		///     center_s(value : bool);        // Set to center if value is true; otherwise it is ignored.
		///     just_center();                 // Same as center()
		///     just_center_s(value : bool);   // Same as center_s()
		///
		/// note:
		///
		/// - Use additional keywords <i>pad_x()</i> and <i>pad_y</i> to move the position left or right, or up and down, respectively. 		
		pub fn just_center() -> kw { kw::center() }

		/// auto_update() - Tells the window to auto-update the window is items or text are drawn to it. 
		/// 
		/// The default setting is to not auto-update the window, unless a Sagebox function that requires input, creates a control, or
		/// otherwise makes sense to update the window. 
		///
		/// - With <i>auto_update()</i> off, a call to <i>win.update()</i> is required to ensure the contents drawn to the window are updated on the screen.
		/// - With <i>auto_update()</i> on, a contents are updated on a regular basis as contents are drawn to the window, in the range of 10ms-20ms,
		/// keeping the screen update during long drawing processes, when desired.
		/// (note that since this happens with drawing functions, sometimes adding a <i>win.update()</i> call after drawing routines can update the
		/// last part drawn more quickly). 
		///
		/// Function Parameter Forms: 
		/// 
		///		auto_update();                     // Set auto_update to On (true as default)
		///    	auto_update_s(auto_update: bool);  // Set auto-update to on (when true) or off (when false)
		///
		/// Notes:
		///
		/// - <i>win.auto_update()</i> can be called to set the auto-update via a function call rather than a keyword when creating the window.
		/// - see the <i>real_time()</i> keyword for real-time windows that go further than auto_update OFF, when using real-time graphics (i.e. continuous,
		/// changing graphics in real-time vs. less intensive graphics drawing displays).
		///
		/// Examples:
		///	
		///		let win = new_window_s(kw::auto_update());                 // Turn auto-update On.
		/// 	let win = new_window_s(kw::auto_update_s(update_status));  // Set auto_update based on a boolean value
		/// 	win.set_auto_update(update_status)                         // Same as above, but through a function call directly to the window.
		///
		pub fn auto_update() -> kw { Self::generic_bool(22,true) }

		/// auto_update() - Tells the window to auto-update the window is items or text are drawn to it. 
		/// 
		/// The default setting is to not auto-update the window, unless a Sagebox function that requires input, creates a control, or
		/// otherwise makes sense to update the window. 
		///
		/// - With <i>auto_update()</i> off, a call to <i>win.update()</i> is required to ensure the contents drawn to the window are updated on the screen.
		/// - With <i>auto_update()</i> on, a contents are updated on a regular basis as contents are drawn to the window, in the range of 10ms-20ms,
		/// keeping the screen update during long drawing processes, when desired.
		/// (note that since this happens with drawing functions, sometimes adding a <i>win.update()</i> call after drawing routines can update the
		/// last part drawn more quickly). 
		///
		/// Function Parameter Forms: 
		/// 
		///		auto_update();                     // Set auto_update to On (true as default)
		///    	auto_update_s(auto_update: bool);  // Set auto-update to on (when true) or off (when false)
		///
		/// Notes:
		///
		/// - <i>win.auto_update()</i> can be called to set the auto-update via a function call rather than a keyword when creating the window.
		/// - see the <i>real_time()</i> keyword for real-time windows that go further than auto_update OFF, when using real-time graphics (i.e. continuous,
		/// changing graphics in real-time vs. less intensive graphics drawing displays).
		///
		/// Examples:
		///	
		///		let win = new_window_s(kw::auto_update());                 // Turn auto-update On.
		/// 	let win = new_window_s(kw::auto_update_s(update_status));  // Set auto_update based on a boolean value
		/// 	win.set_auto_update(update_status)                         // Same as above, but through a function call directly to the window.
		///
		pub fn auto_update_s(_auto_update : bool) -> kw { Self::generic_bool(22,_auto_update) }

		/// title() - Sets the title of a window or control such as a combo box, radio button, input box, etc.
		/// 
		/// Having a title in a window or control is most always optional. the title() keyword allows a title to be set for the window or control.
		///
		/// Function Parameter Forms: 
		/// 
		///		title(title : &str);  // String-slice text for the title text.
		///
		/// Notes:
		///
		/// - For Windows, the title is displayed in the top-bordered, non-client top area, outside of the window contents.
		/// - For controls, the title is displayed in a position selected for the type of control.
		/// - See title justification keywords, such as <i>title_right</i>, <i>title_top</i>, etc. to adjust the orientation of the title for various controls. 
		///
		/// Examples:
		///	
		///		let win = new_window_s(kw::title("This is a new window",kw::auto_update());  	// Set the title of the window
		/// 	let win = input_box = Sagebox::input_box_s(kw::title("this is an input box"));  // Set the title of an input box
		///
		pub fn title(_title : &str) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_string(18,_title.as_ptr(),_title.len()); }
			kw{pointer : ret_val}
		}
		
		pub fn label(_title : &str) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_string(38,_title.as_ptr(),_title.len()); }
			kw{pointer : ret_val}
		}
    	pub fn label_font(_font : i32) -> kw { unsafe { kw{pointer :  ext_func::rust_kw_generic_integer(39,_font) } } }
		pub fn label_font_str(_font : &str) -> kw { unsafe { kw{pointer :  ext_func::rust_kw_generic_string(39,_font.as_ptr(),_font.len()) } } }


		fn _bg_color(_color : RgbColor) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_rgb(1,_color.red,_color.green,_color.blue,255); }
			kw{pointer : ret_val}
		}
	
		fn _bg_color_a(_color : RgbColorA) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_rgb(1,_color.red,_color.green,_color.blue,_color.opacity); }
			kw{pointer : ret_val}
		}
		fn _bg_color_str(_color : &str) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_string(2,_color.as_ptr(),_color.len()); }
			kw{pointer : ret_val}
		}

		fn _pen_color(_color : RgbColor) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_rgb(0,_color.red,_color.green,_color.blue,255); }
			kw{pointer : ret_val}
		}

		fn _pen_color_a(_color : RgbColorA) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_rgb(0,_color.red,_color.green,_color.blue,_color.opacity); }
			kw{pointer : ret_val}
		}

		fn _pen_color_str(_color : &str) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::rust_kw_generic_string(3,_color.as_ptr(),_color.len()); }
			kw{pointer : ret_val}
		}
		fn _range(min_range : i32,max_range : i32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::sage_rust_kw_range_int(min_range,max_range); }
			kw{pointer : ret_val}
		}		

		fn _rangef(min_range : f32,max_range : f32) -> kw
		{
			let ret_val : i64;
			unsafe { ret_val = ext_func::sage_rust_kw_range_float(min_range,max_range); }
			kw{pointer : ret_val}
		}

		fn _size(win_size : &(i32,i32)) -> kw { kw{pointer : unsafe { ext_func::sage_rust_kw_size_int(win_size.0,win_size.1) } } }		
		fn _location(loc : &(i32,i32)) -> kw { kw{pointer : unsafe { ext_func::sage_rust_kw_location_int(loc.0,loc.1) } } }		
		pub fn width(width : i32) -> kw { Self::generic_integer(50,width) }  		
		

		/// height() - Sets the height of a control, such as an input box.
		/// 
		/// Function Parameter Forms: 
		/// 
		///		height(height : i32)  // Set the height with an integer.
		///
		/// Note: 
		///
		/// - For input boxes, the <i>height()</i> keyword is not mandatory, and a default height is chosen when the <i>height()</i> keyword is not used.
		///
		/// Example:
		///	
		/// 	let input_box = Sagebox::input_box_s(kw::height(100),kw::title("this is an input box"));  // Set the height to 100 in the input box
		///
		pub fn height(height : i32) -> kw { Self::generic_integer(25,height) }
	

		/// disabled() - Sets the disabled status of a control (or window).
		/// 
		/// Function Parameter Forms: 
		/// 
		///		disabled()                    // Disable the control (or window)
		///		disabled_s(disabled : bool)   // Set the Disabled status of the control (disabled when true, enabled or re-enabled when false)
		///
		/// Notes: 
		///
		/// - Controls (and windows) are enabled by default.  disabled() is used to default the control (or window) to disabled.
		/// - When a control (or window) is disabled, it may be re-enabled by using <i>disabled(false)</i> to enabled it.
		/// - When a control is disabled, the control will typically turn gray to denote it's disabled status.  It will also become inoperable, not 
		///reacting to mouse movements, presses, etc.
		/// - When a window is disabled, the contents of the window remain the same (it is not grayed-out), and the program may still write and draw to the window. 
		///However, the user will not be able to use the mouse or keyboard to interact with the window until it is re-enabled.
		/// - All controls have an <i>enable()</i> and <i>disable()</i> function that may be called via a function call (e.g. <i>checkbox.disable()</i>).  Using the <i>disabled()</i>
		///keyword is not required to disabled the control at creation, and may be done via the control's <i>enable()</i> or <i>disabled()</i> function later.
		///
		/// Example:
		///	
		/// 	my_checkbox = Sagebox::dev_checkbox("This is a checkbox",kw::disabled());             // Create a disabled checkbox
		/// 	my_slider   = Sagebox::dev_slider("This is a slider",kw::disabled_s(disable_status)); // Create a slider and disable (or keep it enabled) it via a boolean value.
		/// 	my_slider.enable(enable_status);  // Example using the disable() or enable() function call independently.
		///
		pub fn disabled() -> kw { Self::generic_bool(27,true) }

		/// disabled() - Sets the disabled status of a control (or window).
		/// 
		/// Function Parameter Forms: 
		/// 
		///		disabled()                    // Disable the control (or window)
		///		disabled_s(disabled : bool)   // Set the Disabled status of the control (disabled when true, enabled or re-enabled when false)
		///
		/// Notes: 
		///
		/// - Controls (and windows) are enabled by default.  disabled() is used to default the control (or window) to disabled.
		/// - When a control (or window) is disabled, it may be re-enabled by using <i>disabled(false)</i> to enabled it.
		/// - When a control is disabled, the control will typically turn gray to denote it's disabled status.  It will also become inoperable, not 
		///reacting to mouse movements, presses, etc.
		/// - When a window is disabled, the contents of the window remain the same (it is not grayed-out), and the program may still write and draw to the window. 
		///However, the user will not be able to use the mouse or keyboard to interact with the window until it is re-enabled.
		/// - All controls have an <i>enable()</i> and <i>disable()</i> function that may be called via a function call (e.g. <i>checkbox.disable()</i>).  Using the <i>disabled()</i>
		///keyword is not required to disabled the control at creation, and may be done via the control's <i>enable()</i> or <i>disabled()</i> function later.
		///
		/// Example:
		///	
		/// 	my_checkbox = Sagebox::dev_checkbox("This is a checkbox",kw::disabled());             // Create a disabled checkbox
		/// 	my_slider   = Sagebox::dev_slider("This is a slider",kw::disabled_s(disable_status)); // Create a slider and disable (or keep it enabled) it via a boolean value.
		/// 	my_slider.enable(enable_status);  // Example using the disable() or enable() function call independently.
		///
		pub fn disabled_s(_disabled : bool) -> kw { Self::generic_bool(27,_disabled) }

		/// columns() - Sets the number of columns used on a radio button grouping.
		/// 
		/// - By default, radio button groups are vertically placed, with one radio button and text per-line. 
		/// - see <i>horz()</i> keyword to make radio buttons horizontally placed.
		/// - Whether the orientation is vertical (default) or horizontal, the <i>columns()</i> keyword can set the number of radio buttons in a group per-line.
		/// 
		/// Function Parameter Forms: 
		/// 
		///		columns(columns : i32)   // Sets the columns via an integer
		///
		/// Example:
		///	
		/// 	my_radio_btns = Sagebox::dev_radio_buttons_s("button 1\nbutton 2\nbutton 3\nbutton 4",kw::columns(2))  // Sets 2 radio buttons per-line
		///
		pub fn columns(columns : i32) -> kw { Self::generic_integer(28,columns) }


		/// realtime() -- Sets a window's status to use real-time graphics output.
		///
		/// - Use the realtime() keyword when using intensive, repetitive graphics that are meant to keep a high framerate.
		/// - realtime() is meant to support programs that use intensive, real-time graphics that want to keep 
		/// an smooth frame-rate of 60fps or more. 
		/// - When the realtime() keyword is used, the window will not update the window for any reason, and relies on the 
		/// user to update() the window. 
		/// - When realtime() is used, Sagebox sets some system and internal variables to make working with real-time graphics
		/// easier to keep the frame-rate as smooth as possible.
		/// - realtime() is much like turning auto_update off, but goes a little farther. 
		///
		/// Function Parameter Forms: 
		/// 
		///		realtime() // Sets the window's status to realtime
		///
		/// Example:
		///	
		/// 	let my_win = Sagebox::new_window_s(kw::realtime()) 	// Set a real-time status for the window being created.
		///
		pub fn realtime() -> kw { Self::generic_bool(30,true) }

		/// icon_warning() -- Adds a 'warning' icon to an info_window() to add a more serious tone.
		///
		/// - The warning icon is typically a yellow triangle with an exclamation (!) point inside.
		///
		/// Function Parameter Forms: 
		/// 
		///		icon_warning() // Add the warning icon to the window 
		///
		/// Example:
		///	
		/// 	Sagebox::info_window("Something went wrong.  Press OK to continue.",kw::icon_warning()) // create an info_window with a warning icon.
		///		
		pub fn icon_warning() -> kw {Self::generic_bool(31,true) }

		/// hidden() -- Sets the status of a window or control as hidden() (i.e. invisible) on creation.
		///
		/// - Windows and controls are visible by default when created.
		/// - All windows and controls have a <i>show()</i> or <i>hide()</i> function to show/hide the window or control independently, after they are created.
		/// - Since windows and controls are initially visible, they may appear in a quick 'flash' if they are hidden directly after being created.
		/// - The <i>hidden()</i> keyword keeps the window or control from being shown at all, even on creation, until the window or control is shown
		///with the <i>show()</i> function for that window or control.
		///
		/// Function Parameter Forms: 
		/// 
		///		hidden()                 // Makes the window or control invisible/hidden on creation 
		///		hidden_s(hidden : bool)  // Makes the window or control invisible when true (ignored and visible when false)
		///
		/// Examples:
		///	
		///		let win = Sagebox::new_window_s(kw::hidden)); // Create a window, but keep it invisible until the windows win.show() function is called.
		///		
		///		let my_checkbox = Sagebox::dev_checkbox_s("This is a checkbox",kw::hidden(hide_status)) // Create a checkbox and keep it invisible only if hide_status is true.
		///				
		pub fn hidden() -> kw { Self::hidden_s(true) }

		/// hidden() -- Sets the status of a window or control as hidden() (i.e. invisible) on creation.
		///
		/// - Windows and controls are visible by default when created.
		/// - All windows and controls have a <i>show()</i> or <i>hide()</i> function to show/hide the window or control independently, after they are created.
		/// - Since windows and controls are initially visible, they may appear in a quick 'flash' if they are hidden directly after being created.
		/// - The <i>hidden()</i> keyword keeps the window or control from being shown at all, even on creation, until the window or control is shown
		///with the <i>show()</i> function for that window or control.
		///
		/// Function Parameter Forms: 
		/// 
		///		hidden()                 // Makes the window or control invisible/hidden on creation 
		///		hidden_s(hidden : bool)  // Makes the window or control invisible when true (ignored and visible when false)
		///
		/// Examples:
		///	
		///		let win = Sagebox::new_window_s(kw::hidden)); // Create a window, but keep it invisible until the windows win.show() function is called.
		///		
		///		let my_checkbox = Sagebox::dev_checkbox_s("This is a checkbox",kw::hidden(hide_status)) // Create a checkbox and keep it invisible only if hide_status is true.
		///				
		pub fn hidden_s(hidden : bool) -> kw { Self::generic_bool(32,hidden) }

		/// [**sage_icon()**] — Displays a Sagebox Green Cube icon in Quick Form Windows
		///
		/// - This is used to make Sagebox videos and have the Sagebox icon display. 
		/// - <i>This is not intended as a general or public function.</i>
		///
		pub fn sage_icon() -> kw { Self::generic_bool(34,true) }

		/// [**horz()**] — Sets a group of radio buttons or checkboxes in a horizontal row. 
		///
		/// - Default placement may be vertical or horizontal, depending on the control type and 
		///   type of window.
		/// - **<i>horz()</i>** tells Sagebox to place the controls in a horizontal row. 
		/// - **note:** Some functions will place controls on the next line once they exceed the limits of the window edge. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		horz()                 // Sets Horizontal Status for controls
		///		horz_s(horz : bool)    // Sets Horizontal Status conditionally, based on 'horz' boolean
		///
		/// **Notes** 
		///
		/// - See the **<i>cols()</i>** keyword to set number of preferred columns to use for control groups.
		///   - This will place the controls in the columns set, with more controls in the next row.
		///
		/// **Example**
		///
		///	let button_group = Sagebox::dev_radio_buttons_s("button 1\nbutton 2\button 3",kw::horz())
		///
		pub fn horz() -> kw { Self::generic_bool(36,true) }
		
		/// [**horz()**] — Sets a group of radio buttons or checkboxes in a horizontal row. 
		///
		/// - Default placement may be vertical or horizontal, depending on the control type and 
		///   type of window.
		/// - **<i>horz()</i>** tells Sagebox to place the controls in a horizontal row. 
		/// - **note:** Some functions will place controls on the next line once they exceed the limits of the window edge. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		horz()                 // Sets Horizontal Status for controls
		///		horz_s(horz : bool)    // Sets Horizontal Status conditionally, based on 'horz' boolean
		///
		/// **Notes** 
		///
		/// - See the **<i>cols()</i>** keyword to set number of preferred columns to use for control groups.
		///   - This will place the controls in the columns set, with more controls in the next row.
		///
		/// **Example**
		///
		///	let button_group = Sagebox::dev_radio_buttons_s("button 1\nbutton 2\button 3",kw::horz())
		///
		pub fn horz_s(horz: bool) -> kw { Self::generic_bool(36,horz) }

		/// [**vert()**] — Sets a group of radio buttons or checkboxes in a horizontal row. 
		///
		/// - Default placement may be vertical or horizontal, depending on the control type and 
		///   type of window.
		/// - **<i>horz()</i>** tells Sagebox to place the controls in a vertical column. 
		/// - **note:** Some functions will place controls on the next line once they exceed the limits of the window edge. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		vert()                 // Sets Vertical Status for controls
		///		vert_s(vert : bool)    // Sets Vertical Status conditionally, based on 'vert' boolean
		///
		/// **Notes** 
		///
		/// - See the **<i>cols()</i>** keyword to set number of preferred columns to use for control groups.
		///   - This will place the controls in the columns set, with more controls in the next row.
		///
		/// **Example**
		///
		///	let button_group = Sagebox::dev_radio_buttons_s("button 1\nbutton 2\button 3",kw::vert())
		///
		pub fn vert() -> kw { Self::generic_bool(37,true) }

		/// [**vert()**] — Sets a group of radio buttons or checkboxes in a horizontal row. 
		///
		/// - Default placement may be vertical or horizontal, depending on the control type and 
		///   type of window.
		/// - **<i>horz()</i>** tells Sagebox to place the controls in a vertical column. 
		/// - **note:** Some functions will place controls on the next line once they exceed the limits of the window edge. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		vert()                 // Sets Vertical Status for controls
		///		vert_s(vert : bool)    // Sets Vertical Status conditionally, based on 'vert' boolean
		///
		/// **Notes** 
		///
		/// - See the **<i>cols()</i>** keyword to set number of preferred columns to use for control groups.
		///   - This will place the controls in the columns set, with more controls in the next row.
		///
		/// **Example**
		///
		///	let button_group = Sagebox::dev_radio_buttons_s("button 1\nbutton 2\button 3",kw::vert())
		///
		pub fn vert_s(vert: bool) -> kw { Self::generic_bool(37,vert) }

		/// [**_set_caption()**] — Sets a caption for an image title in **`ImageView`** windows
		/// 
		/// - for **<i>image_view()</i>**, this will set the title of the image. 
		/// - for **<i>image_view_before_after()</i>**, this can be used to set the before and after image titles by separating the title with a newline ('\n')
		///   - example **<i>"before title\nafter title"</i>**
		///
		/// **Notes**
		///
		/// - See the **<i>font()</i>** keyword to set the font of the caption.
		///
		/// **Example**
		///
		///		Sagebox::image_view_before_after_s(&image1,&image2,kw::caption("Before Image\nAfter Image"));
		///
		fn _set_caption(caption : &str) -> kw { Self::generic_string(40, caption) }

		/// [**use_event_thread()**] — ** This function is TBD
		///
		pub fn use_event_thread() -> kw { Self::generic_bool(41,true) }    

		/// [**use_event_thread()**] — ** This function is TBD
		///
		pub fn use_event_thread_s(_use_event_thread : bool) -> kw { Self::generic_bool(41,_use_event_thread) }    

		/// [**maximize()**] — Maximizes an Image View window when launched
		///
		/// - By default, unless a specific size is set in keywords, Sagebox decides the size of the
		///   **<i>Image View</i>** window when displaying an image to the screen. 
		/// - **<i>maximize()</i>** will create a window the size of the entire screen (e.g. 1920x1080) to display the image
		///   - The window can still be resized to a smaller size once it is display. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		maximize()                    // Maximizes the window
		///		maximize_s(maximize : bool)   // Maximizes the window conditionally, based on 'maximize' boolean	
		///
		/// **Example**
		///
		///		Sagebox::image_view_before_after_s(&image1,&image2,kw::maxmize());
		///
		pub fn maximize_s(_maximize : bool) -> kw { Self::generic_bool(43,_maximize) }

		/// [**maximize()**] — Maximizes an Image View window when launched
		///
		/// - By default, unless a specific size is set in keywords, Sagebox decides the size of the
		///   **<i>Image View</i>** window when displaying an image to the screen. 
		/// - **<i>maximize()</i>** will create a window the size of the entire screen (e.g. 1920x1080) to display the image
		///   - The window can still be resized to a smaller size once it is display. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		maximize()                    // Maximizes the window
		///		maximize_s(maximize : bool)   // Maximizes the window conditionally, based on 'maximize' boolean	
		///
		/// **Example**
		///
		///		Sagebox::image_view_before_after_s(&image1,&image2,kw::maxmize());
		///
		pub fn maximize() -> kw { Self::generic_bool(43,true) }
	
		/// [**reverse()**] — Reverse the bitmap (i.e. makes it upside-down) when displaying a bitmap in various functions
		///
		/// - By default, Sagebox image routines assume a bitmap is in BGR (blue, green, red) order and is upside-down in memory (i.e. the bottom line displayed is the first line in memory)
		///	  - This is a standard Bitmap form
		/// - For some bitmaps, such as those in Vectors or other non-standard memory, the bitmap may be in forward order, with the first line displayed also the first line in memory
		/// - The reverse() function causes the bitmap to be displayed upside-down relative to it's current memory disposition.
		///   - This does not affect the contents of the incoming bitmap itself.
		///
		/// **Function Parameter Forms** 
		/// 
		///		reverse()                   // Reverses the Bitmap
		///		reverse_s(reverse : bool)   // Reverses the Bitmap conditionally, based on 'reverse' boolean	
		///
		/// **Example**
		///
		///		win.display_bitmap_s((50,50),&my_bitmap,kw::reverse());
		///
		pub fn reverse() -> kw { Self::generic_bool(44,true) }

		/// [**reverse()**] — Reverse the bitmap (i.e. makes it upside-down) when displaying a bitmap in various functions
		///
		/// - By default, Sagebox image routines assume a bitmap is in BGR (blue, green, red) order and is upside-down in memory (i.e. the bottom line displayed is the first line in memory)
		///	  - This is a standard Bitmap form
		/// - For some bitmaps, such as those in Vectors or other non-standard memory, the bitmap may be in forward order, with the first line displayed also the first line in memory
		/// - The reverse() function causes the bitmap to be displayed upside-down relative to it's current memory disposition.
		///   - This does not affect the contents of the incoming bitmap itself.
		///
		/// **Function Parameter Forms** 
		/// 
		///		reverse()                   // Reverses the Bitmap
		///		reverse_s(reverse : bool)   // Reverses the Bitmap conditionally, based on 'reverse' boolean	
		///
		/// **Example**
		///
		///		win.display_bitmap_s((50,50),&my_bitmap,kw::reverse());
		///
		pub fn reverse_s(_reverse : bool) -> kw { Self::generic_bool(44,_reverse) }

		/// [**no_close()**] — Prevents the user from closing the window when viewing Image View Windows
		///
		/// - By default, Image View windows (functions **<i>image_view()</i>** and **<i>image_view_before_after()</i>**) are independent windows
		///that can be closed by the user, which causes them to close and disappear from the screen
		/// - **<i>no_close()</i>** can be used to prevent the user from closing the window, making the window persistent until closed by the program or the program ends.
		/// - The program can use the returned Image View object from the **<i>image_view()</i>** or **<i>image_view_before_after()</i>** function to close the window programmatically. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		no_close()                    // Prevents the use from closing the window
		///		no_close_s(no_close : bool)   // Prevents the use from closing the window conditionally, based on 'no_close' boolean	
		///
		pub fn no_close_s(_no_close : bool ) -> kw { Self::generic_bool(45,_no_close) }

		/// [**no_close()**] — Prevents the user from closing the window when viewing Image View Windows
		///
		/// - By default, Image View windows (functions **<i>image_view()</i>** and **<i>image_view_before_after()</i>**) are independent windows
		///that can be closed by the user, which causes them to close and disappear from the screen
		/// - **<i>no_close()</i>** can be used to prevent the user from closing the window, making the window persistent until closed by the program or the program ends.
		/// - The program can use the returned Image View object from the **<i>image_view()</i>** or **<i>image_view_before_after()</i>** function to close the window programmatically. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		no_close()                    // Prevents the use from closing the window
		///		no_close_s(no_close : bool)   // Prevents the use from closing the window conditionally, based on 'no_close' boolean	
		///
		pub fn no_close() -> kw { Self::generic_bool(45,true) }

		/// [**wait_for_close()**] — Causes Sagebox to wait for an Image View window to be closed by the user before returning. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		wait_for_close()                         // Waits for the user to close the window
		///		wait_for_close_s(wait_for_close : bool)  // Waits for the user to close the window conditionally, based on 'wait_for_close' boolean	
		///
		///- By default, when functions **<i>image_view()</i>** or **<i>image_view_before_after()</i>** are called, the function displays the window
		///and returns immediately, allowing the returned **`ImageView`** object to manage the window through various functions.
		///- using **<i>wait_for_close()</i>** will display the window and not return from **<i>image_view()</i>** or **<i>image_view_before_after()</i>** until the user
		///closes the window.
		///   - This can allow the program to display the image and not worry about managing the returned **`ImageView`** Object, in fire-and-forget fashion.
		/// 	- The returned **`ImageView`** object does not need to be saved or managed.
		///		- Sagebox takes ownership and management of the window and closes it when the user closes it or when the program ends.
		///
		/// **Example**
		///
		///		Sagebox::image_view_s(,&my_bitmap,kw::wait_for_close());
		///
		pub fn wait_for_close_s(_wait_for_close : bool) -> kw { Self::generic_bool(42,_wait_for_close) }  

		/// [**wait_for_close()**] — Causes Sagebox to wait for an Image View window to be closed by the user before returning. 
		///
		/// **Function Parameter Forms** 
		/// 
		///		wait_for_close()                         // Waits for the user to close the window
		///		wait_for_close_s(wait_for_close : bool)  // Waits for the user to close the window conditionally, based on 'wait_for_close' boolean	
		///
		///- By default, when functions **<i>image_view()</i>** or **<i>image_view_before_after()</i>** are called, the function displays the window
		///and returns immediately, allowing the returned **`ImageView`** object to manage the window through various functions.
		///- using **<i>wait_for_close()</i>** will display the window and not return from **<i>image_view()</i>** or **<i>image_view_before_after()</i>** until the user
		///closes the window.
		///   - This can allow the program to display the image and not worry about managing the returned **`ImageView`** Object, in fire-and-forget fashion.
		/// 	- The returned **`ImageView`** object does not need to be saved or managed.
		///		- Sagebox takes ownership and management of the window and closes it when the user closes it or when the program ends.
		///
		/// **Example**
		///
		///		Sagebox::image_view_s(,&my_bitmap,kw::wait_for_close());
		///
		pub fn wait_for_close() -> kw { Self::generic_bool(42,true) }     
		fn _percent_i32(percent : i32) -> kw { Self::generic_integer(48,percent) }
		fn _percent_f32(percent : f32) -> kw { Self::generic_float(48,percent) }


		/// [**zoom_box()**] — Brings up (or prevents) the Zoom Box/Navigator window showing when using Image View Windows
		///
		/// - **<i>image_view()</i>** - By default, the Zoom Box/Navigator window is not shown initially.
		/// - **<i>image_view_before_after()</i>** - By default, the Zoom Box/Navigator window is shown initially. 
		///	
		/// **Function Parameter Forms** 
		/// 
		///		zoom_box()                        // Shows the Zoom Box/Navigator window
		///		zoom_box_s(show_zoom_box : bool)  // Shows the Zoom Box/Navigator window conditionally, based on 'show_zoom_box' boolean	
		///
		/// **Notes**
		///
		/// - The Zoom Box/Navigator Window allows the user to move a small rectangle in a smaller window to move the focus of the image in the window, 
		/// when the window is Zoomed In
		/// - The Zoom Box/Mavigator Window has a Zoom slider to control the Zoom factor
		///   - The user can use the mousewheel on the image directly too zoom in and out in the window
		/// - When multiple **`Image View`** windows are displayed, the Zoom Box automatically switches it display to the window with the focus, and
		///when the focus changes to a new window (i.e. the user clicks on a different **`Image View`** window that is displayed in the Zoom Box)
		///
		/// **Example**
		///
		///		Sagebox::image_view_s(,&my_bitmap,kw::zoom_box());
		///
		pub fn zoom_box_s(show_zoom_box : bool) -> kw { Self::generic_bool(49,show_zoom_box) }

		/// [**zoom_box()**] — Brings up (or prevents) the Zoom Box/Navigator window showing when using Image View Windows
		///
		/// - **<i>image_view()</i>** - By default, the Zoom Box/Navigator window is not shown initially.
		/// - **<i>image_view_before_after()</i>** - By default, the Zoom Box/Navigator window is shown initially. 
		///	
		/// **Function Parameter Forms** 
		/// 
		///		zoom_box()                        // Shows the Zoom Box/Navigator window
		///		zoom_box_s(show_zoom_box : bool)  // Shows the Zoom Box/Navigator window conditionally, based on 'show_zoom_box' boolean	
		///
		/// **Notes**
		///
		/// - The Zoom Box/Navigator Window allows the user to move a small rectangle in a smaller window to move the focus of the image in the window, 
		/// when the window is Zoomed In
		/// - The Zoom Box/Mavigator Window has a Zoom slider to control the Zoom factor
		///   - The user can use the mousewheel on the image directly too zoom in and out in the window
		/// - When multiple **`Image View`** windows are displayed, the Zoom Box automatically switches it display to the window with the focus, and
		///when the focus changes to a new window (i.e. the user clicks on a different **`Image View`** window that is displayed in the Zoom Box)
		///
		/// **Example**
		///
		///		Sagebox::image_view_s(,&my_bitmap,kw::zoom_box());
		///
		pub fn zoom_box() -> kw { Self::generic_bool(49,true) }

		/// [**text_center()**] — Sets the text in a window or text widget to the center of the window or control
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's dimensions, relative to the widget's position, with the **<i>kw::text_center()</i>** keyword.
		///
		pub fn text_center_s(center_text : bool) -> kw { Self::generic_bool(51, center_text ) }

		/// [**text_center()**] — Sets the text in a window or text widget to the center of the window or control
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center()`** and **`just_text_center()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's dimensions, relative to the widget's position, with the **<i>kw::text_center()</i>** keyword.
		///
		pub fn text_center() -> kw { Self::generic_bool(51, true ) }

		/// [**text_center_x()**] — Sets the text in a window or text widget to the center of the window or control in the X plane only
		///
		/// - This only centers the text in the X dimension. The veritical placement is not changed.
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center_x())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's horizontal dimension, relative to the widget's position, with the **<i>kw::text_center_x()</i>** keyword.
		///
		pub fn text_center_x_s(center_text_x : bool) -> kw { Self::generic_bool(52, center_text_x ) } 

		/// [**text_center_x()**] — Sets the text in a window or text widget to the center of the window or control in the X plane only
		///
		/// - This only centers the text in the X dimension. The veritical placement is not changed.
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center_x())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's horizontal dimension, relative to the widget's position, with the **<i>kw::text_center_x()</i>** keyword.
		///
		pub fn text_center_x() -> kw { Self::generic_bool(52, true ) } 

		/// [**text_left()**] — Sets the text in a window or text widget to the leftmost part of the window or control horizontal position
		///
		/// - Text left-justification is typically default behavior. 
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_left()`** and **`just_text_left()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_left())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set to the text widget's leftmost X position (default behavior), relative to the widget's position, with the **<i>kw::text_left()</i>** keyword.
		///
		pub fn text_left_s(text_left : bool) -> kw { Self::generic_bool(53, text_left ) } 

		/// [**text_left()**] — Sets the text in a window or text widget to the leftmost part of the window or control horizontal position
		///
		/// - Text left-justification is typically default behavior. 
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_left()`** and **`just_text_left()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_left())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set to the text widget's leftmost X position (default behavior), relative to the widget's position, with the **<i>kw::text_left()</i>** keyword.
		///
		pub fn text_left() -> kw { Self::generic_bool(53, true ) } 

		/// [**text_right()**] — Sets the text in a window or text widget to the rightmost part of the window or control horizontal position
		///
		/// - The length of the text is calculated, and placed where the end of the text is next to the rightmost part of the control or window.
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_right()`** and **`just_text_right()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_right())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the end of the text is at the rightmost position of the text widget, relative to the widget's position, with the **<i>kw::text_right()</i>** keyword.
		///
		pub fn text_right_s(text_right : bool) -> kw { Self::generic_bool(54, text_right ) } 

		/// [**text_right()**] — Sets the text in a window or text widget to the rightmost part of the window or control horizontal position
		///
		/// - The length of the text is calculated, and placed where the end of the text is next to the rightmost part of the control or window.
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_right()`** and **`just_text_right()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_right())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the end of the text is at the rightmost position of the text widget, relative to the widget's position, with the **<i>kw::text_right()</i>** keyword.
		///
		pub fn text_right() -> kw { Self::generic_bool(54, true ) } 

		/// [**text_top()**] — Sets the text in a window or text widget to the top of the control or window
		///
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_top()`** and **`just_text_top()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_top())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the Y position of the text as at the top of the widget's window area, relative to the widget's position, with the **<i>kw::text_top()</i>** keyword.
		///
		pub fn text_top_s(text_top : bool) -> kw { Self::generic_bool(55, text_top ) } 

		/// [**text_top()**] — Sets the text in a window or text widget to the top of the control or window
		///
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_top()`** and **`just_text_top()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_top())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the Y position of the text as at the top of the widget's window area, relative to the widget's position, with the **<i>kw::text_top()</i>** keyword.
		///
		pub fn text_top() -> kw { Self::generic_bool(55, true ) } 

		/// [**text_bottom()**] — Sets the text in a window or text widget to the bottom of the control or window
		///
		/// - The height of the text is measured so that the bottom of the text is at the bottom of the window or control
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_bottom()`** and **`just_text_bottom()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_bottom())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the bottom of the text is at the bottom Y position of the widget's window area, relative to the widget's position, with the **<i>kw::text_bottom()</i>** keyword.
		///
		pub fn text_bottom_s(text_bottom : bool) -> kw { Self::generic_bool(56, text_bottom ) } 

		/// [**text_bottom()**] — Sets the text in a window or text widget to the bottom of the control or window
		///
		/// - The height of the text is measured so that the bottom of the text is at the bottom of the window or control
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_bottom()`** and **`just_text_bottom()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_bottom())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the bottom of the text is at the bottom Y position of the widget's window area, relative to the widget's position, with the **<i>kw::text_bottom()</i>** keyword.
		///
		pub fn text_bottom() -> kw { Self::generic_bool(56, true ) } 

		/// [**just_text_center()**] — Sets the text in a window or text widget to the center of the window or control
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's dimensions, relative to the widget's position, with the **<i>kw::text_center()</i>** keyword.
		///
		pub fn just_text_center_s(center_text : bool) -> kw { Self::generic_bool(51, center_text ) }

		/// [**just_text_center()**] — Sets the text in a window or text widget to the center of the window or control
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's dimensions, relative to the widget's position, with the **<i>kw::text_center()</i>** keyword.
		///
		pub fn just_text_center() -> kw { Self::generic_bool(51, true ) }

		/// [**just_text_center_x()**] — Sets the text in a window or text widget to the center of the window or control in the X plane only
		///
		/// - This only centers the text in the X dimension. The veritical placement is not changed.
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center_x())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's horizontal dimension, relative to the widget's position, with the **<i>kw::text_center_x()</i>** keyword.
		///
		pub fn just_text_center_x_s(center_text_x : bool) -> kw { Self::generic_bool(52, center_text_x ) } 

		/// [**just_text_center_x()**] — Sets the text in a window or text widget to the center of the window or control in the X plane only
		///
		/// - This only centers the text in the X dimension. The veritical placement is not changed.
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_center_x()`** and **`just_text_center_x()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_center_x())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is centered inside of the widget's horizontal dimension, relative to the widget's position, with the **<i>kw::text_center_x()</i>** keyword.
		///
		pub fn just_text_center_x() -> kw { Self::generic_bool(52, true ) } 

		/// [**just_text_left()**] — Sets the text in a window or text widget to the leftmost part of the window or control horizontal position
		///
		/// - Text left-justification is typically default behavior. 
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_left()`** and **`just_text_left()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_left())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set to the text widget's leftmost X position (default behavior), relative to the widget's position, with the **<i>kw::text_left()</i>** keyword.
		///
		pub fn just_text_left_s(text_left : bool) -> kw { Self::generic_bool(53, text_left ) } 

		/// [**just_text_left()**] — Sets the text in a window or text widget to the leftmost part of the window or control horizontal position
		///
		/// - Text left-justification is typically default behavior. 
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_left()`** and **`just_text_left()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_left())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set to the text widget's leftmost X position (default behavior), relative to the widget's position, with the **<i>kw::text_left()</i>** keyword.
		///
		pub fn just_text_left() -> kw { Self::generic_bool(53, true ) } 

		/// [**just_text_right()**] — Sets the text in a window or text widget to the rightmost part of the window or control horizontal position
		///
		/// - The length of the text is calculated, and placed where the end of the text is next to the rightmost part of the control or window.
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_right()`** and **`just_text_right()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_right())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the end of the text is at the rightmost position of the text widget, relative to the widget's position, with the **<i>kw::text_right()</i>** keyword.
		///
		pub fn just_text_right_s(text_right : bool) -> kw { Self::generic_bool(54, text_right ) } 

		/// [**just_text_right()**] — Sets the text in a window or text widget to the rightmost part of the window or control horizontal position
		///
		/// - The length of the text is calculated, and placed where the end of the text is next to the rightmost part of the control or window.
		/// - The Y positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_right()`** and **`just_text_right()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_right())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the end of the text is at the rightmost position of the text widget, relative to the widget's position, with the **<i>kw::text_right()</i>** keyword.
		///
		pub fn just_text_right() -> kw { Self::generic_bool(54, true ) } 

		/// [**just_text_top()**] — Sets the text in a window or text widget to the top of the control or window
		///
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_top()`** and **`just_text_top()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_top())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the Y position of the text as at the top of the widget's window area, relative to the widget's position, with the **<i>kw::text_top()</i>** keyword.
		///
		pub fn just_text_top_s(text_top : bool) -> kw { Self::generic_bool(55, text_top ) } 

		/// [**just_text_top()**] — Sets the text in a window or text widget to the top of the control or window
		///
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_top()`** and **`just_text_top()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_top())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the Y position of the text as at the top of the widget's window area, relative to the widget's position, with the **<i>kw::text_top()</i>** keyword.
		///
		pub fn just_text_top() -> kw { Self::generic_bool(55, true ) } 

		/// [**just_text_bottom()**] — Sets the text in a window or text widget to the bottom of the control or window
		///
		/// - The height of the text is measured so that the bottom of the text is at the bottom of the window or control
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_bottom()`** and **`just_text_bottom()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_bottom())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the bottom of the text is at the bottom Y position of the widget's window area, relative to the widget's position, with the **<i>kw::text_bottom()</i>** keyword.
		///
		pub fn just_text_bottom_s(text_bottom : bool) -> kw { Self::generic_bool(56, text_bottom ) } 

		/// [**just_text_bottom()**] — Sets the text in a window or text widget to the bottom of the control or window
		///
		/// - The height of the text is measured so that the bottom of the text is at the bottom of the window or control
		/// - The X positioning of the text is not changed
		///
		/// **Notes**
		///
		///- The text justification keywords are specific to text positioning.  
		///   - e.g. **<i>text_center_x()</i>** centers text, where **<i>center_x()</i>** or **<i>just_center_x()</i>** centers a control or object
		///     - In some cases, they perform the same function, such as the **<i>write()</i>** function where the intention is clear. 
		/// - For controls such as a **`Text Widget`**, the text justifiction keywords work on the text within the bounds and relative to the control (text widget) itself,
		/// and the generalized justification keywords (e.g. **<i>center_x()</i>** or **<i>just_center_x()</i>**) work on justifying the control itself relative to its parent window.
		/// - **`text_bottom()`** and **`just_text_bottom()`** are the same function.
		///
		/// **Example**
		///
		///		let my_widget = win.text_widget_s((0,50),"This is a text widget",kw::width(200) + kw::center_x() + kw::text_bottom())
		///
		///- The above example creates a text widget at vertical pixel 50 in the window and centers it in the X plane with **<i>center_x()</i>**
		///  - The text in the widget is set so the bottom of the text is at the bottom Y position of the widget's window area, relative to the widget's position, with the **<i>kw::text_bottom()</i>** keyword.
		///
		pub fn just_text_bottom() -> kw { Self::generic_bool(56, true ) } 

		/// [**no_border()**] — Tells the Window, Control, input box, or Widget to no use a border when it may otherwise default to using a border.
		///
		pub fn no_border_s(no_border : bool) -> kw { Self::generic_bool(57, no_border ) } 

		/// [**no_border()**] — Tells the Window, Control, input box, or Widget to no use a border when it may otherwise default to using a border.
		///
		pub fn no_border() -> kw { Self::generic_bool(57, true ) } 

		/// [**border()**] — Adds a border to a control or Child Window
		///
		/// - This function is useful to adding a sunken border to a child window within a parent window.
		/// - By default, child windows are created with no border. 
		/// 
		/// **Example**
		///
		/// 		let my_child_window = win.child_window_s((50,100),(400,200),kw::border())
		///
		pub fn border_s(no_border : bool) -> kw { Self::generic_bool(58, no_border ) } 

		/// [**border()**] — Adds a border to a control or Child Window
		///
		/// - This function is useful to adding a sunken border to a child window within a parent window.
		/// - By default, child windows are created with no border. 
		/// 
		/// **Example**
		///
		/// 		let my_child_window = win.child_window_s((50,100),(400,200),kw::border())
		///
		pub fn border() -> kw { Self::generic_bool(58, true ) } 

	}

	impl std::ops::Add<kw> for kw 
	{ 
		type Output = kw;

		fn add(self, _rhs: kw) -> kw 
		{
			let ret_val : i64;
			unsafe { ret_val =  ext_func::rust_kw_add_objects(self.pointer,_rhs.pointer); }
			kw{pointer : ret_val }
		}
	}

	impl SageKwNumberType<i32> for kw { fn percent(_percent : i32) -> kw { Self::_percent_i32(_percent) } }
	impl SageKwNumberType<f32> for kw { fn percent(_percent : f32) -> kw { Self::_percent_f32(_percent) } }
	impl SageKwNumberType<f64> for kw { fn percent(_percent : f64) -> kw { Self::_percent_f32(_percent as f32) } }
	impl SageKwStringType<&str> for kw
	{
		fn set_caption(caption : &str) -> kw { Self::_set_caption(caption) }
		fn set_captions(caption : &str) -> kw { Self::_set_caption(caption) }
	}
	impl SageKwStringType<&String> for kw
	{
		fn set_caption(caption : &String) -> kw { Self::_set_caption(caption.as_str()) }
		fn set_captions(caption : &String) -> kw { Self::_set_caption(caption.as_str()) }
	}

	impl  SageKwSingleColor<&str> for kw
	{
		fn bg_color(_color : &str) -> kw { Self::_bg_color_str(_color) }
		fn pen_color(_color : &str) -> kw { Self::_pen_color_str(_color) }
	}
	impl  SageKwSingleColor<&String> for kw 
	{
		fn bg_color(_color : &String) -> kw { Self::_bg_color_str(_color.as_str()) }
		fn pen_color(_color : &String) -> kw { Self::_pen_color_str(_color.as_str()) }
	}
	impl  SageKwSingleColor<RgbColor> for kw 
	{
		fn bg_color(_color : RgbColor) -> kw { Self::_bg_color(_color) }
		fn pen_color(_color : RgbColor) -> kw { Self::_pen_color(_color) }
	}
	impl  SageKwSingleColor<(i32,i32,i32)> for kw 
	{
		fn bg_color(_color : (i32,i32,i32)) -> kw { Self::_bg_color(RgbColor::new(_color.0,_color.1,_color.2)) }
		fn pen_color(_color : (i32,i32,i32)) -> kw { Self::_pen_color(RgbColor::new(_color.0,_color.1,_color.2)) }
	}
	
	impl  SageKwSingleColor<(i32,i32,i32,i32)> for kw 
	{
		fn bg_color(_color : (i32,i32,i32,i32)) -> kw { Self::_bg_color_a(RgbColorA::new(_color.0,_color.1,_color.2,_color.3)) }
		fn pen_color(_color : (i32,i32,i32,i32)) -> kw { Self::_pen_color_a(RgbColorA::new(_color.0,_color.1,_color.2,_color.3)) }
	}
	impl  SageKwSingleColor<RgbColorA> for kw 
	{
		fn bg_color(_color : RgbColorA) -> kw { Self::_bg_color_a(_color) }
		fn pen_color(_color : RgbColorA) -> kw { Self::_pen_color_a(_color) }
	}	



	impl SageKwDefault<i32> for kw { fn default(default : i32) -> kw { Self::generic_integer(11,default) } }	
	impl SageKwDefault<f32> for kw { fn default(default : f32) -> kw { Self::generic_float(17,default) } }	
	impl SageKwDefault<f64> for kw { fn default(default : f64) -> kw { Self::generic_float(17,default as f32) } }	
	impl SageKwDefault<bool> for kw { fn default(default : bool) -> kw { Self::generic_bool(26,default) } }	
	impl SageKwDefault<&str> for kw { fn default(default : &str) -> kw { Self::generic_string(35,default) } }	
	impl SageKwDefault<&String> for kw { fn default(default : &String) -> kw { Self::generic_string(35,default.as_str()) } }	
	impl SageKwDefault<String> for kw { fn default(default : String) -> kw { Self::generic_string(35,&default.as_str()) } }	

	impl SageKwRange<(i32,i32)> for kw { fn range(range : (i32,i32)) -> kw { Self::_range(range.0,range.1) } }	
	impl SageKwRange<(f32,f32)> for kw { fn range(range : (f32,f32)) -> kw { Self::_rangef(range.0,range.1) } }		
	impl SageKwRange<Point<i32>> for kw { fn range(range : Point<i32>) -> kw { Self::_range(range.x,range.y) } }	
	impl SageKwRange<Point<f32>> for kw { fn range(range : Point<f32>) -> kw { Self::_rangef(range.x,range.y) } }	
	impl SagekwPairTypei32<(i32,i32)> for kw
	{
		fn size(size : (i32,i32)) -> kw { Self::_size(&size) }
		fn location(loc : (i32,i32)) -> kw { Self::_location(&loc) }
		fn pos(loc : (i32,i32)) -> kw { Self::_location(&loc) }
	}
	impl SagekwPairTypei32<Point<i32>> for kw
	{
		fn size(size : Point<i32>) -> kw { Self::_size(&(size.x,size.y)) }
		fn location(loc : Point<i32>) -> kw { Self::_location(&(loc.x,loc.y)) }
		fn pos(loc : Point<i32>) -> kw { Self::_location(&(loc.x,loc.y)) }
	}
		


