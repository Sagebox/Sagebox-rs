//---------------------------------
// Sagebox Double Pendulum Examples
// --------------------------------
//
// This project demonstrates Sagebox rendering a real-time double pendulum at 60 frames per second.
// 
// The directory contains three projects that build upon one another, progressively adding more functionality - beginning with a basic 
// real-time simulation and incrementally adding Sagebox features to control and explore the pendulum’s behavior.
// 
// Each example displays a real-time, graphical double pendulum with a fading trail and live physics data rendered in the window. 
// Here's how each project builds upon the last:
// 
// 	• Simple Pendulum       – Displays the pendulum with a motion trail and outputs timing information to the Sagebox debug window.
// 	• Interactive Pendulum  – Introduces a handle_events() function, allowing the user to drag and reposition the pendulums during simulation.
// 	• Full Pendulum         – Expands on the previous version with GUI controls such as sliders, checkboxes, and input boxes, offering full 
//                            control over physical parameters (e.g., pendulum mass) and display options (e.g., trail length, timing output).
// 
// ---------------------
// Notes on This Example
// ---------------------
// 
// Uses: 
//
// The Double Pendulum program is a great example of using Sagebox in education, research, and engineering. 
// 
// The double-pendulum algorithm is pure math and science.  Sagebox functions were added on top of this existing algorithm (found in various places
// on the Internet) to bring it to life.  In addition to showing the double pendulum moving in realtime, Sagebox tools not only allow 
// controlling the pendulum placement itself, but also dynamic control of its behavior — adjusting parameters like mass, gravity, and 
// damping to explore physical effects in an interactive manner.
//  
// ---------------
// Vertical Resync
// ---------------
// 
//      This program works by waiting for the vertical resync, then drawing the pendulum and updating the window. 
//      The kw::RealTime() setting enables the high resolution timer and sets other configurations to allow better
//      real-time display
// 
// --------------
// Pendulum Trail
// --------------
// 
//      The program shows a fading trail of where the second bob swings around.  This can be disabled by setting show_trail in 
//      DoublePendulum structure to false.
// 
// --------------
// Timing Display 
// --------------
// 
//      When show_timintgis set to true, the time for each loop is displayed in the Sagebox Process Window, showing the milliseconds
//      taken to calculate and draw the pendulum.

use sagebox::*;
use crate::double_pendulum::*;

use std::time::Instant;

pub struct PendMain
{

    qf                      : QuickForm,     // Quick form main window containing the main interior window (right) and Dev Window (left)
    win                     : Window,        // Main Interior Window of the Quickform (qf) window (to make it easier to use)
    dev_win                 : DevWindow,     // Dev Window object of the Quickform (qf) Dev Window (left-side of the window)
    dragging                : usize,
    display_instructions    : bool,
    pend                    : DoublePendulum,
    input_weight1           : InputBox,
    input_weight2           : InputBox,
    slider_dampen           : Slider,
    slider_zoom             : Slider,
    slider_pend_size        : Slider,
    radio_thick_lines       : ButtonGroup,      
    button_display          : Button,
    button_static_length    : Button,
    button_show_trail       : Button,
    button_single_pendulum  : Button,
    button_show_timing      : Button,
    button_start            : Button,  
    quit_button             : Button,

    display_values          : bool,             // Display current values to the window on/off
    show_timing             : bool,             // Display real-time timing information in ms 
    keep_rod_length         : bool,
    quit_program            : bool,
    thickness_levels        : [f64; 3],
    
}

impl Default for PendMain {
    fn default() -> PendMain {
        PendMain {
        qf                      : QuickForm::default(),
    	win                     : Window::default(),
        dev_win                 : DevWindow::default(),
        dragging                : 0,
        display_instructions    : true,
        pend                    : DoublePendulum::default(),
        input_weight1           : InputBox::default(),
        input_weight2           : InputBox::default(),
        slider_dampen           : Slider::default(),
        slider_zoom             : Slider::default(),
        slider_pend_size        : Slider::default(),
        radio_thick_lines       : ButtonGroup::default(),      
        button_display          : Button::default(),
        button_static_length    : Button::default(),
        button_show_trail       : Button::default(),
        button_single_pendulum  : Button::default(),
        button_show_timing      : Button::default(),
        button_start            : Button::default(),  
        quit_button             : Button::default(),
        display_values          : true,
        show_timing             : false,
        keep_rod_length         : false,
        quit_program            : false,
        thickness_levels        : [1.0, 2.0, 3.0 ],
        }
    }
}

impl PendMain
{

pub fn new() -> PendMain
{
    let mut this = PendMain { ..Default::default() };

    // Create main pybox window

    this.qf      = Sagebox::quick_form_s("label=' Sagebox Rust Double Pendulum Example'",kw::realtime() + kw::sage_icon());   // $$ Kw::sage_icon()
    this.win     = this.qf.win.clone(); // Assign the graphic window
    this.dev_win = this.qf.dev_win;     // Assign the dev window -- the main Sagebox Dev Window can be assigned here by calling Sagebox::GetDevCo
    
    // Initialize the pendulum with the Sagebox Window, as well as the Rod Lengths, Mass1 & Mass2, dampening
    // and starting angles (in degrees)

	this.pend = DoublePendulum::new(this.win.clone(),220.0, 185.0, 10.0, 10.0, -15.0, -15.0, 0.9985, 0.33);
    this.init_controls();
    this
}

pub fn run(&mut self)
{
    // -- Main Render Loop --
    //
    // Waits for vertical resync and then draws updates the pendulum and renders it. 
 
    while self.win.vsync_wait() && !self.quit_program
    {
		let timer_main = Instant::now();	// Time the loop so we can see how long it takes

        // Use EventPending() so we only look for events when we know there is one to look for. 
        // EventPending() is not required, but allows us to not spend time looking for events when nothing has happened.

        self.win.cls();             		// Clear the window

        // Print title in upper center of window. 
        //
        // {w} sets color to white (overriding current gray color for output)

        self.win.write_s("{w}{30}Sagebox {orange}Rust{} Interactive Double Pendulum",kw::just_top_center() + kw::pad_y(10));

        if self.win.event_pending() { self.handle_events() }
        self.pend.update();              // Update the pendulum position
        self.pend.render();              // Draw the pendulum

     
        if self.display_instructions { self.display_instructions(); }
        if self.display_values { self.pend.display_values(); }

        self.win.update();                                          // Update the bitmap in the window.
		let elapsed = timer_main.elapsed().as_micros() as i32;		// Get time we took to update and draw the pendulum in microseconds

		if self.show_timing { Sagebox::debug_writeln(format!("Time = {{p}}{}{{}} ms", elapsed)); }   // Print the time out to the Sagebox process/debug window
     
        // If the radio buttons have changed, then set the new thickness value. 

        if self.radio_thick_lines.pressed() { self.pend.thick_mul = self.thickness_levels[self.radio_thick_lines.get_checked_button() as usize]; }

        self.button_static_length.if_pressed(&mut self.keep_rod_length);        // Set KeepRodLength if checkbox pressed
        self.button_display.if_pressed(&mut self.display_values);               // Display real-time values (angle, mass, etc.)
        self.button_single_pendulum.if_pressed(&mut self.pend.single_pend);     // Single Pendulum on/off
        self.button_show_trail.if_pressed(&mut self.pend.show_trail);           // Show bottom pendulum trail

    }

}

}

mod event_functions;        // Functions handling control events, such as mouse or slider movements, checkboxes, etc.
mod display_functions;      // Function handling info display output to the main window
mod init_controls;          // Function initializing the graphic controls on the left side of the window.