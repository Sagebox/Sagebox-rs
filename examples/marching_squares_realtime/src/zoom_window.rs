// ----------
// ZoomWindow
// ----------
//
// This file contains all of the functions related to the Zoom Window Interface:
// 
//      1. It creates the zoom window and displays the background graphic.
//         (The background graphic is imported and not loaded here, to keep all graphics in one place.  It is obtained from 
//         the Marching Square class that creates the Zoom Window.
//      2. When the mouse is clicked, it starts watching the mouse movements to draw the zoom rectangle.
//      3. If the mouse is clicked and then released, it removes the zoom window.
//      4. The Zoom window can be moved to move the zoomed-in portion in the window while the zoom window is displaying.
// 
// This set of code is fairly discrete, with just 100+ lines of code (minus comments).  It shows how to use Sagebox in a procedural manner
// (i.e. non-event-driven), also showing how to use multiple windows in Sagebox. 
// 
// --> Other notes:
// 
// -----------------------------------------------------
// How to use this file to use Sagebox for your own code
// -----------------------------------------------------
//
// As with all Sagebox project and demo code, you are encouraged to use it to see how Sagebox works, and
// then copy, modify, and use this code for your own purposes, professional or otherwise.
// 
// All files in Sagebox projects or demos are for this purpose.
// 
// ------------------------------------
// About comments with '$$' in the note
// ------------------------------------
//
// These are to-do items left in to show this as a real piece of code, that might be developed further in an
// in-house (professional or semi-professional), research, education, or other real-world application environment. 
//
// That is, the $$ notations are my personal notations I just haven't gotten to yet, to do in a refactoring stage or just fix-it issues,
// exactly as I do for professional code I write elsewhere.

use crate::marching_squares;
use sagebox::*;
pub struct ZoomWindow
{
    in_zoom         : bool,             // Zoom window is up and displaying zoomed-in contents
    in_move_zoom    : bool,             // The user is currently moving the zoom window rectangle around the main window
    in_mouse_drag   : bool,             // The user is currently creating the Zoom Window Rectangle (i.e. moving the mouse while clicked)
    start_zoom      : Point<i32>,       // Various Point values of current state where the user is using the
    cur_zoom        : Point<i32>,       //    mouse to create, move, or dismiss the zoom rectangle
    start_zoom2     : Point<i32>,
    cur_zoom2       : Point<i32>,
    start_zoom_move : Point<i32>,
    size_zoom       : Point<i32>,       // Size of the zoom rectangle in the main window
    zoom_bitmap     : Bitmap,           // Stores the copied zoomed-in area from the main window
                                        // This stores it at a 1:1 size, and stretches it later to the Zoom Window
    win              : Window,          //  Main Window
    win_size         : Point<i32>,      
    zoom_win         : Window,          // Zoom Window
    zoom_win_size    : Point<i32>,      // Zoom Window Size -- this can be any size.  The code is written to scale and clip to 
                                        // whatever this is set to.
    debug            : bool,            // When true, debug output is written to the Sagebox debug window (set by Marching Squares mod)
}

impl Default for ZoomWindow {
    fn default() -> ZoomWindow {
        ZoomWindow {

        in_zoom             : false,
        in_move_zoom        : false,
        in_mouse_drag       : false,
        start_zoom          : Point::<i32>::default(),
        cur_zoom            : Point::<i32>::default(),
        start_zoom2         : Point::<i32>::default(),
        cur_zoom2           : Point::<i32>::default(),
        start_zoom_move     : Point::<i32>::default(),
        size_zoom           : Point::<i32>::default(),
        zoom_bitmap         : Bitmap::default(),
        win                 : Window::default(),
        win_size            : Point::<i32>::default(),
        zoom_win            : Window::default(),
        zoom_win_size       : Point::<i32>::new(700,400),
        debug               : false,
        }
    }
}

impl ZoomWindow
{

/// Initialize the Zoom Window
///
/// This creates the window, displays the background graphic, etc.
///
pub fn new(ms : &marching_squares::MarchingSquares) -> ZoomWindow
{
    let mut this  = ZoomWindow { ..Default::default() };
    
    // Create the zoom window: 
    // kw::Size() sets the size, where we add borders (50,75) for the display graphic
    // kw::Realtime() tells Sagebox we're going to use it for graphics
    // kw::Hidden() starts it off as initially hidden from view

    this.zoom_win = Sagebox::new_window_s(kw::size(this.zoom_win_size + (50,75)) + kw::realtime() + kw::hidden());

    this.zoom_win.set_as_topmost_window();                  // Keeps it on top no matter what -- i.e. will not let it get behind other windows, even if those 
                                                            // windows have the focus.
    this.zoom_win.cls_radial_str("lightblue,black");        // Set an aesthetic background, just in case we don't have the background graphic
    this.zoom_win.disable_close();                          // $$ Check this logic... It's confusing to look at "WindowClosing" 
                                                            //    -- probably change windows to never really close; you have to check it yourself. 
    this.zoom_win.display_bitmap((0,0), &ms.bitmap_zoom_window);    // Display the main background graphic (we only need to do this once)
 
    this.win_size = ms.win_size;                // Make the main window size local
    this.win = ms.win.clone();                   // Get the main window's pointer as a local pointer.

    this
}	

/// show_zoom_window() -- Shows or hides the Zoom Window
/// 
/// This is placed here as a separate function to get graphic functions out of the main code.
///
fn show_zoom_window(&self,show_window : bool)
{
    self.zoom_win.show_s(show_window);  // Show or Hide the Zoom Window based on status
}

/// window_closed() -- returns true if the user pressed the 'x' button on the Zoom Window
///
pub fn window_closed(&self) -> bool
{
    self.zoom_win.close_button_pressed()
}

/// stop_zooming() -- Close down the Zoom Window
///
pub fn stop_zooming(&mut self)
{
    self.in_zoom = false;
    self.in_mouse_drag = false;
    self.show_zoom_window(false);
}

// display_zoom_rectangle() -- Displays the Zoom Rectangle in the Window and optionally closes down the Zoom Window
//                             if the user closed it or dismissed the rectangle
//
// This function does a little more than displaying the Zoom rectangle.
// It looks to see if we just ended the mouse drag and makes decisions about closing down or showing the Zoom Window
//
// $$ Essentially, this function is doing two separate things and needs to be refactored into two functions, one that is called immediately
//    after the user unclicks the mouse (i.e. Mouse Drag Ended), and another that does the display, which should be called only if the Zoom Window
//    is still active (rather than making that decision itself, as it does now)
//
fn display_zoom_rectangle(&mut self,mouse_drag_ended : bool)
{
    if !(self.in_mouse_drag || self.in_zoom) { return; }  // If we don't have anything to do, then return. 

    let mut cur_start = self.start_zoom;
    let mut size = self.cur_zoom - cur_start + 1; 

    // Switch Points if we have negative distances
    // 
    // If the current mouse point (i.e. ending rectangle point) is negative (in X or Y plane), then
    // set the starting point to the ending point (X and Y, respectively and separately), and then 
    // make the size positive. 
    //
    // This ensures the rectangle points are always starting from top-left (start point) to bottom-right (ending point), leaving
    // a positive size and ensured orientation.

    if size.x < 0 { cur_start.x = self.cur_zoom.x; size.x = - size.x; }
    if size.y < 0 { cur_start.y = self.cur_zoom.y; size.y = - size.y; }

    let mut p1 = cur_start; 
    let mut size_org = size; 
    
    // Determine the Ratio we want for the Zoom Window, since the user can draw any rectangle and we only want to 
    // sample the part in an area consistent with the width and height of the Zoom Window
    
    let ratio_f = self.zoom_win_size.x as f32/self.zoom_win_size.y as f32;
    
    // Determine the Ratio we want for the Zoom Window, since the user can draw any rectangle and we only want to 
    // sample the part in an area consistent with the width and height of the Zoom Window    

    let new_ratio_f = size.x as f32/size.y as f32; 

    // Check to see if Y is over the aspect ratio and adjust

    if new_ratio_f < ratio_f
    {
        size.y = (size.x as f32/ratio_f) as i32;
        p1.y += (size_org.y-size.y)/2;
    } 
   
    // Check to see if X is over the aspect ratio and adjust

   if new_ratio_f > ratio_f
   { 
        size.x = (size.y as f32*ratio_f) as i32;
        p1.x += (size_org.x-size.x)/2;
   }

   // If we just hit a Mouse Drag End (i.e. the user unclicked the mouse after drawing the Zoom rectangle), then 
   // set the final adjustments.
   //
   // $$ This is the part that doesn't belong in this function and should be refactored out into a separate function.

    if mouse_drag_ended
    {
        // Set the final values that were previously in-progress

        self.cur_zoom = p1 + size - 1;
        self.size_zoom = size;
        size_org = size;
        self.start_zoom = p1; 
        cur_start = p1;

        if self.debug { Sagebox::debug_writeln(format!("{{p}}Start = {{y}}{},{}{{}} - Size = {{y}}{},{}",
                                                self.start_zoom.x,self.start_zoom.y,self.size_zoom.x,self.size_zoom.y)); }

        // If the rectangle is too small, then just close down the zoom window -- this allows the user to click on the window to dismiss the rectangle

        if self.size_zoom.x > 1 && self.size_zoom.y > 1 { self.show_zoom_window(true); } else { self.stop_zooming() } 
    }


    // Display the rectangle in-progress or the final rectangle
    // 
    // If the rectangle is too small, don't display it. 

    if size.x > 1 && size.y > 1
    {
        // Show a light transparent filled rectangle denoting the area that will be sampled out to the Zoom Window
        // (this area is based on clipping the user-drawn rectangle to an area that keeps the aspect ratio of the zoom window.

        self.win.fill_rectangle(p1.f32(),size.f32(),"LightGreen(50)");

        // Display a red rectangle to show the aspect-ratio-corrected rectangle clipped to sample, so that we can put it into the 
        // zoom window with the correct aspect ratio.  This is only displayed while the user is drawing the rectangle

        if !mouse_drag_ended { self.win.draw_rectangle(p1.f32(),size.f32(),"red(190)"); }

        if !self.in_zoom { if self.debug { Sagebox::debug_writeln(format!("{{c}}Start = {{y}}{},{}{{}} - Size = {{y}}{},{}",
                                                                                cur_start.x,cur_start.y,size_org.x,size_org.y)); } }
        // Draw the exterior of the Zoom Inset rectangle

        self.win.draw_rectangle_s(cur_start.f32(),size_org.f32(),"green",kw::pen_size(2.0));        
    }
}

/// display_zoom_inset() -- Copies the contents beneath the Zoom Rectangle in the main window and then displays it to
///                         the Zoom Window
///
fn display_zoom_inset(&mut self)
{
    if self.in_zoom
    {
        self.zoom_bitmap = self.win.get_window_bitmap_s(self.start_zoom,self.size_zoom);    // Get the contents of the Zoom Rectangle in the main window

        if self.zoom_bitmap.is_valid()  // There is virtually no chance it is invalid, bit it doesn't hurt to check.
        {                               // --> Even if it were bad, the following code would just fall through innocuously anyway.

            // Put the bitmap out to the Zoom Window, stretched to the display area of the Zoom Window (outside the border)

            self.zoom_win.stretch_bitmap((25,40),self.zoom_win_size.i32(),&self.zoom_bitmap);
            self.zoom_win.update(); // Update it to the window display
        }            
    }
}
/// process_zoom_window_display() -- Update the Zoom WIndow (If it's showing)
///
pub fn process_zoom_window_display(&mut self,mouse_drag_ended : bool)
{
    self.display_zoom_inset();                          // Display (and stretch) the zoom bitmap created
    self.display_zoom_rectangle(mouse_drag_ended);      // Display the rectangle (in the main screen) of the currently zoomed-in portion of the window
}

/// HandleZoomMouseEndDrag() -- The user has stopped drawing the Zoom Rectangle. 
/// 
/// If the rectangle is large enough, we set it as the new Zoom Rectangle and display it in the Zoom Window (and show the Zoom Window)
///
/// If the rectangle is <= 1x1, then the user just clicked the screen to remove the Zoom Rectangle altogether -- in this case,
/// we just close the zoom window and remove the Zoom Rectangle in the main window
///
pub fn handle_zoom_mouse_end_drag(&mut self)
{
    if self.debug { Sagebox::debug_writeln("{p}Mouse Drag Event Ended"); }

    self.in_mouse_drag = false;

    let point = self.win.mouse_drag_get_pos();

    if self.debug {  Sagebox::debug_writeln(format!("End Mouse = {{y}}{},{}",point.x,point.y)); }

    // The part where the Zoom window is closed down is detected later when we display the contents of the Zoom Window
    // These two values set that status

    self.in_zoom = true;
    self.in_move_zoom = false;
}

/// draw_zoom_rect() -- Draws the Zoom Rectangle that is in-progress
///
fn draw_zoom_rect(&mut self,point : Point<i32>)
{
    self.in_zoom    = false;        // Since we're drawing it, we can't be in a zoom state
    self.cur_zoom   = point;        // Set as last known point
  
    self.start_zoom = self.win.mouse_drag_get_start();  
    if self.debug { Sagebox::debug_writeln(format!("Mouse = {{g}}{},{}{{}} - Start = {{g}}{},{}",point.x,point.y,self.start_zoom.x,self.start_zoom.y)); }
    self.in_mouse_drag = true;
}

/// check_zoom_rect_move_start() -- check to if someone starting to move the zoom rectangle making a new one (or discarding it)
///                                 We already know the mouse button is down, but not our status. 
///
fn check_zoom_rect_move_start(&mut self,point : Point<i32>)
{
    // Check if the mouse was clicked -- this tells us that this is the initial point of the rectangle (since the mouse is only clicked to start it)
 
    if self.win.mouse_clicked()
    {
        if point.x >= self.start_zoom.x && point.y >= self.start_zoom.y && point.x <= self.cur_zoom.x && point.y <= self.cur_zoom.y        
        {
            // If we're within the Zoom rectangle in the main window, then we're moving the zoom window

            self.start_zoom_move = self.win.mouse_drag_get_pos();

            if self.debug { Sagebox::debug_writeln("Started Move Mouse Zoom"); }

            self.in_move_zoom   = true;
            self.start_zoom2    = self.start_zoom;
            self.cur_zoom2      = self.cur_zoom;
        }
        else 
        {
            // The user clicked outside of the Zoom rectangle, so start a new one.

            self.in_move_zoom   = false;
            self.in_zoom        = false;
            self.start_zoom     = self.win.mouse_drag_get_start();
            self.cur_zoom       = self.start_zoom;
            self.in_mouse_drag  = true;
            self.stop_zooming();            // Hide the zoom window until we have a new zoom rectangle
                                            // (That way the user can see the whole window)
        }
    }
}

/// CheckZoomRectMove() - Adjusts the Zoom Rectangle currently being drawn
///
fn check_zoom_rect_move(&mut self,in_point : Point<i32> )
{
    let size = self.cur_zoom - self.start_zoom + 1;         // Get size of current rectangle
    let diff = in_point-self.start_zoom_move;

    self.start_zoom = self.start_zoom2 + diff;          // Set the new start value.
    self.cur_zoom   = self.cur_zoom2 + diff;            // Add to the current mouse value to check it 

    // If we've exceeded any window bounds, clip the or move the StartZoom point, and then adjust the ending (or current) mouse point accordingly.

    if self.start_zoom.x < 0 { self.start_zoom.x = 0; }
    if self.start_zoom.y < 0 { self.start_zoom.y = 0; }
    if self.cur_zoom.x >= self.win_size.x { self.start_zoom.x = self.win_size.x - size.x; }
    if self.cur_zoom.y >= self.win_size.y { self.start_zoom.y = self.win_size.y - size.y; }

    // Adjust the ending (or current) mouse point relative to the adjusted starting position

    self.cur_zoom = self.start_zoom + self.size_zoom + 1;
}

/// handle_mouse_move() -- Deal with movement of the mouse in the main window, 
///                        looking at mouse movements related to the Zoom Window
///
/// The user can be starting to draw a zoom window, in the middle of drawing a zoom window, moving a zoom window, or 
/// dismissing a zoom window (which is initially treated as starting to draw one). 
///
pub fn handle_mouse_move(&mut self,mut point : Point<i32>)
{
    // Clip incoming mouse point to window -- since we have 'auto mouse capture' on and can move outside the window
    // when the mouse is down (so we can get negative values and positive values outside the size of the window)

    if point.x >= self.win_size.x { point.x = self.win_size.x - 1; }
    if point.y >= self.win_size.y { point.y = self.win_size.y - 1; }
    if point.x < 0 { point.x = 0; }
    if point.y < 0 { point.y = 0; }

    // Do various things while the mouse is clicked and moving, depending on our status
    
    if self.in_zoom && !self.in_move_zoom   { self.check_zoom_rect_move_start(point); } else
    if self.in_move_zoom                    { self.check_zoom_rect_move(point); } else
                                            { self.draw_zoom_rect(point); }
}

pub fn set_debug(&mut self,debug_on : bool)
{
    self.debug = debug_on;
}

}