
use super::MarchingSquares;
use sagebox::*;

impl MarchingSquares
{

/// [InitializeGraphics()] -- Create the main window (Quick Form Window in this program) and set various options and initial states.
///
/// **Quick Form Window vs. Standard Window  (calling Sagebox::quick_form() vs Sagebox::new_window())**
///
/// In most Sagebox examples, Sagebox::new_window() is called to create a simple window, and then the standard Dev Window is (optionally) used to add controls.
/// With the Quick Form Window, the same process occurs, except in a window that contains both the created window and a Dev Window created for the main Quick Form Window
///
/// A Quick Form Window contains a DevWindow (usually to the left) and a standard Window to the right. 
/// Not shown here, the Quick Form Window has a large number of various options for labeling, formatting, etc. 
/// 
/// This allows a more cohesive look, with a program that has one window, with one section for controls and another section for outputting the the window (graphics, text, etc.).
///
/// When the Quick Form Window is created, the window is assigned to the window variable in the class object, and the Dev Window is assigned to 
/// a DevControls object -- when using the standard Dev Window, this is not necessary, since these Dev Window functions can be called directly. 
///
/// QuickForm Window Notes: 
/// 
///    - "devsize=330" sets the width of the DevWindow.  The standard value is pretty wide, and I shortened it to make it a good fit.
///       In addition to values, the following shortcut terms may be used: small, smaller, wide, wider, and widest; e.g. "devsize=wide"
///         
///    - kw::title() sets the title of the window in the upper-left of the window's title bar.
///           Lw::label() will do the same thing, but also add a text label in the right-window with the same text
///           (unless Title() is also used, in which case the upper-left title will default to that text).
///           - The label text color, font size, background color, and justification (left, right, etc.) can also be set.
///             A bitmap may be also placed instead of the text or directly before or after the text for a branded-style display.
///     
///    - realtime() - As with all Sagebox windows, kw::Realtime() tells Sagebox we'll be using this window to update in real-time,
///                   which shuts of auto-update (and makes it more stringent then regular windows with auto-update turned off), and sets some internal
///                   values related to tracking the vertical resync more efficiently.
///     
///     - kw::size() - This sets size of the main window (not the dev window).  The exterior "Quick Form Window" (the window that contains both the Dev Window and
///                    graphic window) adjust accordingly.
///                  - Without kw::Size(), the default is 1200x800 for the graphic window size.
///
/// -----------
/// Other Notes
/// -----------
///
/// - Currently, the GDI is being used directly, as Sagebox functions using Paths and other elements are still being refined. 
///   Sagebox encapsulates most GDI functions in more generic functions for cross-platform consistency. 
///
/// - In all platforms in which the OS supports the GDI, the GDI functions are always available by calling cWin.GetGdiGraphics(), which
///   returns the direct handle to the GDI attached to the window (as shown below)
///
/// - See notes in the code below for more information
///
pub (crate) fn init_graphics(&mut self)
{
    // Create the main 'Quick Form' Window, creating a Dev Window and Main Graphic Window together in the same parent window

    self.qf      = Sagebox::quick_form_s("devsize=330",kw::title("This is a test quick form") + kw::size(self.win_size) + kw::realtime());
    self.win     = self.qf.win.clone(); // Assign the graphic window
    self.dev_win = self.qf.dev_win;     // Assign the dev window -- the main Sagebox Dev Window can be assigned here by calling Sagebox::GetDevControlsPtr();

    self.dev_win.set_bg_bitmap(&self.dev_win_background);    // Set the background of the Dev Window (left) to something unique for our program

    // Sets the graphics smoothing mode for shapes (circles, rectangles, lines, etc.).  The default is anti-aliased.
    // Here, it is turned off (e.g. 'None') since we're doing a lot of drawing at 60fps, which speeds up the graphics a little bit. 
    // --> It really doesn't make that much of a difference, maybe 1ms or so, so it this line can be commented-out to see the difference. 

    self.win.set_smoothing_mode(GdiSmoothingMode::None);

    self.win.set_auto_mouse_capture();  // This is a very useful function -- this tells Sagebox that when the mouse is moved with the mouse button down, and the mouse
                                        // moves outside the window, don't release the mouse and track the values, which may exceed the window size or become negative.
                                        // 
                                        // By default, when the mouse button down and is moving out of the window, the mouse cursor stops at the edge of the window, which
                                        // makes it difficult to draw boxes and other things. 
                                        //
                                        // This is put here to make the drawing the Zoom Rectangle much easier, by allowing the mouse to exceed the window boundaries
                                        // while the Zoom Rectangle is being drawn.

    self.win.set_font(10);              // Set a font or Arial 10pt. This is used for writing corner-point strength values
    self.create_controls();             // Crate the graphic controls (e.g. sliders, checkboxes, etc.)
    self.set_tooltips();                // Set tooltips for the controls.  The messages display when the mouse hovers over the control for a second or so.

}

/// set_tooltips() -- Set the tool-tip messages for each control. 
///
/// Tool Tip messages display when the mouse hovers over the control (e.g. checkbox, slider, etc.) for a second or so.
///
pub (crate) fn set_tooltips(&self)
{
    self.slider_num_balls.set_tooltip("Sets the number of circles used to create the Meta Ball structure"); 
    self.slider_speed.set_tooltip("Sets the speed in which the circles move around the screen (or how fast the Meta Ball shape moves around the screen)");    

    self.checkbox_show_grid.set_tooltip("Shows the grid that represents the size of each element in the Meta Ball array, i.e. one pixel/value.  Larger grids use smaller Meta Ball array sizes.  See the values in the upper-left part of the window");       
    self.checkbox_show_circles.set_tooltip("Show the circles that are used to create the Meta Ball structure (which is then used to create the Marching Square outline)");    
    self.checkbox_show_corners.set_tooltip("Show corner values calculate for Marching Square square (i.e. grid).  Cyan (hot) values are values included (> 1.0).  Gray values are < 1.0 and not used to calculate the Meta Ball Shape."); 

    self.checkbox_show_values.set_tooltip("Show the corner values in real-time.  This shows the Meta Ball values, where green values are > 1 and included, and gray values are not included.  Only works with Grid Sizes of 40x40 or greater.");     
    self.checkbox_show_optimal.set_tooltip("Shows the actual 1:1 curve the Marching Squares is interpolating.  This can be used for comparison.  Grid Sizes of 20x20 or less show nearly the same result, showing a great use for Marching Squares.");
    self.checkbox_fill_squares.set_tooltip("Shows the Meta Ball structure the way it would be calculated (at the current grid size) pixel-by-pixel and then upscaled to window size, instead of using more accurate Marching Square algorithm");
    self.checkbox_show_debug.set_tooltip("Show debug information in the Sagebox Debug Window, useful for debugging.  Debug statements are mostly used when drawing a Zoom Window");      
    self.checkbox_multi_thread.set_tooltip("When the Grid Size is < 5, the Meta Ball Calculation Routine is multi-threaded, which is much faster when grid sizes are 2 or less, or showing Full/Optimal Curve. Does not multi-thread when unchecked.");      
    self.checkbox_show_int_int.set_tooltip("Shows the interior (filled Meta Ball structure) of Interpolated Marching Square outline (only when the outline itself is showing");     
    self.checkbox_show_raw_int.set_tooltip("Shows the interior (filled Meta Ball structure) of Raw (non-interpolated) Marching Square outline (only when the outline itself is showing");       
    
    self.radio_show_outlines.get_button(0).set_tooltip("Shows the Raw Marching Square Calculation - The line segments are calculated, but not interpolated, and are less accurate than when interpolated");
    self.radio_show_outlines.get_button(1).set_tooltip("Shows the Interpolated Marching Square Calculation - The line segments are interpolated.  Compare with the \"Raw Marching Square Outline\" for comparison, which is far less accurate");
    self.radio_show_outlines.get_button(2).set_tooltip("Show the interpolated and Raw (non-interpolated) outlines together.  The Raw outline (red) is much less accurate than the interpolated outline (cyan)");
    self.radio_show_outlines.get_button(3).set_tooltip("Don't show Marching Square Outline, showing only the circles that are used to create the Meta Ball shape");
}

/// create_controls() -- Create the graphic controls used in the program, e.g. sliders, checkboxes, combo box, etc.
///
fn create_controls(&mut self)
{
    // We use two text widgets for two lines of code, one bigger then the other.
    // The first shows the image size, and the second shows the relative grid size compared to the window size
    //
    // - The Font is set to a 16 point for the first line and a 14 point for the second text widget
    // - The "[-]" in the second line tells Sagebox to add the text widget directly after the first one to keep them together as a group. 
    //   Without the "[-]", the text widget is placed a few more pixels down to give it its own space.
    //       - kw::pad_y() with a negative value can also be used, such as kw::pad_y(-5), which brings up the vertical position of 
    //         the text widget 5 pixels

    self.dev_text = self.dev_win.new_text_widget_s("",kw::font(16));
    self.dev_text2 = self.dev_win.new_text_widget_s("[-]",kw::font(14));
    self.dev_win.new_text_s("{skybluelight}Use Mouse to Draw Zoom Window Rectangle\n\n",kw::font_str("15,italic")); 

     // Add Basic controls (sliders, checkboxes, etc.) with defaults and setups (such as kw::Range(), etc.)

    self.slider_num_balls   = self.dev_win.new_slider_s("Number of Cicles",kw::range((1,30)) + kw::default(self.num_balls));
    self.slider_speed       = self.dev_win.new_slider_s("Speed",kw::default(self.speed));

    // Add a combobox for the various grid sizes.  kw::Title() Puts the title to the right or above the combobox
    // --> Note:  Entries can be added by calling m_cComboBoxGridSize.AddItem(). 
    //            The text used allows us to add entries on creation as a shortcut, when we don't have too many and they are known
    //            at the time of creation.
    
    self.combo_box_grid_size =  self.dev_win.new_combobox_s("Grid Size 1x1\nGrid Size 2x2\nGrid Size 5x5\nGrid Size 10x10\n\
                        Grid Size 20x20\nGrid Size 25x25\nGrid Size 40x40\nGrid Size 50x50\nGrid Size 100x100",
                                                                    kw::title("Grid Size") + kw::default(self.radio_grid_size)); 
    
    // Add a set of Radio Buttons to select Marching Square Outline type (Raw, Interpolated, Both, or None)
    
    self.radio_show_outlines     =    self.dev_win.new_radio_buttons_s("Raw Calculation\nInterpolated Lines\nBoth\nNone",
                                    kw::title("Show Derived Marching Square Outline") + kw::columns(2) + kw::default(self.outlines_select));

    // Add Various Checkboxes
    //
    // --> Notes:
    //
    //    1. '-' puts the checkbox on the same line as the last one (if space permits, otherwise it is put on the next line)
    //    2. '+' puts the checkbox on the next line, but doesn't add vertical space to make the checkboxes look put together as a group
    //           Without the '+', the checkbox is placed a few more pixels down to visually separate it from the checkboxes above.
    //    3. {x=170} -- this puts the checkbox as X position 170.  This is done to align all secondary checkboxes to make it look nicer in the window
    //    4. kw::Disabled() sets a potential 'disabled' status on the checkbox, which may not be active in certain modes.

    self.checkbox_show_grid         = self.dev_win.new_checkbox_s("+Show Grid"                 ,kw::default(self.show_grid)); 
    self.checkbox_show_circles      = self.dev_win.new_checkbox_s("-{x=170}Show Circles"       ,kw::default(self.show_circles)); 
    self.checkbox_show_corners      = self.dev_win.new_checkbox_s("+Show Corner Points"        ,kw::default(self.show_corner_points)); 
    self.checkbox_show_values       = self.dev_win.new_checkbox_s("-{x=170}Show Corner Values" ,kw::default(self.show_corner_values)); 
    self.checkbox_show_optimal      = self.dev_win.new_checkbox_s("+Show Exact Curve"          ,kw::default(self.calc_full_curve)); 
    self.checkbox_fill_squares      = self.dev_win.new_checkbox_s("-{x=170}Show Filled Squares",kw::default(self.fill_squares)); 
    self.checkbox_show_debug        = self.dev_win.new_checkbox_s("+Show Debug Window"         ,kw::default(self.debug)); 
    self.checkbox_multi_thread      = self.dev_win.new_checkbox_s("-{x=170}Multithreaded"      ,kw::default(self.multi_thread)); 
    self.checkbox_show_int_int      = self.dev_win.new_checkbox_s("+Show Interpolated Interior",kw::default(self.show_int_interior) + kw::disabled_s(!self.show_int_outline)); 
    self.checkbox_show_raw_int      = self.dev_win.new_checkbox_s("-Show Raw Interior"         ,kw::default(self.show_raw_interior) + kw::disabled_s(!self.show_raw_outline));

}

/// Display the information in the top of the Dev Window using the two Text Widgets we created
///
pub (crate) fn print_window_info(&self)
{
    let grid_width  = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;  

    // {y} = yellow, {g} = green {c} = cyan, {} = end current mode (color, font, etc.)
    
    self.dev_text.write(format!("Processing {{y}}{}x{}{{}} image ({{g}}{}{{}} pixels)",grid_width,grid_height,grid_width*grid_height)); 

    if self.grid_size != 1 { self.dev_text2.write(format!("{{c}}{}{{}} times smaller than 1:1 array of {{g}}{}",
                                                            self.grid_size*self.grid_size,self.win_size.x*self.win_size.y)); }
    else { self.dev_text2.write(format!("Same size as 1:1 array of {{g}}{}",self.win_size.x*self.win_size.y));  }
}

/// process_win_controls() -- Handle all controls not handled in GetControls()
///
/// GetControls() just gets the value of the controls on every frame.
///
/// ProcessWinControls() (this function) looks at specific events in which we may be interested, such as the user using the mouse to create
/// a Zoom-Window selection
///
pub (crate) fn process_win_controls(&mut self)
{
   // Look for a 'Drag Event', which is someone clicking the mouse or moving the mouse while the mouse is down
   // The Drag Event is active until the user releases the mouse button

    if self.win.mouse_drag_event(true) { self.zoom_window.handle_mouse_move(self.win.mouse_drag_get_pos()); }

    // If the Zoom Window is closed by the user, or the Right Mouse Button is clicked, close the zoom window and remove the
    // Zoom rec
    
    if self.win.mouse_r_button_clicked() || self.zoom_window.window_closed() { self.zoom_window.stop_zooming(); }
  
    self.mouse_drag_ended = self.win.mouse_drag_ended();        // The user has released the mouse, so we can check if the intention is to dismiss
                                                                // The Zoom window or to draw a new one

    if self.mouse_drag_ended { self.zoom_window.handle_zoom_mouse_end_drag(); }   // The user has released the mouse, so we can check if the 
                                                                                  // intention is to dismiss the Zoom window or to draw a new one
}

/// get_control_values() -- Get current status of all controls
///
/// We can wait for events or check events here before we act on getting the values.
/// In tight, real-time loops, this is a good option.
///
/// We're in a real-time loop, but it's not criticel to the 200us level, so we just grab the control values to make things easy.
///
/// --> Note:  There are some controls where we look for events (like the ComboBox) because we change other things based
///            on these events.  The very first line of code in this function, for example, does just that in checking the combo box.
///
pub (crate) fn get_control_values(&mut self)
{
    self.show_int_interior  = self.checkbox_show_int_int.checked() && self.show_int_outline;    // Only show interior if interpolated border is drawn
    self.show_raw_interior  = self.checkbox_show_raw_int.checked() && self.show_raw_outline;    // Only show interior if raw border is drawn -- $$ These should be done elsewhere

    self.show_grid          = self.checkbox_show_grid.checked(); 
    self.show_circles       = self.checkbox_show_circles.checked(); 
    self.show_corner_points = self.checkbox_show_corners.checked(); 
    self.show_corner_values = self.checkbox_show_values.checked(); 
    self.fill_squares       = self.checkbox_fill_squares.checked(); 
    self.calc_full_curve    = self.checkbox_show_optimal.checked();
    self.multi_thread       = self.checkbox_multi_thread.checked() && self.num_threads > 1; // Don't multi-thread if we don't have at least 2 

    self.speed              = self.slider_speed.get_pos();
    self.num_balls          = self.slider_num_balls.get_pos();

    // $$ This code needs to be refactored to be symbolic, as in case OutlineMode::Raw, OutlineMode::Interpolated, etc.
    //    rather than 'case 0' or 'case 1'

    if self.radio_show_outlines.pressed()
    {
        match self.radio_show_outlines.get_checked_button()
        {
            0 => { self.show_raw_outline = true;  self.show_int_outline = false; }      // Raw (non-interpolated) Marching Square
            1 => { self.show_raw_outline = false; self.show_int_outline = true; }       // Interpolated Marching Square
            2 => { self.show_raw_outline = true;  self.show_int_outline = true; }       // Both
            3 => { self.show_raw_outline = false; self.show_int_outline = false; }      // None
            _ => {}
        }

        // Enable or disabled checkboxes based on our new status

       self.checkbox_show_int_int.enable(self.show_int_outline);
       self.checkbox_show_raw_int.enable(self.show_raw_outline);

    }
    
    // If the Grid Size has changed, then there are some more things to do than just set the grid size
    
    if self.combo_box_grid_size.item_selected()
    {
        self.grid_size = self.grid_sizes[self.combo_box_grid_size.get_item_selected() as usize];
        self.checkbox_show_values.enable(self.grid_size >= 40); // Enable or Disable the Show Values checkbox based on Grid Size

        self.print_window_info();    // Update on the top of the Dev Window 
    }
    
    // If the 'debug' checkbox is pressed, hide or show the debug window when setting the debug status
    
    if self.checkbox_show_debug.pressed()
    {
        self.debug = self.checkbox_show_debug.checked();
        self.zoom_window.set_debug(self.debug);         // Set it in the zoom window object/mod also
        Sagebox::show_debug_window_s(self.debug);
    }

}

/// blend_top_images() -- Blend the top and bottom graphics.  These have masks, so we blend them instead of displaying them.
///                       (i.e. for PNG or 32-bit bitmaps with a mask)
///
pub (crate) fn blend_top_images(&self)
{
    self.win.blend_bitmap32_s((0,0),&self.bitmap_top,kw::top_center());
    self.win.blend_bitmap32_s((0,0),&self.bitmap_bot,kw::just_bottom_center());
}
}