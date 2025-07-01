use super::PendMain;
use sagebox::*; 

impl PendMain
{
// Display instruction in the window
//
pub (crate) fn display_instructions(&self)
{

    self.win.set_fg_color("Gray192");         // Set the text color

    // {y} = yellow, {g} = green. {30} = set font to size 30pt

    self.win.set_write_indent(300);   // Set the newline position for each line 
    self.win.set_write_pos((300,100)); 
    self.win.write("{30}{y}Double Pendulum\n"); 
    self.win.write("\nClick on either pendulum balls to set position and move pendulum ball.\n"); 
    self.win.write("{12,italic}{lightgreen}(click outside the pendulum to freeze pendulum motion)\n\n"); 
    self.win.write("Click on the screen to drop the pendulum.\n\n");
    self.win.write("Click on {g}\"Maintain Rod Length\"{/} to change pendulum angles without changing the rod length.\n\n");
    self.win.write("Use the controls to the left to change states while pendulum is in motion or before dropping the pendulum.\n\n");
    self.win.write("{p}While the pendulum is moving\n\n"); 
    self.win.write("Right-click on the screen to move the display area up and down\n"); 
    self.win.write("Use the Mouse Wheel to zoom in and out\n"); 
    self.win.write("Click on the display area to stop the pendulums so you can move them.");            
    self.win.set_write_indent(0);     // Set newline indent back to 0
}
}