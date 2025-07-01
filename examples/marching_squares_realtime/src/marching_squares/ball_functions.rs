
use super::{MarchingSquares,BallStruct};
use sagebox::*;

impl MarchingSquares
{

/// UpdateBallPositions() -- Update the position of each circle, based on its current position and angle of movement (speed in X and Y
/// axes, respectively)
///
pub (crate) fn update_ball_positions(&mut self)
{
    let mult = self.speed as f32/60.0;  // The value is divided by 60 to make it slower then the slider value
                                        // (allowing a simple slider of 0-100, rather than a floating-point slider of 
                                        //  1.0-1.6, which we can do, but looks more complicated to the user)

    // If any edge is touching the edge of the window, rotate the angle appropriately so it bounces off. 
    // When an edge is hit, we just negate the speed direction for that axis (e.g. if we hit the right wall, we negate
    // dir X-axis speed for that circle (from positive to negative)).

        for i in 0..self.num_balls as usize
        {
            let mut loc     = self.balls[i].loc;
            let mut step    = self.balls[i].step;
            let     radius  = self.balls[i].radius as f32;
            let     win_size = self.win_size.f32();

            if self.balls[i].loc.x + radius as f32 >= win_size.0-1.0
            {
                loc.x = self.win_size.x as f32-1.0-radius;          // We went over, so adjust back to the edge of the wall
                step.x = -step.x;                                   // Negate the direction for this axis
            }   
            
            if self.balls[i].loc.y  + radius >= win_size.1-1.0
            {
                loc.y = self.win_size.y as f32 - 1.0 - radius;
                step.y = -step.y; 
            }

            if self.balls[i].loc.x - radius <= 0.0
            {
                loc.x = radius+1.0; 
                step.x = -step.x; 
            }           
                
            if self.balls[i].loc.y - radius <= 0.0
            {
                loc.y = radius + 1.0; 
                step.y = -step.y; 
            }

            self.balls[i].loc += step * mult * self.step_mul;
            self.balls[i].step = step;
        }
}


/// InitBalls() -- Initialize the Meta Ball Array
///
/// Mostly, this just defines a number of circles based on radius and other criteria, setting:
///
///      1. The radius of each circle
///      2. The starting location, 
///      3. How fast it moves in a specific direction as it drifts across the window
///      4. The color of the circle
///
/// These circles are then used to calculate the Meta Ball structure
///
/// The Metaball Calculation itself is pretty easy -- it's just a modified way of looking at a circle, but then
/// doing this for ALL circles and deciding if we're inside or outside the Meta Ball structure based on our value (higher is inside, lower is outside)
///
pub (crate) fn init_balls(&mut self)
{
    // Initialize all circles, even if they are not showing.
    // This allows us to change the number of circles showing without
    // changing the ones we're seeing, as all circles are maintained even when they are not displaying
    
    for _i in 0..self.max_balls
    {
        // Get a random radius between the min and max radius
        // Also get an X and Y point in the window
        // --> This uses the Sagebox random tools, but any random function that returns integers will do.   
        //     (std::rand is not used, as it is much slower than a generalized random number generator)

        let radius  = Sagebox::rand_range(self.min_radius,self.max_radius+1);
        
        // Dunno why, but I though having an angle would be a better way to describe motion.
        // Realistically, I'm just assigning a random X and Y step movement.. Overkill code, basically
        
        let angle   = (Sagebox::rand(360) as f32)*3.14159/180.0;    // Convert to radians       

        let ball    = BallStruct
        {   
            radius      : radius,
            radius_sq   : (radius*radius) as f32,
            loc         : Point::<f32>::new(Sagebox::rand_range(radius,self.win_size.x - radius*2) as f32,
                                Sagebox::rand_range(radius,self.win_size.y - radius*2) as f32) ,
            step        : Point::<f32>::new(angle.cos(),angle.sin()),
            color       : Sagebox::get_color(self.color_strings[Sagebox::rand(10) as usize]),       // Light blue for now. 
        };
        self.balls.push(ball);
    }
}


}