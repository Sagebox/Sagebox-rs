// -----------------------------------------------
// Sagebox Marching Squares Explorer Demo for Rust
// -----------------------------------------------
//
// This example uses traditional (non-GPU) graphics to render a real-time outline of a dynamic 
// Meta Ball structure using the Marching Squares algorithm.
//
// It’s designed as a more complete demonstration of Sagebox in Rust — going beyond basic examples 
// to show how interactive graphics, real-time performance, and UI elements can work together in 
// medium-complexity educational tools, research projects, or early-stage prototypes.
//
// Even in debug mode, it runs at 60fps with a 5x5 grid or larger. Smaller grids benefit from 
// release builds, which are significantly faster.
//
// ------------------
// About this Example
// ------------------
//
// The core Marching Squares and Meta Ball logic took about 2–3 hours to implement, with additional 
// time spent integrating UI controls like sliders and checkboxes to explore behavior dynamically.
//
// The idea here is to show how a using a program using Sagebox can grow naturally — starting with a 
// physics or graphics algorithm and adding interactivity without needing to restructure the program.
//
// -------------------------------
// Notes on Graphics and GPU Usage
// -------------------------------
//
// This version uses CPU-based rendering only — no GPU acceleration. The goal is to demonstrate 
// how much can be done with classic 2D graphics, which can make early-stage development simpler 
// and more accessible.
//
// Moving this to the GPU is certainly possible (and Sagebox will support GPU features), but many 
// types of graphical applications can be prototyped or completed entirely using traditional methods.
//
// -----------------------------
// Using or Exploring the Code
// -----------------------------
//
// This program is a working example of how Sagebox functions can be combined to build larger 
// applications. Feel free to borrow code, experiment, and adapt it for your own use.
//
// For now, the examples themselves act as a kind of living documentation, showing how and why 
// Sagebox was built — focused on simplicity, flexibility, and making UI-heavy or graphics-based 
// applications easier to build from scratch.
//
// ---------------------------------------
// Code Comments and Style in this Example
// ---------------------------------------
//
// The code includes many comments to help explain Sagebox behavior and typical use. In general, 
// the style follows what I use for collaborative environments — readable, clear, and ready for 
// others to pick up.
//
// Some extra comments are added for educational clarity and to highlight how Sagebox fits into 
// the program structure.

use sagebox::*;
use crate::zoom_window::*;
use std::time::Instant;
use std::thread::available_parallelism;

const GRID_SIZE_MULTI_THRESHOLD : i32 = 5;
const MAX_CPU_USAGE : usize = 10;


/// ArrayIndex -- used to send index of Meta Ball arrays into calculation functions, rather
///               than other methods to send the arrays themselves from 
///               one MarchingSquares to another. 
///
#[derive(Eq, PartialEq)]  
pub (crate) enum ArrayIndex 
{
    MainV,
    CenterV,
    Full,
}

/// GridIndex  -- used to send index of Marching Square arrays into calculation functions, rather
///               than other methods to send the arrays themselves from 
///               one MarchingSquares to another. 
///
#[derive(Eq, PartialEq)] 
pub (crate) enum GridIndex
{
    Norm,
    Full,
}

/// BallStruct - Structure for Meta Ball Circles used to create the Meta Ball structure, 
/// 
pub (crate) struct BallStruct
 {
    radius      : i32,
    radius_sq   : f32,          // Used to avoid calculating radius^2 in the calc_metaball_array() function,
                                // which is so intensive, this actually saves time.
    loc         : Point<f32>,
    step        : Point<f32>,   // direction and speed of the circle
    color       : RgbColor,
 }


pub struct MarchingSquares
{
    pub win_size         : Point<i32>,      // Main Interior window size (e.g. 1200x800)
    num_balls            : i32,             // number of Meta Balls/Circles being processed/shown on screen
    max_balls            : i32,             // Maximum number of Meta Balls (moved, but not processed in real-time)
    pub speed            : i32,             // Current speed of Meta Ball/Circle Movement (via speed slider)
    show_grid            : bool,            // When true, shows the grid lines in the window
    show_circles         : bool,            // When true, shows the circles used to create the Meta Ball array
    show_corner_points   : bool,            // When true, shows cyan points for Meta Ball cell corner values > = 1.0; gray otherwise
    show_corner_values   : bool,            // When true, shows the cell corner values (where 1.0 is active, and < 1.0 is not considered)
    calc_full_curve      : bool,            // When true, calculates and overlays a 1:1 Meta Ball Curve in gold. 
                                            //    This is used to show the actual curve/outline shape the Marching Squares (at various grid sizes)
                                            //    is approximating (and interpolating)
    fill_squares         : bool,            // When true, this shows how the Meta Ball array looks if we just super-sample the
                                            // Meta Ball array based on the cell-size, rather than using the Marching Squares technique
    debug                : bool,            // When true, the debug window is shown.  The multi-thread vs. static thread time also shows in the window
    multi_thread         : bool,            // When true, the calc_metaball_array() function is multi-threaded, where applicable (grid sizes < 5x5)
    show_int_interior    : bool,            // When true (and the interpolated outline is showing), this fills the interior in a semi-transparent cyan color
    show_raw_interior    : bool,            // When true (and the raw outline is showing), this fills the raw (non-interpolated) interior in a semi-transparent red color
    show_int_outline     : bool,            // When true, the Marching Squares interpolated outline is shown in a cyan color
    show_raw_outline     : bool,            // When true, the Marching Squares Raw (non-interpolated) outline is shown in a red color
    radio_grid_size      : i32,             // Enumerated selection value for the grid-size radio buttons (not sure this value is really used)
    grid_size            : i32,             // Current grid size, i.e. 1x1, 5x5, 20x20, etc.
    outlines_select      : i32,             // Current selection for the outline radio buttons ("Raw","Interpolated","Both","None")
    mouse_drag_ended     : bool,            // True when the user has released the mouse after creating a zoom rectangle.

    grid_sizes           : [i32;9],         // Grid sizes index based on the Grid Size combobox, i.e. 1x1 = 0, 2x2 = 1, 5x5 = 2, etc.
    min_radius           : i32,             // Minimum radius when creating the circles used to create the Meta Ball array
    max_radius           : i32,             // Maximum radius
    step_mul             : f32,             // An emperical value used to control the speed of the circle/Meta Balls movement
    grid_line_size       : f32,             // Size of the grid lines when drawn

    grid_line_color      : RgbColor,        // Color of the grid lines      
    color_int_interior   : RgbColorA,       // Color of Marching Squares Interpolated interior (RgbColorA because it's semi-transparent)
    color_raw_interior   : RgbColorA,       // Color of the mArching Squares Raw (non-interpolated interior)
    color_int_outline    : RgbColor,        // Color of the Marching Squares interpolated outline
    color_raw_outline    : RgbColor,        // Color of the Marching Squares Raw (non-interpolated) outline
    color_value_active   : RgbColor,        // Color of the printed cell values when the value is >= 1.0
    color_value_inactive : RgbColor,        // Color of the pringed cell values when the value is < 1.0
    color_corner_active  : RgbColor,        // Color of the corner-point circle when the value is >= 1.0
    color_corner_inactive: RgbColor,        // Color of the corner-point circle when the value is < 1.0
    color_filled_square  : RgbColorA,       // Color of the 'filled squares' display (i.e. up-sampled pixeletted vs. Marching Squares)
    color_full_curve     : RgbColor,        // Color of the optimal/full-curve 1:1 display showing the actual 1:1 Meta Ball outline
    outline_pen_size     : f32,             // Pen size to use when drawing Meta Ball outlines    

    qf                     : QuickForm,     // Quick form main window containing the main interior window (right) and Dev Window (left)
    pub win                : Window,        // Main Interior Window of the Quickform (qf) window (to make it easier to use)
    dev_win                : DevWindow,     // Dev Window object of the Quickform (qf) Dev Window (left-side of the window)

    // Graphic Controls

    dev_text               : TextWidget,    // Two text widgets used to print information to the upper part of the Dev Window (on the left)
    dev_text2              : TextWidget,
    slider_num_balls       : Slider,        // Number of Meta Balls Slider
    slider_speed           : Slider,        // Speed Slider
    combo_box_grid_size    : ComboBox,      // Combobox to select grid size
    radio_show_outlines    : ButtonGroup,   // Radio Buttons to select outline type (Raw, Interpolated, Both, None)     
    checkbox_show_grid     : Button,       
    checkbox_show_circles  : Button,
    checkbox_show_corners  : Button,
    checkbox_show_values   : Button,
    checkbox_show_optimal  : Button,        // Shows full/optimal 1:1 Meta Ball curve overlayed on top of Marching Square interpolation
    checkbox_fill_squares  : Button,        // Shows up-scaled pixellated version of Meta Ball Array vs. Marching Square interpolation
    checkbox_show_debug    : Button,        // Shows the debug window -- when true, it also shows timing in the window for the Meta Ball Array calculation
    checkbox_multi_thread  : Button,        // Checkbox to select multi-threading on/off
    checkbox_show_int_int  : Button,        // Checkbox to show the interior of the interpolated Marching Sqare outline
    checkbox_show_raw_int  : Button,        // Checkbox to show the interior of the raw (non-interpolated) Marching Square outline

    balls                   : Vec<BallStruct>,  // Structure contining the Meta Ball circles, location, direction and speed.
    v                       : Vec<f32>,         // Main Meta Ball Array output structure
    v_full                  : Vec<f32>,         // Full-sized version when displaying 1:1 optimal/full curve
    v_center                : Vec<f32>,         // Version to show when using pixellated 'fill squares' version
    v_grid                  : Vec<i32>,         // Marching Squares line-segment type output array
    v_grid_full             : Vec<i32>,         // Full-sized version when displaying 1:1 optimal/full curve

    p_grid                  : Vec<(f32,f32)>,   // Stored point information -- used to store line-segment information
    p_grid2                 : Vec<(f32,f32)>,   //    which is initially haphazard.  These arrays are used to convert them into
    p_grid3                 : Vec<(f32,f32)>,   //    much longer polygon segments to optimized graphics output.

    point_storage1          : Vec<(f32,f32)>,
    point_storage2          : Vec<(f32,f32)>,

    num_threads             : i32,              // Number of threads to use when multi-threading

    bitmap_top              : Bitmap32,         // Top-Screen graphic overlay
    bitmap_bot              : Bitmap32,         // Bottom-screen graphic overlay
    pub bitmap_zoom_window  : Bitmap,           // Zoom Window background
    dev_win_background      : Bitmap,           // Dev Window background
    zoom_window             : ZoomWindow,       // Zoom Window object
 
    color_strings           : [&'static str;10],  // Color Selection for Meta Ball Circle display (in string form, e.g. "red","green", etc.)
}


impl Default for MarchingSquares {
    fn default() -> MarchingSquares {
        MarchingSquares {
            win_size              : Point::<i32>::new(1200,800),
            num_balls             : 10,
            max_balls             : 100,
            speed                 : 25,
            show_grid             : true,
            show_circles          : true, 
            show_corner_points    : false,
            show_corner_values    : false,
            calc_full_curve       : false, 
            fill_squares          : false, 
            debug                 : false, 
            multi_thread          : true,
            show_int_interior     : true,
            show_raw_interior     : false, 
            show_int_outline      : true,
            show_raw_outline      : false,
            radio_grid_size       : 6,
            grid_size             : 40,
            outlines_select       : 1,
            mouse_drag_ended      : false,
            grid_sizes            : [1, 2, 5, 10, 20, 25, 40, 50, 100],
            min_radius            : 50,
            max_radius            : 110,
            step_mul              : 4.0,              // Some empirical test value that made is way here, but is still used
            grid_line_size        : 1.6,
            grid_line_color       : RgbColor::new(60,60,60),
            color_int_interior    : sage_color_a::SkyBlue(120),
            color_raw_interior    : sage_color_a::LightRed(130),
            color_int_outline     : sage_color::Cyan,
            color_raw_outline     : sage_color::Red,
            color_value_active    : sage_color::Green,
            color_value_inactive  : sage_color::Gray64,
            color_corner_active   : sage_color::Cyan,
            color_corner_inactive : RgbColor::new(50,50,50),
            color_filled_square   : pan_color_a::Pink(150),
            color_full_curve      : pan_color::Gold,

            outline_pen_size        : 2.0,

            qf                      : QuickForm::default(),
            win                     : Window::default(),
            dev_win                 : DevWindow::default(),
            dev_text                : TextWidget::default(),
            dev_text2               : TextWidget::default(),
            slider_num_balls        : Slider::default(),
            slider_speed            : Slider::default(),

            combo_box_grid_size     : ComboBox::default(), 
            radio_show_outlines     : ButtonGroup::default(),
            checkbox_show_grid      : Button::default(),
            checkbox_show_circles   : Button::default(),
            checkbox_show_corners   : Button::default(),
            checkbox_show_values    : Button::default(),
            checkbox_show_optimal   : Button::default(),
            checkbox_fill_squares   : Button::default(),
            checkbox_show_debug     : Button::default(),
            checkbox_multi_thread   : Button::default(),
            checkbox_show_int_int   : Button::default(),
            checkbox_show_raw_int   : Button::default(),

            balls                   : Vec::new(),
            v                       : vec![], //Vec::new(),
            v_full                  : vec![], //Vec::new(),
            v_center                : vec![], //Vec::new(),
            v_grid                  : vec![], //Vec::new(),
            v_grid_full             : vec![], //Vec::new(),

            p_grid                  : vec![], //Vec::new(),
            p_grid2                 : vec![], //Vec::new(),
            p_grid3                 : vec![], //Vec::new(),
            point_storage1          : vec![],
            point_storage2          : vec![],

            num_threads             : 1,

            bitmap_top              : Bitmap32::default(),
            bitmap_bot              : Bitmap32::default(),
            bitmap_zoom_window      : Bitmap::default(),
            dev_win_background      : Bitmap::default(),
            zoom_window             : ZoomWindow::default(),
            color_strings : [ "pink",     "orange",       "green",    "red",      "cyan",     
                              "beige",    "forestgreen",  "cyan",     "skyblue",  "green",       
                            ],

        }
    }
}


 
impl MarchingSquares
{

pub fn new() -> MarchingSquares
{
    MarchingSquares { ..Default::default() }
}	


/// initialize() -- Sets the sizes of all arrays used, intializes the Meta Ball structures, etc.
///                 This function also sets the number of threads to use for multi-threading, and 
///                 calls out to other initialization functins, such as initializing graphics, controls, etc.
pub fn initialize(&mut self) -> bool
{
    Sagebox::rand_seed(1249);       // A debug random seed to canonize data during testing.  It's left here as a reminder
                                    // rand_seed_time() can also be used to set the seed by the current clock ms 
                                    // (so it will be a different seed each time)

    let cores = available_parallelism().unwrap().get();
    self.num_threads = MAX_CPU_USAGE.min(cores) as i32;

    self.v.resize(((self.win_size.x+1)*(self.win_size.y+1)) as usize,0.0);          // The main Meta Ball Result Array, which calculates each corner of the array (vs. the center)
    self.v_full.resize(((self.win_size.x+1)*(self.win_size.y+1)) as usize,0.0);     // Full-Screen version for displaying 1:1 'Optmal Curve' (Meta Ball Result) over ones with a higher Grid Size
    self.v_center.resize(((self.win_size.x+1)*(self.win_size.y+1)) as usize,0.0);   // Meta Ball Result Array for the 'Show Filled Squares', which calculates based on the center position
    self.v_grid.resize(((self.win_size.x)*(self.win_size.y) + 1) as usize,0);       // Marching Square Grid, converting the Meta Ball result to a value of 0-15 so we know which lines to draw
                                                                                    // (Must go over by 1)
    self.v_grid_full.resize(((self.win_size.x+1)*(self.win_size.y+1)+1) as usize,0);    // Version of vGrid when the 1:1 'Optmial Curve' is displayed and we have a Grid Size > 1
    
    // pGrid 1-3 --> These are used to store line segments to create a much faster way to print the individual line segments.
    // (The problem with the Marching Squares is that it is very difficult to understand where lines are connected, so these array piece them together)
    //
    // The grids are for the Interpolated Result, Raw Result, and Full (Optimal 1:1) Result, respectivelly
    //
    // --> HPC note: The original methodology was one array with a stride for each type.  However, this casued havoc with either
    //                memory caching or optimizing with AVX, or both -- the result was over 2x slower vs. using independent memory
    //                for each set of line segments. This turned into a great example of why testing with timing and various
    //                methods is important when looking for optimized, high-performance solutions. 
    // --> To avoid having to check border conditions, the pGrid(1-3) arrays extend outside the array size by a set of points in all directions
    //     This allows us not to check borders.  Therefore, the arrays are set to (-10000.0,10000.0) so the edges are always this value and can 
    //     never match a valid point in the array.

    self.p_grid.resize(((self.win_size.x+2)*(self.win_size.y+2)*2) as usize,(-10000.0,-10000.0));
    self.p_grid2.resize(((self.win_size.x+2)*(self.win_size.y+2)*2) as usize,(-10000.0,-10000.0));
    self.p_grid3.resize(((self.win_size.x+2)*(self.win_size.y+2)*2) as usize,(-10000.0,-10000.0));


    self.point_storage1.resize((self.win_size.x*self.win_size.y) as usize,(0.0,0.0));       // These are used for stragglers when constructing line segments for   
    self.point_storage2.resize((self.win_size.x*self.win_size.y) as usize,(0.0,0.0));       // the Marching Square results.  The pGrid(1-3) constructs long series of line segments,
                                                                                            // but we eventually run into some that can't be placed anywhere, and these are drawn after-the-fact.
                                                                                            // (These usually number between 1-4 (at the most) for each set of Marching Square border output).    

  
    // Load Graphic Images from Disk or in-memory PGR file
    //
    // Both methods are presented, since we usually load from disk during development and then look at using a more 
    // secure common resource (in this case a PGR file, but it can be anything) when we do a release, so that the 
    // other files can be all in one place, such as in memory or just one file instead of many.
    //
    // See notes in the function LoadImages()
    //
    // note: LoadImages() returns a boolean, which is false if any image was not loaded.
    //       in an industrial or more of a release than just this as a demo program, we would react to it and print a message on the screen, potentially
    //       aborting the entire program.
    //
    //       In the case, however, we don't really care.  a) the PGR file really can't fail once it works once, and 
    //                                                    b) If, for some reasons, an image fails to loads, it just passively doesn't
    //                                                       display, with no harm to the running program (for example, try commenting out the LoadImages() call below).

    let files_ok = self.load_images();

    self.init_balls();          // Fill the array with simple circles, which are used to create the Meta Ball array
    self.init_graphics();       // Create Main Window, initialize controls, and other graphics initializations
    self.print_window_info();   // Print initial information in the two text widgets in the upper-left.
  
    self.zoom_window = ZoomWindow::new(&self);      // Create the Zoom Window class and window.
    self.zoom_window.set_debug(self.debug);         // Set the zoom-window initial debug setting. 

    files_ok
}

/// Run() -- this is the main real-time loop that calls out to calculate the Meta Ball Array, Marching Square Array,
///          and handles all the control changes, etc. 
///
/// This function is in a real-time loop and waits for the Vsync to keep a rate of 60fps (as possible). 
/// The function exits when the user closes the main window
///
pub fn run(&mut self)
{
    let files_ok = self.initialize();   // Initialize object ($$ This doesn't need to be done here.)

    // If we didn't load the files we want, then display a message box and then return. 
    if !files_ok
    {
        Sagebox::info_window_s("One or more graphics files are missing.  Please place files in executable directory and run again.\n(verify the location of marching_squares_images.pgr)",kw::icon_warning());
        return;
    }

    // Run the while loop until someone closes the window

    while !self.win.window_closed()
    {
        let timer_main = Instant::now();    // Create the timer for the loop
        
        self.get_control_values();          // Get all user graphic control values, such as speed, number of circles, checkboxes, grid-size changes, etc.

        let start = Instant::now();         // Get a timer for the meta-ball calculations (so we can see multi-threaded vs. single-threaded times)

        // If we're using multi-threading, calculated the meta ball arrays that benefit from multi-threading. 
        // (the benefit is really only multi-threading for the full/optimal 1:1 curve, or grid sizes < 2 -- multi-threading occurs
        // with grid sizes <= 5, but 5x5 probably doesn't benefit from multi-threading.)

        if self.multi_thread { self.calc_meta_ball_arrays_threaded(); } // Multi-thread what makes sense
        
        // If we're not multi-threading, or each section doesn't fit the criteria for multi-threading, then use single-threaded versions

        if self.grid_size > GRID_SIZE_MULTI_THRESHOLD || !self.multi_thread { self.calc_meta_ball_array(self.grid_size,ArrayIndex::MainV,false);  }
        if self.calc_full_curve && self.grid_size != 1 && !self.multi_thread {  self.calc_meta_ball_array(1, ArrayIndex::Full,false);  }
        if self.fill_squares && (self.grid_size > GRID_SIZE_MULTI_THRESHOLD || !self.multi_thread) { self.calc_meta_ball_array(self.grid_size, ArrayIndex::CenterV,true);  }

        let elapsed = start.elapsed();      // Get time it took to do all of the meta ball calculations (and print it later)

        // Calculate the marching squares segments for each array type (the main marching squares, but also the full 1:1 curve, if we're showing that, and
        // we're not already 1:1 anyway)

        self.calc_marching_squares(self.grid_size, ArrayIndex::MainV,GridIndex::Norm); 
        if self.calc_full_curve && self.grid_size != 1 { self.calc_marching_squares(1,ArrayIndex::Full,GridIndex::Full); }

        self.win.cls();     // Clear the window to it's backrgound color/gradient/bitmap

        if self.show_grid          { self.draw_grid_lines() };        

        // Draw corner points with < 1.0 strength. These are done first so was can overlay the Marching Square Line Segments over them
        // --> The active corner points (strength values >= 1.0) are done later so they overlay the interior of the Meta Ball structure (when drawn)

        if self.show_corner_points { self.draw_corner_points(false); }  

        // If we're showing the raw pixelized image (where we don't look at the corners, and are just looking at the 
        // single point of a low-resolution image, then show them as upscaled squares, that represent a low-res image 
        // that we just made bigger, rather than interpolating as we do with the Marching Squares algorithm

        if self.fill_squares { self.draw_filled_squares(); }
    
        // Show the marching square line segments we calculated (either the fast version or even faster high-performance version)
        self.draw_marching_squares_performance();      // self.draw_marching_squares_faster(); 

        if self.calc_full_curve { self.draw_marching_squares_full(); }  // Display optimal/full 1:1 curve

        if self.show_circles { self.draw_circles(); }                                   // Draw base circles (the circles we use to create everything here)
        if self.show_grid && self.show_corner_values { self.print_corner_values(); }    // Print the Meta Ball Strength values for each corner in the grid
        if self.show_corner_points { self.draw_corner_points(true); }                   // Show corner points for each grid corner, highlighted when strength is >= 1.0
                                                                                        // (meaning the corner point is inside the Meta Ball structure we created)
        self.process_win_controls();    // Handle any other actions going on other than just grabbing control values, such as
                                        // someone drawing or moving the Zoom Window

        self.zoom_window.process_zoom_window_display(self.mouse_drag_ended);    // Handle anything going on with the Zoom Window (movement, drawing, etc.)

        self.update_ball_positions();       // Advance circles to their next position (based on speed and direction)
        self.blend_top_images();            // Display the top and bottom images that overlay everything and blend in via a mask/alpha channel (i.e. PNG, etc.)

        // Display timing for meta-ball calculation in upper-left part of the screen (only if debug is on)
        if self.debug { self.win.write(format!(" time = {} ms",elapsed.as_secs_f32()*1000.0)); }

        self.win.update();      // Update the window with the new frame

        // Check the overall elapsed time.  If we're not seeing about 70fps, then don't wait for the Vsync.
        // Otherwise, we have time to spare, so wait for the Vsync to keep the speed the same, since different
        // grid-size values take drastically different amounts of time to calculate and display. 

        if timer_main.elapsed().as_micros() < 1000000/70 { self.win.vsync_wait(); }
    }
}

}

mod draw_functions;     // Functions that draw the marching squares outlines, interiors, circles, grids, etc.
mod ui_functions;       // Functions that deal with the UI, e.g. control values (checkboxes, sliders, etc.), text output, overlays, etc.
mod calc_functions;     // Functions that calculate the meta ball array and also calculate the Marching Squares Line-Segment type arrays
mod load_images;        // Loads images, either from independent files or .PGR resource file (depending on settings -- load_images())
mod multi_thread;       // Multi-threaded version of calc_meta_ball_array(), thread launcher, etc.
mod ball_functions;     // Just a couple functions to initialize the circles/Meta Ball structure and move them about that didn't seem a
                        //    good fit in the main mod.rs file.