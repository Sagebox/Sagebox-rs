
use super::{MarchingSquares,BallStruct,GRID_SIZE_MULTI_THRESHOLD};
use std::thread;

impl MarchingSquares
{

/// calc_meta_ball_array_threaded() -- Same as calc_meta_ball_array(), but global and oriented to a specific length 
/// of a slice starting at index 0, but with an incoming Y value
///
/// --> Note: calc_meta_ball_array() could have just been deleted and this function used instead, with the 
///           incoming values sent to this function rather than pulled directly from 'self'
///
///           I kept them separate for clarity, but basically calc_meta_ball_array() became redundant when this function was created.
///           One nice thing going for calc_meta_ball_array(), is that it has a much simpler parameter structure, since many values
///           are just referred to directly in 'self. 
///
pub (crate) fn calc_meta_ball_array_threaded(start_y : i32,num_lines : i32, in_grid_size : i32,grid_width : i32,balls : &[BallStruct],num_balls : i32,v2 : &mut [f32],calc_center : bool)
{

    // --> This code is nearly identical to calc_meta_ball_array() 
    //     See comments there for more information on what this function is performing.

    let grid_size_f = in_grid_size as f32;

    let f_add = if calc_center { 0.5 } else { 0.0 };    // If we're calling for the 'show filled squares' where we're just calculating the center (instead of the corners)
                                                        // We add .5 to center it

    for i in 0..num_lines
    {
        let mut p = (i*(grid_width + 1)) as usize;     // How to assign as usize here?

        let fy = ((i + start_y) as f32 + f_add)*grid_size_f;
        let mut fx = f_add*grid_size_f;

        // --> See preamble in calc_meta_ball_array() for comments and description

        for _j in 0..grid_width+1
        {
            let mut result = 0.0;

            for k in 0..num_balls as usize
            {
                let dist_x = fx-balls[k].loc.x;
                let dist_y = fy-balls[k].loc.y;
                let f = balls[k].radius_sq/(dist_x*dist_x+dist_y*dist_y); 
                result += f;
            }

            v2[p] = result;
            p += 1;
            fx += grid_size_f;
        }

    }

}

/// calc_meta_ball_array_threaded_job() -- Determine which jobs to do, and then call calc_meta_ball_array_threaded() to 
/// calculate the meta ball array for the current thread section
///
pub (crate) fn calc_meta_ball_array_threaded_job(v_main : Option<&mut [f32]>,v_full : Option<&mut [f32]>, v_center : Option<&mut [f32]>,start_y : i32,
            start_y_full : i32,mem_size : i32,mem_size_full : i32,in_grid_size : i32,win_width : i32,balls : &[BallStruct],num_balls : i32)
{
    let grid_width = win_width/in_grid_size;        // actual width of array we want to calculate (may be smaller than memory size)
    let num_lines  = mem_size / grid_width;         // number of lines to calculate
    let full_lines = mem_size_full / win_width;     // number of lines to calculate for the full/optimal 1:1 version
                                                    // (this does not get calculated if the general grid size is 1:1, because
                                                    //  the data for the main array is used in its place)

    // Calculate each section (i.e. chunk) of memory for each array type we want to calculate

    if v_main   != None { Self::calc_meta_ball_array_threaded(start_y,num_lines,in_grid_size,grid_width,balls,num_balls,v_main.unwrap(),false); }
    if v_full   != None { Self::calc_meta_ball_array_threaded(start_y_full,full_lines,1,win_width,balls,num_balls,v_full.unwrap(),false); }
    if v_center != None { Self::calc_meta_ball_array_threaded(start_y,num_lines,in_grid_size,grid_width,balls,num_balls,v_center.unwrap(),true); }
}

/// calc_meta_ball_arrays_threaded() -- Set up threads to multi-thread the Meta Ball Calculaton
///
/// The Meta Ball array calculation is very intensive and time-consuming.  That's why Meta Balls are such a great example of the 
/// Marching Squares, because we can use a grid size of 10x10 (100x less work!) and get pretty much the same result with a 1:1 array.
///
/// When the Grid Size is < 5, or we're showing the Optmial/Full 1:1 curve, the Meta Ball calculation uses this function to 
/// multi-thread the calculation.
///
/// ** Not the best approach **
///
/// This sets up threads for every pass, at 60fps.  The better approach would be permanent worker threads that can be suspended and then resumed, so 
/// we don't have the overhead of setting up threads.  But, that would require more re-working, and this was basically fast enough. 
///
/// On my system, a 1:1 array calculation is about 4 times faster, from 8ms to a little less than 2ms.  When added together with other display items,
/// this can make the difference between 60fps and something slower. 
///
/// This approach is probably costing upwards of 1ms by not using the worker-thread approach, so in a more optimal environment, the multi-threaded 
/// version would be roughly 8x faster. 
///
/// - Click the "Show Debug Window" option to see timing.  The check and uncheck the "multi-threaded" checkbox to turn multi-threading on or off.
/// - The max threads are limited to 10, or best fit of your processor has less than 10 cores.
///
pub (crate) fn calc_meta_ball_arrays_threaded(&mut self)
{
    let grid_width      = self.win_size.x/self.grid_size;
    let grid_height     = self.win_size.y/self.grid_size;

    // Values based on grid size
    
    let line_chunk      = (grid_height+1)/self.num_threads; // Number of lines per thread to calculate, based on grid size.
    let chunk_size  = line_chunk*(grid_width+1);        // Memory size of each section per-thread    
 
    // Values based on 1:1 full/optimal curve

    let line_chunk_full     = (self.win_size.y + 1)/self.num_threads; 
    let chunk_size_full = line_chunk_full*(self.win_size.x+1); 

    // Local values

    let win_width   = self.win_size.x;          // We get in trouble with the Borrow Checker if we use self.win_size.0 directly

    // Determine which meta ball arrays we will calculate, depending on settings. 

    let calc_full   = self.calc_full_curve && self.grid_size != 1; 
    let calc_main   = self.grid_size <= GRID_SIZE_MULTI_THRESHOLD;
    let calc_center = calc_main && self.fill_squares; 

    if calc_full || calc_main || calc_center
    {
        thread::scope(|s| 
        {
            let mut threads = vec![];       // thread handle storage

            // Get iterators for chunks for each array type

            let mut v_main = self.v.chunks_mut(chunk_size as usize);
            let mut v_full = self.v_full.chunks_mut(chunk_size_full as usize);
            let mut v_center = self.v_center.chunks_mut(chunk_size as usize);
        
            for index in 0..self.num_threads
            {
                let mem_main        = v_main.next().unwrap();           // Get the sections for the current thread index
                let mem_full        = v_full.next().unwrap();
                let mem_center      = v_center.next().unwrap();
                let start_y         = index as i32*line_chunk;          // Start Line Y value
                let start_y_full    = index as i32*line_chunk_full;     // Start Line Y value (full/optimal curve)
                let mem_size        = mem_main.len() as i32;           
                let mem_size_full   = mem_full.len() as i32;    
                let balls           = &self.balls;                          
                let num_balls       = self.num_balls;
                let grid_size       = self.grid_size;

                // Start the thread and send everything to calc_meta_ball_array_threaded_job()

                threads.push(s.spawn(move || Self::calc_meta_ball_array_threaded_job(
                    if calc_main    { Some(mem_main)   } else { None }, // Only send the arrays we decided to multi-thread
                    if calc_full    { Some(mem_full)   } else { None },
                    if calc_center  { Some(mem_center) } else { None },
                        start_y,start_y_full,mem_size,mem_size_full,grid_size,win_width,balls,num_balls
                    ))); 

            }
        });
    }
}

}