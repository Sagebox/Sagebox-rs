//use super::{VecResult,CIntPairBool,ExtRgbColor,SageDLLInit,VecResultBool,CIntPair};
	//mod ext_func
	//{

	#[link(name = "sagebox", kind = "static")]	
	unsafe extern "C"
	{
		pub (crate) unsafe fn sage_rust_show_debug_window(show_window : bool); 
		pub (crate) unsafe fn sage_rust_info_window(pgr_path : * const u8,path_len : usize,keywords_id : i64);
		pub (crate) unsafe fn sage_rust_pgr_is_valid(pgr_id : i64, check_empty : bool) -> bool;
		pub (crate) unsafe fn sage_rust_pgr_read_image_file_str(pgr_id : i64,bitmap_name : * const u8,name_len : usize,is_32bit : bool) -> i64;
		pub (crate) unsafe fn sage_rust_pgr_read_int_pair(pgr_id : i64,key_text : * const u8, key_text_len : usize) -> super::CIntPairBool;
		pub (crate) unsafe fn sage_rust_pgr_read_string(pgr_id : i64,key_text : * const u8, key_text_len : usize) -> super::VecResultBool;
		pub (crate)  unsafe fn sage_rust_read_pgr_file_str(pgr_path : * const u8,path_len : usize) -> i64;
		pub (crate)  unsafe fn sage_rust_pgr_read_binary_file(pgr_id : i64, filename : * const u8,filename_len : usize) -> super::VecResult;
		pub (crate)  unsafe fn sage_rust_pgr_delete(pgr_id : i64);
		pub (crate)  unsafe fn sage_rust_bitmap_is_valid(bitmap_id : i64, check_empty : bool,is_32bit : bool) -> bool;
		pub (crate)  unsafe fn sage_rust_bitmap_get_pixel(bitmap_id : i64, x : i32, y : i32) -> super::ExtRgbColor;
		pub (crate)  unsafe fn sage_rust_bitmap_set_pixel(bitmap_id : i64, x : i32, y : i32,red : i32, green : i32, blue : i32);
		pub (crate)  unsafe fn sage_rust_bitmap_size(bitmap_id : i64,is_32bit : bool) -> super::CIntPair;
		pub (crate)  unsafe fn sage_rust_bitmap_widthbytes(bitmap_id : i64, is_32bit : bool) -> usize;
		pub (crate)  unsafe fn sage_rust_bitmap_get_vector<'a>(bitmap_id : i64) -> super::VecResult;
		pub (crate)  unsafe fn sage_rust_bitmap_from_vector(width : i32, height : i32,bitmap_vec : * const u8, bitmap_len : usize) -> i64;
		pub (crate)  unsafe fn sage_rust_get_open_filename(file_types : * const u8, file_type_len : usize,is_save_type : bool) -> super::VecResult; 
		pub (crate)  unsafe fn sage_rust_read_image_file_str(bitmap_path : * const u8,path_len : usize,is_32bit : bool) -> i64;
		pub (crate)  unsafe fn sage_rust_delete_bitmap(bitmap_id : i64,is32_bit : bool);
		pub (crate)  fn sage_rust_radio_button_pressed(radiobutton_id : i64, peek : bool) -> bool;
		pub (crate)  fn sage_rust_radio_button_get_checked_button(radiobutton_id : i64) -> i32;
		pub (crate)  fn sage_rust_button_pressed(button_id : i64, peek : bool) -> bool;
		pub (crate)  fn sage_rust_button_wait_for_press(radiobutton_id : i64,is_unpress : bool);
		pub (crate)  fn sage_rust_button_checked(button_id : i64) -> bool;
		pub (crate)  fn sage_rust_button_enable(button_id : i64,enable_button : bool);
		pub (crate)  fn sage_rust_button_get_location(button_id : i64) -> (i32,i32);
		pub (crate)  fn sage_rust_button_set_text(button_id : i64,text : * const u8,text_len : usize);
		pub (crate)  fn sage_rust_button_set_style(button_id : i64,text : * const u8,text_len : usize);

		pub (crate)  fn sage_rust_window_get_gdi(window_id : i64) -> i64;
		pub (crate)  unsafe fn sage_rust_window_set_auto_mouse_capture(window_id : i64, auto_capture : bool); 
		pub (crate)  unsafe fn sage_rust_window_set_font_int(window_id : i64, font_type : i32); 
		pub (crate)  unsafe fn sage_rust_window_set_font_str(window_id : i64, msg : * const u8,len : usize);
		pub (crate)  unsafe fn sage_rust_image_view(bitmap_id : i64,bitmap2_id : i64,keywords_id : i64, is_32bit : bool,is_ba : bool) -> i64; 
		pub (crate)  unsafe fn sage_rust_get_sagebox_index() -> super::SageDLLInit; 
		pub (crate)  unsafe fn sage_rust_image_view_closed(index : i64,is_ba : bool) -> bool;
		pub (crate)  unsafe fn sage_rust_image_view_window_count() -> i32;
		pub (crate)  unsafe fn sage_rust_image_view_close_event(index : i64, peek : bool,is_ba : bool) -> bool;
		pub (crate)  unsafe fn sage_rust_image_view_wait_close(index : i64, close_type : i32,is_ba : bool );
		pub (crate)  unsafe fn sage_rust_image_view_perform_task(index : i64,task : i32,is_ba : bool);	
		pub (crate)  unsafe fn sage_rust_image_view_destruct(image_view_index : i64,is_ba : bool); 
		pub (crate)  unsafe fn sage_rust_window_display_bitmap(window_id : i64, bitmap_id : i64, x : i32, y : i32,keywords_id : i64,bits_32 : bool);
		pub (crate)  unsafe fn sage_rust_window_display_bitmap_vec(window_id : i64,  mem_ptr : *const u8, mem_length : usize, x : i32, y : i32,width : i32, height : i32,keywords_id : i64);
		pub (crate)  unsafe fn sage_rust_window_blend_bitmap(window_id : i64, bitmap_id : i64, x : i32, y : i32,keywords_id : i64,bits_32 : bool);
		pub (crate)  unsafe fn sage_rust_window_stretch_bitmap(window_id : i64, bitmap_id : i64, x : i32, y : i32,width : i32, height : i32,keywords_id : i64,bits_32 : bool);
		pub (crate)  unsafe fn sage_rust_window_set_z_position(window_id : i64, top_most : bool,index : i32);
		pub (crate)  unsafe fn sage_rust_window_disable_close(window_id : i64, disable : bool,is_enable : bool);
		pub (crate)  unsafe fn sage_rust_window_mouse_drag_event(window_id : i64, include_mouse_click : bool, peek : bool, ret_point : &(i32,i32)) -> &(i32,i32);
		pub (crate)  unsafe fn sage_rust_window_mouse_drag_get_pos(window_id : i64, ret_point : &(i32,i32), func_index : i32) -> &(i32,i32);
		pub (crate)  unsafe fn sage_rust_window_mouse_drag_ended(window_id : i64) -> bool;
		pub (crate)  unsafe fn sage_rust_window_mouse_clicked(window_id : i64, peek : bool,right_button : bool,check_down : bool) -> bool;
		pub (crate)  unsafe fn sage_rust_window_mouse_wheel_moved(window_id : i64, get_value : bool,peek : bool) -> i32;
		pub (crate)  unsafe fn sage_rust_window_mouse_get_pos<'a>(window_id : i64) -> &'a (i32,i32);
		pub (crate)  unsafe fn sage_rust_window_show_window(window_id : i64, show_window : bool);
		pub (crate)  unsafe fn sage_rust_window_get_window_bitmap(window_id : i64,x : i32, y : i32, width : i32, height : i32) -> i64;

		pub (crate)  unsafe fn sage_rust_gdi_set_smoothing_mode(gdi_id : i64, smoothing_mode : i32);
		pub (crate)  fn sage_rust_gdi_create_path(gdi_id : i64,destroy : bool) -> i64;
 		pub (crate)  unsafe fn sage_rust_gdi_create_solid_brush(gdi_id : i64,red : i32, green : i32, blue : i32, opacity : i32,destroy : bool) -> i64;
 		pub (crate)  unsafe fn sage_rust_gdi_create_pen(gdi_id : i64,red : i32, green : i32, blue : i32, opacity : i32,width : f32,destroy : bool) -> i64;
 		pub (crate)  unsafe fn sage_rust_gdipath_add_rectangle_int(path_id : i64, x : i32, y : i32, width : i32, height : i32);
 	//	pub (crate)  unsafe fn sage_rust_gdipath_add_rectangle_float(path_id : i64, x : f32, y : f32, width : f32, height : f32);
		pub (crate)  unsafe fn sage_rust_gdipath_add_polygon_f(path_id : i64,poly_ptr : * const (f32,f32),len : usize);
		pub (crate)  unsafe fn sage_rust_gdipath_add_line_f(path_id : i64,x1 : f32, y1 : f32, x2 : f32, y2 : f32);
	//	pub (crate)  unsafe fn sage_rust_gdipath_add_polygon(path_id : i64,poly_ptr : * const (i32,i32),len : usize);
 		pub (crate)  unsafe fn sage_rust_gdipath_draw_path(gdi_id : i64, path_id : i64, brush_id : i64, fill : bool);
		pub (crate)  unsafe fn sage_rust_gdi_draw_lines_f(gdi_id : i64, pen_id : i64, line_ptr : * const (f32,f32),len : usize);
		pub (crate)  unsafe fn sage_rust_gdi_draw_line_f(gdi_id : i64, pen_id : i64, x1 : f32, y1 : f32, x2 : f32, y2 : f32);
		pub (crate)  unsafe fn sage_rust_gdi_delete(gdi_id : i64);
 	//	pub unsafe fn sage_rust_gdipath_draw_path_rgb(gdi_id : i64, path_id : i64, brush_id : i64, red : i32, green : i32, blue : i32,
	//								opacity : i32, width : f32, fill : bool);
		pub (crate)  fn sage_rust_gdi_draw_ellipse_rgb(gdi_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, 
							red : i32, green : i32, blue : i32, opacity : i32, pen_width : f32, filled : bool);
		pub (crate)  fn sage_rust_get_color(color_str : * const u8,len : usize,red : &i32,green : &i32, blue : &i32); 
	
		pub (crate)  fn sage_rust_window_get_event(window_id : i64,pending : bool, peek: bool,allow_close : bool) -> bool; 
		pub (crate)  fn sage_rust_window_wait_for_close(window_id : i64); 
		pub (crate)  fn sage_rust_window_update(window_id : i64); 
		pub (crate)  fn sage_rust_window_vsync_wait(window_id : i64) -> bool; 

		pub (crate)  fn sage_rust_radio_button_get_button(radio_button_id : i64,button_id : i32) -> i64;
		pub (crate)  fn sage_rust_add_slider(msg : * const u8,len : usize,float_slider : bool,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_add_input_box(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_add_combo_box(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_add_radio_buttons(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_add_text_widget(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  unsafe fn sage_rust_text_widget_write(text_widget_id : i64, msg : * const u8,len : usize); 
		pub (crate)  fn sage_rust_add_text(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_add_checkbox(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_add_button(msg : * const u8,len : usize,dev_win : i64,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_dev_set_bg(dev_win : i64,bitmap_id : i64) -> bool;

		pub (crate)  fn rust_window_quick_form(msg : * const u8,len : usize,keyword_ptr : i64) -> i64; 
		pub (crate)  fn rust_quickform_get_window(quickform_id : i64) -> i64;
		pub (crate)  fn rust_quickform_get_dev_window(quickform_id : i64) -> i64;
		pub (crate)  fn rust_sagebox_srand(random_seed : i32);
		pub (crate)  fn rust_sagebox_rand(rand_start : i64,rand_end : i64) -> i32;
		pub (crate)  fn rust_sagebox_console_write(test : * const u8,len : usize,add_crlf : bool,as_debug : bool) -> bool;
		pub (crate)  fn rust_sagebox_console_set_fg_color(test : * const u8,len : usize) -> bool;
		pub (crate)  fn sage_rust_get_event() -> bool;
		pub (crate)  fn sage_rust_exit_button(test : * const u8,len : usize) -> i64;
		pub (crate)  fn sage_rust_new_window(keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_dev_slider(msg : * const u8,len : usize,float_slider : bool,keyword_val : i64) -> i64;
		pub (crate)  fn sage_rust_dev_inputbox(msg : * const u8,len : usize,keyword_val : i64) -> i64;
		pub (crate)  fn rust_kw_generic_string(iIndex : i32,test : * const u8,len : usize) -> i64;
		pub (crate)  fn rust_kw_generic_boolean(iIndex : i32,value : bool) -> i64;
		pub (crate)  fn rust_kw_generic_integer(iIndex : i32,value : i32) -> i64;
		pub (crate)  unsafe fn sage_rust_window_generic_integer(window_id : i64, index : i32, value : i32);
		pub (crate)  unsafe fn sage_rust_window_generic_bool(window_id : i64, index : i32, value : bool);
		pub (crate)  fn rust_kw_generic_rgb(iIndex : i32,red : i32, green : i32, blue : i32, opacity : i32) -> i64;
		pub (crate)  fn rust_kw_add_objects(pointer1 : i64, pointer2 : i64) -> i64;
		pub (crate)  fn rust_window_set_auto_autoupdate(pointer1 : i64,auto_update : bool) -> bool;
		pub (crate)  fn rust_window_write(msg : * const u8,len : usize,window_id : i64, keyword_ptr : i64) -> bool;
		pub (crate)  fn rust_window_write_xy(msg : * const u8,len : usize,x : i32, y: i32,window_id : i64, keyword_ptr : i64) -> bool;
		pub (crate)  unsafe fn sage_rust_window_set_write_pos(window_id : i64,x : i32, y : i32);
		//fn rust_test_boolean(test1 : bool,test2 : i32, test3 : i64) -> bool;
		pub (crate)  fn rust_kw_generic_float(iIndex : i32, value : f32) -> i64;
		pub (crate)  fn rust_window_draw_generic_rgb(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, red : i32, green : i32, 
			blue : i32, alpha : i32,keyword_ptr : i64) -> bool;
		pub (crate)  fn rust_window_draw_generic_str(draw_type : i32, window_id : i64, x : f32, y : f32, radius1 : f32, radius2 : f32, color_msg : * const u8,
			color_len : usize,keyword_ptr : i64) -> bool;
		pub (crate)  fn rust_window_draw_triangle_rgb(window_id : i64, x1 : f32, y1 : f32, x2 : f32, y2 : f32, x3 : f32, y3 : f32, red : i32, green : i32, 
			blue : i32, alpha : i32,filled : bool,keyword_ptr : i64) -> bool;
		pub (crate)  fn rust_window_draw_triangle_str(window_id : i64, x1 : f32, y1 : f32, x2 : f32, y2 : f32, x3 : f32, y3 : f32, color_msg : * const u8,
			color_len : usize, alpha : i32,filled : bool,keyword_ptr : i64) -> bool;
	
		pub (crate)  fn sage_rust_window_get_center(window_id : i64) -> i64;
		pub (crate)  fn sage_rust_window_get_size(window_id : i64) -> i64;
		pub (crate)  fn sage_rust_kw_range_int(min_range : i32,max_range : i32) -> i64;
		pub (crate)  fn sage_rust_kw_range_float(min_range : f32,max_range : f32) -> i64;
		pub (crate)  fn sage_rust_kw_size_int(width : i32,height : i32) -> i64;
		pub (crate)  fn sage_rust_kw_location_int(x : i32,y : i32) -> i64;
	
		pub (crate)  fn sage_rust_slider_get_pos_integer(slider_id : i64) -> i32;
		pub (crate)  fn sage_rust_slider_get_pos_float(slider_id : i64) -> f32;
		pub (crate)  fn sage_rust_slider_moved(slider_id : i64) -> bool;
		pub (crate)  fn sage_rust_slider_set_tooltip(msg : * const u8,len : usize,slider_id : i64) -> bool; 
		pub (crate)  fn sage_rust_inputbox_get_text(inputbox_id : i64) -> super::VecResult;

		pub (crate)  fn sage_rust_inputbox_get_integer(inputbox_id : i64) -> i32; 
		pub (crate)  fn sage_rust_inputbox_get_float(inputbox_id : i64) -> f32; 
		pub (crate)  fn sage_rust_inputbox_clear_text(inputbox_id : i64) -> bool; 
		pub (crate)  fn sage_rust_inputbox_return_pressed(inputbox_id : i64) -> bool; 
		pub (crate)  fn sage_rust_window_cls_rgb(window_id : i64,red : i32, green : i32, blue : i32, red2 : i32, green2 : i32, blue2 : i32,radial : bool) -> bool; 
		pub (crate)  fn sage_rust_window_cls_str(window_id : i64, std_addr : * const u8,str_len : usize, radial : bool ) -> bool;
		pub (crate)  unsafe fn sage_rust_window_set_win_color_str(window_id : i64, color_str : * const u8,color_str_len : usize,is_bg_color : bool);
		pub (crate)  unsafe fn sage_rust_window_set_win_color_rgb(window_id : i64, red : i32, green : i32, blue : i32,is_bg_color : bool);
		pub (crate)  fn sage_rust_window_set_pixel_rgb(window_id : i64, x : i32, y: i32,red : i32,green : i32,blue : i32) -> bool;
		pub (crate)  fn sage_rust_window_window_closing(window_id : i64) -> bool;
		pub (crate)  fn sage_rust_window_close_button_pressed(window_id : i64,peek : bool) -> bool;
		pub (crate)  fn sage_rust_checkbox_set_tooltip(msg : * const u8,len : usize,slider_id : i64) -> bool; 
		pub (crate)  fn sage_rust_combobox_item_selected(combobox_id : i64, peek : bool) -> bool;
		pub (crate)  fn sage_rust_combobox_get_item_selected(combobox_id : i64) -> i32;
	
		pub (crate)  fn sage_rust_window_new_text_widget(window_id : i64, x : i32, y : i32, text : * const u8,text_len : usize,keys : i64) -> i64;
		pub (crate)  fn sage_rust_window_new_button(window_id : i64, x : i32, y : i32, text : * const u8,text_len : usize,keys : i64,is_checkbox : bool) -> i64;
		pub (crate)  fn sage_rust_window_child_window(window_id : i64, x : i32, y : i32, width : i32, height : i32,keys : i64) -> i64;

		pub (crate)  unsafe fn sage_rust_window_simple_doc_file_draw(window_id : i64, file_path : * const u8, file_path_len : usize) -> i32;
		pub (crate)  unsafe fn sage_rust_window_simple_doc_filemem_draw(window_id : i64, file_mem : * const u8, file_mem_len : usize) -> i32;
		pub (crate)  unsafe fn sage_rust_window_set_auto_update_type(window_id : i64, update_type : i32);
		pub (crate)  unsafe fn sage_rust_window_delete(window_id : i64);
		pub (crate)  unsafe fn sage_rust_window_clone(window_id : i64) -> i64;
		pub (crate)  unsafe fn sage_rust_gdi_clone(gdi_id : i64) -> i64;

		// $$ determine if bool is 32 bits or 64 bits
	
	}	
//}