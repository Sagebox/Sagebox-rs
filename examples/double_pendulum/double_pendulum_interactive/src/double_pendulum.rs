// --------------------
// Pendulum Calculation
// -------------------- 
//
// Sagebox uses this module calculate the position of the pendulum in each frame.
//
// The only Sagebox-related function in this module is to render the frame in the Sagebox window. 
//
// Otherwise, the code is pure math, and can be found in various places on the Internet.

use sagebox::*;

const PIF64 : f64 = 3.141592653589793238462643383279;

/// Vertex type for trail display containing a position and color
/// Used so the ends of the trail can dissipate.
///
struct Vertex
{
	position : Point<f64>,
	color : RgbColor, 
}

pub struct DoublePendulum
{
  	weight_mult     		: f64,   		// Observational.  Helps with a more realistic wobbling and gravity loss effect.  
                                    	// But can be unstable with floating-point issues.
   	line_mul        		: f64,          // More observational 
   	thick_mul       		: f64,          // Line thickness multiplier for rods (display)

   	pub length      		: [f64; 2],
   	pub mass        		: [f64; 2],
   	line_thickness  		: f64,          // Starting Line thickness
   	trail_thickness 		: f64,          // Line thickness for trails
   	peg_radius      		: f64,          // Peg Radius
   	pub top_radius      	: f64,          // Radius for bobs (changeable with slider)
   	pub bot_radius      	: f64,          
   	max_trail_size  		: i32,          // Max trail point size in the display
  	pub circle_mult     	: f64,          // Bob size multiplier for changing display size of bobs
   	pub zoom            	: f64,          // Zoom factor for display
  	pub overflow_count  	: i32,          // When > 0 an overflow was detected and shut down.   (floating-point resolution issue)
   	pub pause           	: bool,         // When true, the display and update is frozen.
   	show_trail      		: bool,
   	single_pend     		: bool,
   	pub rod_vertex      	: [Point<f64>; 3],

   	pub angle           	: [f64; 2],
   	pub damp1           	: f64,          // Dampening, i.e. friction
   	damp2           		: f64,
   	pub overflow_mul		: f64,

   	pub ang_accel1			: f64,
   	pub ang_accel2      	: f64,
   	pub ang_vel1        	: f64,
   	pub ang_vel2        	: f64,

	line_color 				: RgbColor,
	gravity    				: f64, 
	_fps        			: f64,
	dt         				: f64, // 400.0 is just an observational fit.
	pos1 					: Point<f64>,
	pos2 					: Point<f64>,
	win 					: Window,
	v_trails				: Vec<Vertex>,
}

impl Default for DoublePendulum {
    fn default() -> DoublePendulum {
        DoublePendulum {
    		weight_mult     : 0.05,      
								
			line_mul        : 4.0,      
			thick_mul       : 1.0,      
							
			length          : [ 0.0, 0.0 ],
			mass            : [ 0.0, 0.0 ],
			line_thickness  : 3.0,      
			trail_thickness : 4.0,      
			peg_radius      : 5.0,      
			top_radius      : 23.0,       
			bot_radius      : 23.0,
			max_trail_size  : 300,      
			circle_mult     : 1.0,      
			zoom            : 1.0,      
			overflow_count  : 0,        
			pause           : false,    
			show_trail      : true,
			single_pend     : false,
			rod_vertex      : [ Point::<f64>::default(), Point::<f64>::default(), Point::<f64>::default() ],     

			angle           : [ 0.0, 0.0 ],
			damp1           : 0.9985,       
			damp2           : 0.9985,  
			overflow_mul	: 1.0,

			ang_accel1		: 0.0,
			ang_accel2      : 0.0,
			ang_vel1        : 0.0,
			ang_vel2        : 0.02,

			line_color		: RgbColor::new(255,50,0),
			gravity    		: 9.8,
			_fps       		: 60.0,
			dt         		: 400.0*(1.0/60.0),     // 400.0 is just an observational fit.

			pos1 			: Point::<f64>::default(),
			pos2 			: Point::<f64>::default(),
			win 			: Window::default(),
			v_trails		: vec![],
   
        }
    }
}

impl DoublePendulum
{

pub fn default() -> DoublePendulum
{
	DoublePendulum { ..Default::default() }
}
pub fn new(win : Window, length1 : f64, length2 : f64, mass1 : f64, mass2 : f64, angle1 : f64, angle2 : f64, dampen : f64, peg_y : f64) -> DoublePendulum
{
	let mut this  = DoublePendulum { ..Default::default() };
	
	// Set incoming values for initial calculation and display 

	this.length = [ length1, length2 ];
	this.mass 	= [ mass1, mass2 ];
	this.angle 	= [ angle1*PIF64/180.0, angle2*PIF64/180.0 ];
	this.damp1 	= dampen;
	this.damp2 	= dampen; 

	this.rod_vertex[0] = win.get_window_size_f64()/2.0*Point::<f64>::new(1.0,peg_y*2.0);

	this.win = win; 

	this
}


// Update the pendulum angular velocity and angles. 
//
// This routine was largely copied from Internet sources, except for some observational changes to make it work
// with the 60fps loop better, as well as the overflow checking below. 
//
pub fn update(&mut self)
{
	if self.pause { return; }

	let length1 = self.length[0]/(self.weight_mult*self.line_mul);
	let length2 = self.length[1]/(self.weight_mult*self.line_mul);

	let mass1 = self.mass[0]/10.0;
	let mut mass2 = self.mass[1]/10.0;

	if self.single_pend { mass2 = 0.0; }

	let angle1 = self.angle[0];
	let angle2 = self.angle[1];

    // Code pretty much copied from the internet. 

	let dt = self.dt*self.dt*self.weight_mult;
    let n11 = -self.gravity*(2.0*mass1+mass2)*angle1.sin();
    let n12 = -mass2*self.gravity*(angle1-2.0*angle2).sin();
    let n13 = -2.0*(angle1-angle2).sin()*mass2;
    let n14 = self.ang_vel2*self.ang_vel2*length2 + self.ang_vel1*self.ang_vel1*length1*(angle1-angle2).cos();
    let den = 2.0*mass1+mass2-mass2*(2.0*angle1-2.0*angle2).cos();
    let n21 = 2.0*(angle1-angle2).sin();
    let n22 = self.ang_vel1*self.ang_vel1*length1*(mass1+mass2);
	let n23 = self.gravity*(mass1+mass2)*angle1.cos();
  	let n24 = self.ang_vel2*self.ang_vel2*length2*mass2*(angle1-angle2).cos(); 

	self.ang_accel1 = (n11+n12+n13*n14)/(length1*den*dt);
	self.ang_accel2 = (n21*(n22+n23+n24))/(length2*den*dt);

	self.ang_vel1 += self.ang_accel1;
	self.ang_vel2 += self.ang_accel2;

	self.angle[0] += self.ang_vel1;
	self.angle[1] += self.ang_vel2;

    // Keep the angle within +/- 360 degrees to keep the floating-point resolution as high as possible.

	if self.angle[0] >  PIF64*2.0 { self.angle[0] -= PIF64*2.0; }
  	if self.angle[1] >  PIF64*2.0 { self.angle[1] -= PIF64*2.0; }
  	if self.angle[0] < -PIF64*2.0 { self.angle[0] += PIF64*2.0; }
  	if self.angle[1] < -PIF64*2.0 { self.angle[1] += PIF64*2.0; }
    
 	self.update_pos();

	self.ang_vel1 *= self.damp1*self.overflow_mul;		// Apply dampening (i.e. friction)
	self.ang_vel2 *= self.damp2*self.overflow_mul;


	let v1 = self.ang_vel1/dt;
	let v2 = self.ang_vel2/dt;

    // Check Max Potential Energy vs. Kinetic energy.  Look for overflows when Ke > Pe
    // 
    // Overflows occur because of inaccurate calculations due to time-slice resolution which can 
    // cause the pendulum to move too fast when there is little or no dampening. 
    // This routine works for a 60fps reference with 1 sample per-frame, which is a pretty big dt. 
    // The proper method is to probably call Update() 5-10 times in succession and then render it, rather than
    // just the one time.

    // Note: This equation works, but I am not completely sure why because the v1/v2 calculations are not 
    //       really correct -- the dt division is not right, so it's probably a happy accident.

 	let pe2 = -(mass1+mass2)*self.gravity*length1-mass2*self.gravity*length2; 
	let ke2 = 0.5*mass1*v1*v1*length1*length1 + 0.5*mass2*(v1*v1*length1*length1 + 
				v2*v2*length2*length2 + 2.0*v1*length1*v2*length2*(self.angle[0]-self.angle[1]).cos()); 

    // If the Kinetic energy went over the max potential energy then slow the pendulum down.
    // If we have seen more than two overflows in this session than add some dampening to 
    // keep slowing it down. 

	if ke2.abs() > pe2.abs()
	{
  		let ratio = (pe2/ke2).abs(); 
   		self.overflow_count += 1;
		self.ang_vel1 *= ratio*0.65;
		self.ang_vel2 *= ratio*0.65;

        // Start slowing it down if we've seen more than 2 hits.

 		if self.overflow_count > 2
   		{
			self.overflow_mul *= self.overflow_mul;
			if self.overflow_mul == 1.0 { self.overflow_mul = 0.9999; } 
    	}
	}
}


// Update the pendulum bob positions.
//
fn update_pos(&mut self)
{
    self.pos1 = Point::<f64>::new(self.angle[0].sin(), self.angle[0].cos())* self.length[0];
    self.pos2 = self.pos1 + Point::<f64>::new(self.angle[1].sin(), self.angle[1].cos())* self.length[1];
}


   // Reset all moving values. 
	pub fn reset_overflow(&mut self) { self.overflow_count = 0; self.overflow_mul = 1.0; }

	pub fn reset(&mut self) 
   { 
		self.ang_vel1 = 0.0;
		self.ang_vel2 = 0.0;
		self.ang_accel1 = 0.0;
		self.ang_accel2 = 0.0;
		self.reset_overflow();
		self.update_pos();
		self.v_trails.clear();
   }
}
mod display_functions;