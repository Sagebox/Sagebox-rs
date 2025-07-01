
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
// ---------------------------------------------
// Practical Context and Applications
// ---------------------------------------------
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
    win                     : Window,        // Main Interior Window of the Quickform (qf) window (to make it easier to use)
    display_instructions    : bool,
    pend                    : DoublePendulum,
    dragging                : usize,
    show_timing             : bool,
    
}

impl Default for PendMain {
    fn default() -> PendMain {
        PendMain {
    	win                     : Window::default(),
        display_instructions    : true,
        pend                    : DoublePendulum::default(),
        dragging                : 0,
        show_timing             : true,
        }
    }
}

impl PendMain
{

pub fn new() -> PendMain
{
    let mut this = PendMain { ..Default::default() };

    // Create main Sagebox window

    this.win      = Sagebox::new_window_s(kw::title("Sagebox Rust Double Pendulum") + kw::realtime() + kw::bg_color("black") + kw::size((1200,700)));


    // Initialize the pendulum with the Sagebox Window, as well as the Rod Lengths, Mass1 & Mass2, dampening
    // and starting angles (in degrees)

	this.pend = DoublePendulum::new(this.win.clone(),220.0, 185.0, 10.0, 10.0, -15.0, -15.0, 0.9985, 0.33);

    this
}

pub fn run(&mut self)
{
    // -- Main Render Loop --
    //
    // Waits for vertical resync and then draws updates the pendulum and renders it. 
 
    while self.win.vsync_wait() && !self.win.close_button_pressed()
    {
 		let timer_main = Instant::now();	// Time the loop so we can see how long it takes
        self.win.cls();             		// Clear the window

        // Print title in upper center of window. 
        //
        // {w} sets color to white (overriding current gray color for output)

    	self.win.write_s("{w}{30}Sagebox {orange}Rust{} Interactive Double Pendulum",kw::just_top_center() + kw::pad_y(10));

        // Use EventPending() so we only look for events when we know there is one to look for. 
        // EventPending() is not required, but allows us to not spend time looking for events when nothing has happene

        if self.win.event_pending() { self.handle_events() }

        self.pend.display_values();

        if self.display_instructions { self.display_instructions(); }

        self.pend.update();              // Update the pendulum position
        self.pend.render();              // Draw the pendulum

        self.win.update();                                          // Update the bitmap in the window.
	
	    let elapsed = timer_main.elapsed().as_micros() as i32;		// Get time we took to update and draw the pendulum in microseconds

		if self.show_timing { Sagebox::debug_writeln(format!("Time = {{p}}{}{{}} ms", elapsed)); }   // Print the time out to the Sagebox process/debug window
    }

}

}

mod event_functions;        // Functions handling control events, such as mouse or slider movements, checkboxes, etc.
mod display_functions;      // Function handling info display output to the main window
