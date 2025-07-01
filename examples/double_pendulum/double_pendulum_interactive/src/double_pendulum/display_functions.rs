use super::{DoublePendulum,Vertex,PIF64};
use sagebox::*;

impl DoublePendulum
{

// Draw the pendulum out to the window. 
//
pub fn render(&mut self)
{

	self.rod_vertex[1] = self.pos1*self.zoom + self.rod_vertex[0];
	self.rod_vertex[2] = self.pos2*self.zoom + self.rod_vertex[0];

    //Trails

	let trail_size = self.v_trails.len() as i32;

	if !self.pause
	{
		if trail_size < self.max_trail_size
		{
			// Calculate line color fade, so it disappears into the background

			let red   = self.line_color.red*trail_size/self.max_trail_size; 
			let green = self.line_color.green*trail_size/self.max_trail_size; 
			let blue  = self.line_color.blue*trail_size/self.max_trail_size; 

			self.v_trails.push(Vertex{ position : self.pos2, color : RgbColor::new(red,green,blue) });
		}
		else 
		{
			// Move the position to the left, but not the color. 

			for i in 1..self.max_trail_size as usize { self.v_trails[i-1].position = self.v_trails[i].position; }
			self.v_trails.last_mut().unwrap().position = self.pos2; 
		}
	}

    // Draw the lines.  The DrawLineToEx() call below does a Draw To, except for the first call where it just sets the point. 
    //                  This allows just the one call so we can use the STL loop format (otherwise we'd have to get the size and
    //                  independently address each member with size()-1) loop.

	if !self.single_pend && self.show_trail
	{
		for (index, trail) in self.v_trails.iter().enumerate()
		{
			self.win.draw_line_to_ex_s(index == 0,trail.position*self.zoom+self.rod_vertex[0],trail.color,kw::pen_size(self.trail_thickness as f32)); 
		}
	}
	self.win.draw_line_s(self.rod_vertex[0],self.rod_vertex[1],sage_color::White,kw::pen_size((self.thick_mul*self.line_thickness*self.zoom) as f32)); 

	if !self.single_pend { self.win.draw_line_s(self.rod_vertex[1],self.rod_vertex[2],sage_color::Cyan,
														kw::pen_size((self.thick_mul*self.line_thickness*self.zoom) as f32)); }

	self.win.fill_circle(self.rod_vertex[0],self.thick_mul*self.peg_radius*self.zoom,sage_color::White); 

	self.win.fill_circle_s(self.rod_vertex[1],self.top_radius*self.zoom*self.circle_mult,sage_color::Red,kw::pen_size(2.5));

	if !self.single_pend { self.win.fill_circle_s(self.rod_vertex[2],
						self.bot_radius*self.zoom*self.circle_mult,sage_color::Green,kw::pen_size(2.5)); }

}



pub fn display_values(&self)
{
	let ref win = self.win; 

	win.set_write_indent(10);    // Set leftmost column at 10 pixels out we can just use \n without setting it for each line.
	win.set_write_padding(5);    // Add some space between lines when writing to the window (for nicer display)

	win.set_fg_color(sage_color::Gray172);         // Set the text color.  Also can be simpl "Gray172" in quotes.
	
	 // {g}  green, {c} = cyan.  {x=130) sets the X write position to that value so things line up

	win.set_write_pos((10,10)); 
	win.writeln(format!("Mass 1:{{x=130}}{{g}}{:.2}",self.mass[0]));

    win.writeln(format!("Mass 2:{{x=130}}{{g}}{:.2}",          self.mass[1])); 
    win.writeln(format!("Length 1:{{x=130}}{{g}}{:.2}",        self.length[0])); 
    win.writeln(format!("Length 2:{{x=130}}{{g}}{:.2}",        self.length[1])); 
    win.writeln(format!("Dampening:{{x=130}}{{g}}{:.6}\n",       (1.0-(self.damp1*self.overflow_mul)))); 
    win.writeln(format!("Ang Accel 1:{{x=130}}{{c}}{:.6}",       self.ang_accel1)); 
    win.writeln(format!("Ang Accel 2:{{x=130}}{{c}}{:.6}",       self.ang_accel2)); 
    win.writeln(format!("Ang Velocity 1:{{x=130}}{{c}}{:.6}",    self.ang_vel1)); 
    win.writeln(format!("Ang Velocity 2:{{x=130}}{{c}}{:.6}",    self.ang_vel2)); 
    win.writeln(format!("Angle 1:{{x=130}}{{c}}{:.2}\u{b0}",     (self.angle[0]*180.0/PIF64))); 
    win.writeln(format!("Angle 2:{{x=130}}{{c}}{:.2}\u{b0}\n",   (self.angle[1]*180.0/PIF64))); 

	// If there has been an overflow, then display it in red.  This means the math/float-point had
	// some sort of resolution problem, usually when the bob starts wobbling severely.

	// Only display it if we've have more than two, since they can happen at end points here and there. 

	if self.overflow_count > 2
	{
		win.writeln(format!("\n\n\n\n{{r}}Math Overflow ({})\n",self.overflow_count-2)); 
		win.write("{{12}}Try increasing dampening (also lowering weight values and weight ratios).");
	}

	win.set_write_padding(0);    // Reset Padding (since we may be displaying instructions)
}
}