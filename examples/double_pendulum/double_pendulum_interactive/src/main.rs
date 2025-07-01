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


pub mod pend_main;
pub mod double_pendulum;

use sagebox::*;

fn main()
{
    let mut pendulum = pend_main::PendMain::new(); 
    pendulum.run();
	Sagebox::exit_button();

}