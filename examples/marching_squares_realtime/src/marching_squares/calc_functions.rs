
use super::{MarchingSquares,ArrayIndex,GridIndex};

impl MarchingSquares
{

/// calc_meta_ball_array() -- This is the main calculation for the MetaBall grid, for each pixel in the array (i.e. GridSize, not window size))
///                           Depending on the Grid Size (m_iGridSize), this array can be very small or 1:1 with the window size (m_pWinSize)
/// 
/// This is a pretty small piece of code.  It's this function and the calc_marching_squares() function that create just about all of it.  The rest of the code is just
/// to organize the display and provide basic service functions to make it an organized program. 
///
/// This function simply looks at every pixel in the grid to determine the input
/// from each Circle we're running (usually in the 10-20 range or so). 
///
/// Otherwise, this routine is very simple:  Add the contribution for each circle (turning them into Meta Balls)
/// keeping those that exceed 1.0, throwing out < 1.0 as empty space. 
///
pub (crate) fn calc_meta_ball_array(&mut self,in_grid_size : i32,array_index : ArrayIndex,calc_center : bool)
{
    let mut v2 = self.v.as_mut_slice();
    if array_index == ArrayIndex::CenterV   { v2 = self.v_center.as_mut_slice(); }
    if array_index == ArrayIndex::Full      { v2 = self.v_full.as_mut_slice(); }

    let grid_size_f = in_grid_size as f32;

    let grid_width  = self.win_size.x/in_grid_size;     // Get actual size of Metaball Array Width and Height
    let grid_height = self.win_size.y/in_grid_size;

    let f_add = if calc_center { 0.5 } else { 0.0 };    // If we're calling for the 'show filled squares' where we're just calculating the center (instead of the corners)
                                                        // We add .5 to center it

    for i in 0..grid_height+1
    {
        let mut p = (i*(grid_width + 1)) as usize; 

        let fy = (i as f32 + f_add)*grid_size_f;
        let mut fx = f_add*grid_size_f;                 // Get actual X position based on the size of the window, kind-of normalizing it for different Grid Size calculations later on
        for _j in 0..grid_width+1
        {
            let mut result = 0.0;           // Total value added from contributions of all circles at this (x,y) point
                                            // (point does not need to be in the circle to contribute, which is how we get the malformed 
                                            // shapes we're calling Meta Balls)

            // This just looks at the distance from the center of *every* circle we have displaying, calculated
            // on *every* pixel, which is why the Meta Ball calculation is very demanding, and the reason why we'd
            // want to make it smaller and use interpolation to make it look just as good at the actual window size, using far less calculations
            // than 1:1 
            //
            // The calculation used is essentially r^2/(distx^2+disty^2), which:
            //    1. Will be 1.0 exactly at the border of the circle (the distance is the radius), 
            //    2. < 1 if we're outside the circle
            //    3. > 1 (and exponential to infinity) if we're inside the circle
            //
            // --> This is just a re-arrangement of the usual r = sqrt(distx^2*disty^2)
            // --> This is done (and added) for all circles, which creates our Meta Ball structure
            //
            // So, all we're doing that is different than creating a filled circle is adding the same value
            // for all other circles, which creates a blob like structure instead... that's really the entire Meta Ball algorithm
            // 
            // The rest (and most of this program) is creating the Marching Square Outline. See CalcMarchingSquares()

            for k in 0..self.num_balls as usize
            {
                let dist_x = fx-self.balls[k].loc.x;
                let dist_y = fy-self.balls[k].loc.y;
                let f = self.balls[k].radius_sq/(dist_x*dist_x+dist_y*dist_y); 
                result += f;
            }

            v2[p] = result;
            p += 1;
            fx += grid_size_f;
        }
    }
}

/// calc_marching_squares() -- Calculate the 0-15 values we need to create lines for the marching squares, based on 
/// the Meta Ball results formed in CalcMetaBallArray()
///
/// All this routine does is look at each corner we calculated and assign a value based on what's happening with the corners.
/// There are 16 cases, where 0 is no corner (of the 4 we're looking at for each value in the Meta Ball Array) -- so completely blank --
/// where value 15 is all 4 corners (completely filled).
/// 
/// This leaves us to look at values 1-14 for various lines we want to draw, as described in this web site:
/// 
///      https://jamie-wong.com/2014/08/19/metaballs-and-marching-squares/
///
/// Notes:
///
/// 1. Note multi-threading is not being used here.  It might be a little faster, but not by too much, since
///    there really are no calculations going on here.  It might be faster in terms of memory channel usage, but
///    since most of the time is in creating the Meta Ball structure and all the display graphics performed, 
///    the value of multi-threading here is negligible.
///
/// 2. These lines set are based on the mid-points of various sides (walls) of the cell we're looking at.
///    The non-interpolated and interpolated Marching Square line segments use these. 
///
///    The interpolated line segments simply do a little more by looking at the strength factor for each
///    corner (the Meta Ball array, the same source we're using here) to determine how to move the ends of the
///    line segments, which is very effective in creating a higher-resolution image. 
///
///    The non-interpolated line segments just use these values raw, so they are very jerky and low-resolution in comparison.
///
pub (crate) fn calc_marching_squares(&mut self,grid_size : i32,array_index : ArrayIndex,grid_index : GridIndex)
{
    let mut v1 = self.v.as_mut_slice();
    if array_index == ArrayIndex::Full { v1 = self.v_full.as_mut_slice(); }

    let mut v_grid = self.v_grid.as_mut_slice();
    if grid_index == GridIndex::Full { v_grid = self.v_grid_full.as_mut_slice(); }

    let grid_width  = self.win_size.x/grid_size;
    let grid_height = self.win_size.y/grid_size;

    let mut p = 0 as usize;
   
    for i in 0..grid_height
    {
        for j in 0..grid_width
        {
            let index = (j + i*(grid_width+1)) as usize;

            // Set a value based on each corner point, where >= 1.0 is 'active' 
            // and < 1.0 is inactive
            //
            // This allows us to create a binary value from 0-15, or 4 bits (on or off) for each corner.
            //
            // See: https://jamie-wong.com/2014/08/19/metaballs-and-marching-squares/
            //
            // for images of what these line segments look like.
            //
            // Also look at the draw_marching_squares() function in this program to see how these values are used.

            let p1 = if v1[index+0] >= 1.0 { 1 } else { 0 };
            let p2 = if v1[index+1] >= 1.0 { 1 } else { 0 };
            let p3 = if v1[index + (0 + grid_width+1) as usize] >= 1.0 { 1 } else { 0 };
            let p4 = if v1[index + (1 + grid_width+1) as usize] >= 1.0 { 1 } else { 0 };

            let bin = p3 + (p4 << 1) + (p2 << 2) + (p1 << 3);   // Create and store our binary value from 0-15 (line segment directions)
            v_grid[p] = bin; 
            p += 1;
     
        }
    }
}


}