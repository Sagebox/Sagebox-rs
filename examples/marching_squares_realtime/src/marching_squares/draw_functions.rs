
use super::MarchingSquares;
use sagebox::*;
impl MarchingSquares
{

/// draw_corner_points() -- this draws the corner points of each grid intersection where points are evaluated: 
///
/// When the 'Show Filled Squares' checkbox is not active, the points are drawn for each corner that is analyzed in the grid, to make the 4 points
/// that are analyzed instead of the center point as in non-interpolated, generic pixel-based rendering.
///
/// When the 'Show Fille Squares' checkbox is active (and the Raw Marching Squares or Interpolated Marching Squares are not displaying),
/// the points shown are the center of the grid location, representing a pixel-by-pixel analysis rather than using the Marching Squares technique 
///
/// Note: the incoming 'active_points' boolean tells the function display active corner points (cyan) or non-active ones (dark gray)
/// - This is because this function is called twice: 
///   - Once to show the inactive dots underneath any marching-square outline (because otherwise the dots
///           are placed over it, which makes them look broken)
///   - Then again to show the active (cyan) dots, so that they are much more clear over any interior flood-fill.
///
pub (crate) fn draw_corner_points(&self,active_points : bool)
{   
    let grid_width  = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    // If we have a grid size of 20 or greater, use high-performance graphics to get a nice round shape.
    // Otherwise, the objects are pretty small and it doesn't matter, and when they are small, there are a lot of them,
    // so we want the speed anyway (this program's constructor sets the smoothing quality to lower-quality for performance, but
    // we temporarily set it to higher-quality here when appropriate, setting it back at the bottom of the function)

    if self.grid_size >= 20 { self.win.set_smoothing_mode(GdiSmoothingMode::HighQuality); } 
    
    let mut v_iter = self.v.iter();

    // Show 'center' dots instead in accordance with notes above the function.      
    // i.e. when FilledSquares is true and there are no displaying Marching Squares output.

    let show_squares = self.fill_squares && !self.show_raw_outline && !self.show_int_outline;

    if show_squares { v_iter = self.v_center.iter(); }          // Change to centered Metaball array if we're showing the center values 
                                                                // (different results, corner vs. center)
    let add = if show_squares { self.grid_size/2} else { 0 };   // Dot displacement depending on whether we're centered or not
    for i in 0..grid_height
    {
        let y = i*self.grid_size;

        for j in 0..grid_width
        {
            let x = j*self.grid_size;

            // Change the size of the output rectangle, depending on grid size.
            // We could just calculate it, but this worked fine without having to test a mathematical approach

            let radius = if self.grid_size < 10 { 1.0 } else { if self.grid_size < 20 { 2.0 } else { if self.grid_size < 30 { 2.15 } else { 2.5 } } };

            let mut color = self.color_corner_inactive; // Color to show inactive circle
            let mut show_circle = false;

            // Only show the circle if it is a) active and we want to show active corner point circles, or 
            //                               b) inactive and we want to show inactive corner point circles

            if *v_iter.next().unwrap() >= 1.0
            {
                color = self.color_corner_active;       // Set color for active circle
                if active_points { show_circle = true; }
            }
            else if !self.show_grid && self.grid_size >= 5 && !active_points { show_circle = true; }

            if show_circle
            {
                // If the Grid Size is <= 10, then print out integer-based rectangles, as they are much
                // faster to draw than circles, since they are very small and numerous when the grid size is so small.
                // (this is really overkill, but can be a little faster to use squares vs. circles when printing thousands vs. hundreds 
                // -- but only by 500us to 1ms in total, but this program has a *lot* of real-time things going on, so 500us-1ms is 
                //    a lot when rendering 60fps)

                if self.grid_size <= 10 { self.win.fill_circle(Point::<f32>::fromi32((x,y)) + add, radius, color); } // $$ fill_rectangle
                else                    { self.win.fill_circle(Point::<f32>::fromi32((x,y)) + add, radius, color); }
            }
        }
        v_iter.next();      // Overlap the extra value we use to construct the corner points for the right edge 
    }

    // If we set high-quality graphics, set the graphic quality back to standard/lower-quality to keep performance up.

    if self.grid_size >= 20 { self.win.set_smoothing_mode(GdiSmoothingMode::None); } 
}

/// print_corner_values() -- Print the strength values for each corner point.
///
/// The value printed is the value for the corner next to the output, except:
///   
///      --> If the 'Show Filled Squares' checkbox is checked and no Marching Square output is 
///          displaying (i.e. Raw or Interpolated), the value is the value of the pixel-based version, which equates to the
///          center of the grid square (even though it displays in the upper-right corner)
///
///          When these values are used and Show Corner Points checkbox is checked, the corner points display in the center 
///          of the grid square (vs. the upper-right as they appear when the marching-square algorithm results is being displayed)
///
pub (crate) fn print_corner_values(&self)
{
    let grid_width  = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    // Determine we're showing 'filled squares' and no Marching Square display is active (e.g. Raw or Interpolated results)

    let mut v_iter = if self.fill_squares { self.v_center.iter() } else { self.v.iter() }; 

    for i in 0..grid_height
    {
        let y = i*self.grid_size;

        for j in 0..grid_width
        {
            let x = j*self.grid_size;
    
            let f = *v_iter.next().unwrap();    // Next value in Metaball array

            if self.grid_size >= 40
            {
                // Display different values based on a point that is > 1.0 (i.e. within the Metaball structure)
                // or not (< 1.0 is outside of the structure and not part of the Metaball calculation, so it is printed in gray)

                self.win.set_fg_color(if f > 1.0 { self.color_value_active } else { self.color_value_inactive });

                // Print a different floating-point width if our Grid Size is < 40 so it will fit easier without reducing the font size

                if self.grid_size > 40 { self.win.write_xy((x + 2,y + 1),format!("{:.4}",f)); }
                else                   { self.win.write_xy((x + 2,y + 1),format!("{:.3}",f)); }
            }
        }
        v_iter.next();      // Overlap next value in Metaball array
    }
}

/// Draw Gridlines in the window, with each square representing two things:
///
/// 1. The up-sampled pixel-size (relative to the window) of the size of the Metaball array we're analyzing (e.g. 5x5, 20x20, 1x1, etc.)
/// 2. Each corner represents the 4 values taken for that square area to construct the Marching Square lines, both Raw and Interpolated versions
/// 
/// --> Note:  This function is relatively inefficient for two reasons: 
///
/// 1. It's a little expensive to draw these lines all the time, especially with smaller grid sizes.
///    
///    A few alternatives:
///
///    1. Fill these values in the window's bitmap directly (since this program is not using the GPU)
///       This would be in the range of 5x-10x faster
///
///    2. Use a path via the GDI (or equivalent through Sagebox) so that the GDI can deal with the lines en-mass
///    3. Construct a set of connected lines that connect outside of the window and then use DrawLines() to draw them at once
///       (this would probably be as fast as directly writing the lines to the window's memory)
///
/// 2. A way to make this faster is to draw this once, save it to a bitmap and then redraw the bitmap each subsequent time it is 
///    drawn. 
///
///    The re-draw into a cleared bitmap (again, just once) if the grid size is changed.
///
///
/// --> If you see code below that uses a bitmap, then I took my own advice and implemented step #2, as this code does slow things
///     down significantly when getting into grid sizes of 5x5 or less. 
///
pub (crate) fn draw_grid_lines(&self)
{

    let grid_width = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    // Draw Grid Lines - first horizontal lines, then vertical lines.

    for i in 0..grid_height
    {
        self.win.draw_line_s((0,i*self.grid_size),(self.win_size.x,i*self.grid_size),self.grid_line_color,kw::pen_size(self.grid_line_size));
    }
    for i in 0..grid_width
    {
        self.win.draw_line_s((i*self.grid_size,0),(i*self.grid_size,self.win_size.y),self.grid_line_color,kw::pen_size(self.grid_line_size));
    }
}

/// DrawCircles() -- Draw the base circles from which the Meta Ball shape is created (and the Marching Squares Algorithm defines as an outlined and filled shape)
///
pub (crate) fn draw_circles(&self)
{
    for i in 0..self.num_balls as usize
    {
        let ref g = &self.balls[i];
        self.win.fill_circle_s(g.loc,g.radius as f32,g.color,kw::pen_color(g.color) + kw::opacity(94));
    }
}

pub (crate) fn draw_marching_squares_performance(&mut self)
{

    let grid_width  = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    let gdi = &self.win.gdi;
    
    let mut grid_index = 0 as usize;
    let mut v_index    = 0 as usize;    // Index into Meta Ball result array, telling us the strength of the calculation
    let v = self.v.as_slice();          // Get the Meta Ball Array we calculated specifically for a pixel version (i.e. centered, not marching-square corners)

    let grid_size_f = self.grid_size as f32;
    let grid_div_2  = grid_size_f/2.0;

    let v_grid  = self.v_grid.as_mut_slice();       
    let p1      = self.p_grid.as_mut_slice();       
    let p2      = self.p_grid2.as_mut_slice();       

    let mut p_grid_index = (2 + (grid_width+2)*2) as usize;

    let brush_int   = gdi.new_solid_brush_s(self.color_int_interior);
    let brush_raw   = gdi.new_solid_brush_s(self.color_raw_interior);
 
    let path_int    = gdi.new_path();
    let path_raw    = gdi.new_path();

    for i in 0..grid_height
    {
        let iy = i*self.grid_size;
        let y  = iy as f32;

        let mut j = 0;
        while j < grid_width //for mut j in 0 ..grid_width
        {          
            let mut ix = j*self.grid_size;
            let x = ix as f32;
            let mut b = v_grid[grid_index];  // We know it won't go over
            grid_index += 1;

            let mut rect_count = 0;
            while b == 15
            {
                rect_count += 1;   // Increase rectangle size to avoid drawing them all individually

                b = v_grid[grid_index]; // We know it won't go over         // Get the next line-segment type value
                grid_index += 1;
                j += 1;                            // Increase the loop count
                if j >= grid_width { break; }     // If we're over the loop maximum, exit the loop, which will take care of the pending rectangle
            }

            if rect_count > 0
            {
                ix += rect_count*self.grid_size;
                v_index += rect_count as usize;

                p_grid_index  += (rect_count*2) as usize;           // Same for line-segment output arrays
               
               // Add the larger, calculated rectangle for mass output later.
       
                if self.show_int_interior { path_int.add_rectangle((ix-rect_count*self.grid_size, iy),(self.grid_size*rect_count,self.grid_size)); }
                if self.show_raw_interior { path_raw.add_rectangle((ix-rect_count*self.grid_size, iy),(self.grid_size*rect_count,self.grid_size)); }

               // j -= 1;    // Subtract loop value since we're about to continue and we already increased it in the upper while() loop
                grid_index -= 1;   // Same thing value index value 

               continue; // Start at the top of the loop
                         // We could drop through, but there are a number of calculations (notably pUL, pUR, etc.)
                         // These are discrete and we could just re-calculate them and drop through with no problem,
                         // but if any modifications happen in the future, this drop-through will break the routine and it
                         // will be difficult to figure out what happened, not to mention maintaining dual sets of code
                         // (because we don't want to make it a function call in such a tight real-time loop). 
                         // So, as might as well continue rather than drop through, even if it is technically more efficient.
            }

 
            macro_rules! get_uc { ($var1:ident) => { let $var1 = Point::<f32>::new(x + grid_div_2,y); } }
            macro_rules! get_lc { ($var1:ident) => { let $var1 = Point::<f32>::new(x,y + grid_div_2); } }
            macro_rules! get_bc { ($var1:ident) => { let $var1 = Point::<f32>::new(x + grid_div_2 ,y + grid_size_f); } }
            macro_rules! get_rc { ($var1:ident) => { let $var1 = Point::<f32>::new(x + grid_size_f,y + grid_div_2); } }

            macro_rules! get_ul { ($var1:ident) => { let $var1 = Point::<f32>::new(x,y); } }
            macro_rules! get_ur { ($var1:ident) => { let $var1 = Point::<f32>::new(x + grid_size_f,y); } }
            macro_rules! get_bl { ($var1:ident) => { let $var1 = Point::<f32>::new(x,y + grid_size_f); } }
            macro_rules! get_br { ($var1:ident) => { let $var1 = Point::<f32>::new(x + grid_size_f,y + grid_size_f); } }

            get_ul!(p_ul);
            get_ur!(p_ur);
            get_bl!(p_bl);
            get_br!(p_br);

            macro_rules! f_ul { () => { v[v_index + 0] } }
            macro_rules! f_ur { () => { v[v_index + 1] } }
            macro_rules! f_bl { () => { v[v_index + (0 + grid_width + 1) as usize] } }
            macro_rules! f_br { () => { v[v_index + (1 + grid_width + 1) as usize] } }

            macro_rules! get_prc { ($var1:ident) => { let $var1 = p_ur + (p_br-p_ur)*((1.0 - f_ur!())/(f_br!()-f_ur!())); } }
            macro_rules! get_pbc { ($var1:ident) => { let $var1 = p_bl + (p_br-p_bl)*((1.0 - f_bl!())/(f_br!()-f_bl!())); } }
            macro_rules! get_puc { ($var1:ident) => { let $var1 = p_ul + (p_ur-p_ul)*((1.0 - f_ul!())/(f_ur!()-f_ul!())); } }
            macro_rules! get_plc { ($var1:ident) => { let $var1 = p_ul + (p_bl-p_ul)*((1.0 - f_ul!())/(f_bl!()-f_ul!())); } }

            macro_rules! int_points{ ($p_1:ident,$p_2:ident) => { p1[p_grid_index] = $p_1.f32(); p1[p_grid_index + 1] = $p_2.f32(); } }
            macro_rules! raw_points{ ($p_1:ident,$p_2:ident) => { p2[p_grid_index] = $p_1.f32(); p2[p_grid_index + 1] = $p_2.f32(); } }

            macro_rules! int_poly { ($array:expr) => { let xs = $array; path_int.add_polygon_f(&xs); } } 
            macro_rules! raw_poly { ($array:expr) => { let xs = $array; path_raw.add_polygon_f(&xs); } } 

            match b
            {
                1 => { get_lc!(lc); get_bc!(bc); get_bl!(bl); get_plc!(plc); get_pbc!(pbc);  
                       int_poly!([bl,plc,pbc]);                  raw_poly!([bl,lc,bc]); 
                       int_points!(plc,pbc);                     raw_points!(lc,bc); } 

                2 => { get_bc!(bc); get_rc!(rc); get_br!(br); get_pbc!(pbc); get_prc!(prc); 
                       int_poly!([br,prc,pbc]);                  raw_poly!([br,rc,bc]); 
                       int_points!(pbc,prc);                     raw_points!(bc,rc); }

                3 => { get_lc!(lc); get_rc!(rc); get_bl!(bl); get_br!(br); get_plc!(plc); get_prc!(prc); 
                       int_poly!([prc,plc,bl,br]);               raw_poly!([rc,lc,bl,br]); 
                       int_points!(plc,prc);                     raw_points!(lc,rc); }

                4 => { get_uc!(uc); get_puc!(puc); get_rc!(rc); get_prc!(prc); get_ur!(ur);  
                       int_poly!([ur,puc,prc]);                  raw_poly!([ur,uc,rc]);
                       int_points!(puc,prc);                     raw_points!(uc,rc); }

                5 => { get_lc!(lc); get_plc!(plc); get_uc!(uc); get_puc!(puc); get_ur!(ur);  
                       get_bl!(bl); get_rc!(rc); get_prc!(prc); get_bc!(bc); get_pbc!(pbc);   
                       int_poly!([plc, puc, ur, prc, pbc, bl]);  raw_poly!([lc, uc, ur, rc, bc, bl]);   
                       int_points!(plc,puc);                     raw_points!(lc,uc);  }

                6 => { get_uc!(uc); get_puc!(puc); get_bc!(bc); get_pbc!(pbc); get_ur!(ur); get_br!(br);      
                       int_poly!([puc,ur,br,pbc]);               raw_poly!([uc,ur,br,bc]);          
                       int_points!(puc,pbc);                     raw_points!(uc,bc);   }

                7 => { get_lc!(lc); get_plc!(plc); get_uc!(uc); get_puc!(puc); get_ur!(ur); get_br!(br); get_bl!(bl);
                       int_poly!([plc,puc, ur,br,bl]);           raw_poly!([lc,uc,ur,br,bl]);       
                       int_points!(plc,puc);                     raw_points!(lc,uc);     }

                8 =>  { get_lc!(lc); get_plc!(plc); get_uc!(uc); get_puc!(puc); get_ul!(ul); 
                        int_poly!([ul ,plc,puc]);                raw_poly!([ul,lc,uc]);
                        int_points!(plc,puc);                    raw_points!(lc,uc); }    

                9 =>  { get_uc!(uc); get_puc!(puc); get_bc!(bc); get_pbc!(pbc); get_ul!(ul); get_bl!(bl); 

                        int_poly!([ul ,puc,pbc,bl]);             raw_poly!([ul,uc,bc,bl]);          
                        int_points!(puc,pbc);                    raw_points!(uc,bc); }

                10 => { get_lc!(lc); get_plc!(plc); get_bc!(bc); get_pbc!(pbc); get_ul!(ul); get_br!(br); 
                        get_uc!(uc); get_puc!(puc); get_rc!(rc); get_prc!(prc);    
                        int_poly!([plc,ul ,puc,prc, br, pbc]);   raw_poly!([lc,ul, uc, rc, br, bc]); 
                        int_points!(plc,pbc);                    raw_points!(lc,bc);   }

                11 => { get_uc!(uc); get_puc!(puc); get_rc!(rc); get_prc!(prc); get_ul!(ul); get_br!(br); get_bl!(bl);            
                        int_poly!([ul ,puc,prc,br , bl]);        raw_poly!([ul, uc,rc, br, bl]); 
                        int_points!(puc,prc);                    raw_points!(uc,rc);  }

                12 => { get_lc!(lc); get_plc!(plc); get_rc!(rc); get_prc!(prc); get_ul!(ul); get_ur!(ur); 
                        int_poly!([plc,ul ,ur ,prc]);            raw_poly!([lc,ul,ur,rc]); 
                        int_points!(plc,prc);                    raw_points!(lc,rc);  } 

                13 => { get_bc!(bc); get_pbc!(pbc); get_rc!(rc); get_prc!(prc); get_ul!(ul); get_bl!(bl); get_ur!(ur);  
                        int_poly!([pbc,bl ,ul ,ur , prc]);       raw_poly!([bc,bl, ul, ur, rc]); 
                        int_points!(pbc,prc);                    raw_points!(bc,rc);  }

                14 => { get_lc!(lc); get_plc!(plc); get_bc!(bc); get_pbc!(pbc); get_ul!(ul); get_br!(br); get_ur!(ur);   
                        int_poly!([plc,ul ,ur ,br , pbc]);       raw_poly!([lc,ul, ur, br, bc]); 
                        int_points!(plc,pbc);                    raw_points!(lc,bc);  }

                _ => {}
            }
            p_grid_index += 2;
            v_index += 1;
            j += 1;
        }
        p_grid_index += 4;
        v_index += 1;
    }
    if self.show_raw_interior { gdi.fill_path(&path_raw, &brush_raw); }
    if self.show_int_interior { gdi.fill_path(&path_int, &brush_int); }

    if self.show_int_outline || self.show_raw_outline { self.draw_marching_square_segments(); }

}


/// draw_filled_squares() -- Draw the squares that represent the pure pixelated version that is 
///                          what we do most of the time, degarding the image to the number of pixels we have
///                          and then upscaling to our window size. 
///
/// This shows life without using the Marching Square algorithm where we just calculate the one pixel value in the center and
/// then stretch our image to the size of the window
///
/// Notes:
///
/// 1. This function used a Graphic Path.  This is also known as a 'Shape' in things like P5 and probably other platforms.
///    This is done because it can be much faster than drawing the rectangles independently, because it can do them all at once.
/// 2. GDI functions are being used temporarily while Sagebox functions are being finished, which include functions to use Paths
/// 3. The rectangles are not drawn on a rectangle-by-rectangle basis. They are collected in a line on each row so that we can
///    output one big rectangle as much as we can, which makes things a lot faster when we have small grid sizes (i.e. 5x5, 2x2, etc.)
/// 4. We could directly output to the Window Memory, which would be much faster.  This is an upcoming set of Sagebox functions.
///
pub (crate) fn draw_filled_squares(&mut self)
{
    let path = self.win.gdi.new_path();
    let brush = self.win.gdi.new_solid_brush_s(self.color_filled_square);

    let gdi = &self.win.gdi;                    // Get the GDI graphic object for this Sagebox window
    let v1 = self.v_center.as_mut_slice();      // Get the Meta Ball Array we calculated specifically for a pixel version (i.e. centered, not marching-square corners)

    let grid_width = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    let mut p = 0;
    
    for i in 0..grid_height
    {
        let y = i*self.grid_size;
        let mut rect_count = 0;
        for j in 0..grid_width
        {
            let x = j*self.grid_size;

            // If we have a value, just count it as another rectangle, but don't do anything.
            // If we don't have a value, form a large rectangle based on the number of squares we found so far.

            if v1[p] >= 1.0 { rect_count+=1; }
            else
            {   // Add a rectangle to the path for later display
                if rect_count > 0 { path.add_rectangle((x-rect_count*self.grid_size,y),(self.grid_size*rect_count,self.grid_size)); }
                rect_count = 0; // Reset rectangle count
            }
            p += 1;     // Move to next value
        }
        
        // Put out any remaining rectangles (as a long, single rectangle)   
        if rect_count > 0 { path.add_rectangle(((grid_width-rect_count)*self.grid_size,y),(self.grid_size*rect_count,self.grid_size)); }
        p += 1; // Adjust for 1-value overlap
    }
    gdi.fill_path(&path,&brush);        // Draw all rectangles at once.
}


/// draw_marching_squares_full() -- Draws the Marching Square Outline for 1:1 curve (Show Optimal Curve checkbox)
///
/// This function does exactly the same thing as draw_marching_squares_performance() -- refer to header notes there.
///
/// This is a scaled down version of draw_marching_squares_performance(), with the following changes:
///
/// 1. It only looks at the line-segment generation for the optimal 1:1 curve, so the code for generating the Raw Curve and both
///    interior regions don't exist as it does in draw_marching_squares_performance().
/// 2. Since the Grid Size is 1, all of the factors where the value m_iGridSize was used in various calculations have been removed, 
///
/// Performance Note: 
///
/// In this function, we're looking at a 1:1 function, so we just really need to calculate the center values and set a pixel.
/// In many, many ways, the connecting of line segments of this function is totally overkill.
///
/// A better way to do it would be to just put it out to memory as a set of pixels, which would be a 10-line routine at most.
/// But, we'd have to put out a circular shape, possibly with some aliasing, and that adds more development time.
/// This would be worth if for a long-term release program, but as something where we're just experimenting and demonstrating,
/// it didn't make sense.
/// 
/// I original tried other methods to optimized it, but they all had missing parts or jagged edges, so I just kept this approach since it
/// was easy to copy from the original routine and strip what was needed.
///
/// Basically, this routine is overkill in a large sense, but also makes sense because it already worked above and didn't require
/// any real development time besides stripping unnecessary components and a little testing.
///
pub (crate) fn draw_marching_squares_full(&mut self)
{

    let grid_width  = self.win_size.x;
    let grid_height = self.win_size.y;
    
    let mut grid_index = 0 as usize;
    let mut v_index    = 0 as usize;    // Index into Meta Ball result array, telling us the strength of the calculation

    // Get the Meta Ball Array we calculated specifically for a pixel version (i.e. centered, not marching-square corners)

    let v       = if self.grid_size == 1 { self.v.as_mut_slice()      } else { self.v_full.as_mut_slice()      };   // When the grid size is one, we use the main array 
                                                                                                                    // because the Full grid was never generated to save processing time.             
    let v_grid  = if self.grid_size == 1 { self.v_grid.as_mut_slice() } else { self.v_grid_full.as_mut_slice() };   // Same thing here


    let p3      = self.p_grid3.as_mut_slice();              // Output line-segment (f32,f32) array 
    
    let mut p_grid_index = (2 + (grid_width+2)*2) as usize; // Start index into array 

    for i in 0..grid_height
    {
        let y  = i as f32;

        let mut j = 0;
        while j < grid_width
        {          
            let x = j as f32;
            let b = v_grid[grid_index] & 0x0F;  // And-out any values that were marked by the previous drawing routine 
            grid_index += 1;

            // See notes about all these values and macros in the original function draw_marching_squares_performance()

            let p_ul = Point::<f32>::new(x,y);
            let p_ur = Point::<f32>::new(x + 1.0,y);
            let p_bl = Point::<f32>::new(x,y + 1.0);
            let p_br = Point::<f32>::new(x + 1.0,y + 1.0);

            macro_rules! f_ul { () => { v[v_index + 0] } }
            macro_rules! f_ur { () => { v[v_index + 1] } }
            macro_rules! f_bl { () => { v[v_index + (0 + grid_width + 1) as usize] } }
            macro_rules! f_br { () => { v[v_index + (1 + grid_width + 1) as usize] } }

            macro_rules! get_prc { ($var1:ident) => { let $var1 = p_ur + (p_br-p_ur)*((1.0 - f_ur!())/(f_br!()-f_ur!())); } }
            macro_rules! get_pbc { ($var1:ident) => { let $var1 = p_bl + (p_br-p_bl)*((1.0 - f_bl!())/(f_br!()-f_bl!())); } }
            macro_rules! get_puc { ($var1:ident) => { let $var1 = p_ul + (p_ur-p_ul)*((1.0 - f_ul!())/(f_ur!()-f_ul!())); } }
            macro_rules! get_plc { ($var1:ident) => { let $var1 = p_ul + (p_bl-p_ul)*((1.0 - f_ul!())/(f_bl!()-f_ul!())); } }

            macro_rules! add_points{ ($p_1:ident,$p_2:ident) => { p3[p_grid_index] = $p_1.f32(); p3[p_grid_index + 1] = $p_2.f32(); } }
 
            // Match the value for the line segment type.  
            // --> Skip processing 0 of 0x0F -- this may or may not make things faster by
            //     not going through the match table, but it's worth a try since at least it
            //     won't be slower. 

            if b > 0 && b < 0x0F
            {
                match b
                {
                    1 =>  { get_plc!(plc); get_pbc!(pbc); add_points!(plc,pbc); } 
                    2 =>  { get_pbc!(pbc); get_prc!(prc); add_points!(pbc,prc); }
                    3 =>  { get_plc!(plc); get_prc!(prc); add_points!(plc,prc); }
                    4 =>  { get_puc!(puc); get_prc!(prc); add_points!(puc,prc); }
                    5 =>  { get_plc!(plc); get_puc!(puc); add_points!(plc,puc); }
                    6 =>  { get_puc!(puc); get_pbc!(pbc); add_points!(puc,pbc); }
                    7 =>  { get_plc!(plc); get_puc!(puc); add_points!(plc,puc); }
                    8 =>  { get_plc!(plc); get_puc!(puc); add_points!(plc,puc); }    
                    9 =>  { get_puc!(puc); get_pbc!(pbc); add_points!(puc,pbc); }
                    10 => { get_plc!(plc); get_pbc!(pbc); add_points!(plc,pbc); }
                    11 => { get_puc!(puc); get_prc!(prc); add_points!(puc,prc); }
                    12 => { get_plc!(plc); get_prc!(prc); add_points!(plc,prc); } 
                    13 => { get_pbc!(pbc); get_prc!(prc); add_points!(pbc,prc); }
                    14 => { get_plc!(plc); get_pbc!(pbc); add_points!(plc,pbc); }

                    _ => {}
                }
            }
            p_grid_index += 2;      // Go to next point output (skips by 2 values for line segment point set)
            v_index += 1;           // Go to next Meta Ball array value
            j += 1;
        }
        p_grid_index += 4;          // Adjust for two-value overlap on each line
        v_index += 1;               // Meta Ball array overlaps by 1 value
    }

    self.draw_marching_square_segments_full();  // Draw the collected segments, creating as many chained line-segments as possible. 

}

/// draw_marching_square_segments() -- Draw the Marching Square Line Segments in as much order as possible to group them
///                                    together as long singular polygon segments. 
///
/// The entire purpose of this function is to put together connecting line-segments so that we may call draw_lines()/add_lines() and draw as many 
/// connecting lines as possible, rather than drawing each line segment individually.  This is essentially 4x-5x faster, 
/// and connects 98%+ of the lines together in anywhere from 2-4 groups of lines.
/// 
/// This is necessary because the line segments via Marching Squares (at least the way we did it) does not flow logically into a 
/// polygonal shape, but are put together separately with no easy way to connect them to each other, except for how we did it
/// later on.
/// 
/// This function does the following:
///
/// 1. Looks for a first value in the segment line points, which are placed in their original position in the grid, converted from 
///    Segment types. 
/// 
/// 2. Looks at all diagonal points (left, right, lower-right, etc. -- 8 directions) to find a connecting point
///    It looks at the end point first, then looks for a backward line segment by looking at the first point.
/// 3. If it finds a segment, then it connects it and puts it into a growing sequential list of line segments to form a partial polygon
/// 4. When it finds the first point again (which is marked) or does not find a connecting point, it draws all lines at once. 
///
/// This process of drawing the segments as a large, massive partial polygon structure is much faster than drawing each line segment individually.
/// 'Partial Polygon' is used, because some lines segments are shared with other line segments. As such, they are ambiguous.
/// 
/// In general, for each distinctive Meta Ball shape in the output, this technique draw about 2-4 polygon per shape at most, meaning it
/// can draw just 2-4 polygons with one call instead of drawing thousands (sometimes tens of thousands) of line-segments one after the other.
///
/// --> Note;  For the shapes, the GDI is used directly.  Sagebox Shape/Path functions are currently in-progress.
///
/// --> Also Note: This functions serves both the Raw (non-interpolated) line segments as well as interpolated segments.
///                Both sets always have the same common points; where one is defined, the other is defined.
///                The point are different and, therefore, stored separately into two different arrays
///
///                In the calculation, we just need to check the connecting-point of one set and we assured the other one
///                also has a connector, as well.
///
pub (crate) fn draw_marching_square_segments(&mut self)
{
    let gdi = &self.win.gdi;                        // Get pointer to GDI graphics

    let grid_width  = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    let v_grid  = self.v_grid.as_mut_slice();       // Line segment types (0-15) as calculated for Marching Squares       
    let p_grid1 = self.p_grid.as_mut_slice();       // Input line segments for interpolated Marching Square
    let p_grid2 = self.p_grid2.as_mut_slice();      // Input line segments for Raw (non-interpolated segments).  

    // point_storage(s) missing here

    let p_row = (grid_width+2)*2;                   // Amount to add for each row, as we check each of 8 directions (left, right, up, down, etc.)

    let mut points1_iter = self.point_storage1.iter_mut();  // Connecting lines constructed as a large and sequential partial polygon set of segments
    let mut points2_iter = self.point_storage2.iter_mut();  // So that we can draw it all at once.

    let mut grid_index  = 0;
    let mut point_count = 0;
    let mut main_index  = (2+ (grid_width+2)*2) as usize; 

    let pen_int     = gdi.new_pen_s(self.color_int_outline,self.outline_pen_size); 
    let pen_raw     = gdi.new_pen_s(self.color_raw_outline,self.outline_pen_size); 

    for ii in 0..grid_height
    {
        for jj in 0..grid_width
        {
            let mut grid_value = v_grid[grid_index];        // Get the line-segment type, but don't analyze, and just look for active line segments
            grid_index += 1;

            let mut p1_in = p_grid1[main_index + 1];        // Get current point (which is used if the grid value is from 1-14)

            // If the gridvalue is between 1 and 15, we want to use it.
            // If we've run into this value before (we've completed an entire round-trip, and we're duplicating efforts),
            // the value has been or'd with 0x10 and places it > 15, so it is skipped.
            //
            // When we find values, we just follow them around the entire array, wherever they lead us, 
            // connectings lines and marking them as seen.

            if grid_value > 0 && grid_value < 15
            {
                let mut i = ii;     // Save current values
                let mut j = jj;

                *points1_iter.next().unwrap() = p_grid1[main_index + 0];    // Add Start Point for both sets of lines
                *points1_iter.next().unwrap() = p_grid1[main_index + 1];

                *points2_iter.next().unwrap() = p_grid2[main_index + 0];    // Also increase our count point by 2 for this set.
                *points2_iter.next().unwrap() = p_grid2[main_index + 1];

                point_count += 2;

                // As we move around, we just add one more point as a connector to this upper point.
                
                let mut p_index = (i*grid_width+j) as usize;    // Set this point as our current point (changes as we move around)

                // Follow as many points as possible, re-checking each points validity between 1 and 14

                while grid_value > 0 && grid_value < 15
                {
                    let p_add_index = (i+1) *p_row + j*2 + 2;  // Current Point (Base Point) for each line set
                   

                    // --> For each direction:
                    //    
                    //      1. The neighbor point (right, left, up, down, etc.) is check for a connector to the end point
                    //      2. If that fails, we check the start point as a connector.
                    //      4. If we find a connector, we add this point.  
                    //         a. the X and Y value are adjusted as required, 'i' for y-axis movement, 'j' if we move on the x-axis
                    // 
                    //      Macro Usage: 
                    //
                    //          inline macros are very useful here to look at each point in one of the 8 directions
                    //          as well as to process the values when completed. 
                    //
                    //          The macros provide a nice way to inline code, where calling out would
                    //          take more time in this function, which is pretty intensive time-wise, so 
                    //          using the macro rather than calling allows a lot of inlining, processor caching, etc.
                    //          (or otherwise saving us from writing a *lot* of repetitive code)

                    // macro compare() -- Look for a point (in one of the 8 directions) that connects directly to the point we're evaluating. 
                    //                    It looks at the first point of the evaluated value as the second part of the line segment. 
                    //                    if found, it adds the segment. 
                    //
                    //                    If not found, it looks at the second point for the connection (since the segments may be reversed)
                    //                    the process() macros is then used to follow-up

                    macro_rules! compare{($_index:expr,$add_j:expr,$add_i:expr) =>
                    {
                        let index = (p_add_index + $_index) as usize;
                        if (p1_in.0 == p_grid1[index+0].0) && (p1_in.1 == p_grid1[index+0].1) { *points1_iter.next().unwrap() = p_grid1[index+1];  
                                                                                                *points2_iter.next().unwrap() = p_grid2[index+1]; 
                                                                                                process!($add_j,$add_i,p_grid1[index+1]);  }
                        
                        if (p1_in.0 == p_grid1[index+1].0) && (p1_in.1 == p_grid1[index+1].1) { *points1_iter.next().unwrap() = p_grid1[index+0];  
                                                                                                *points2_iter.next().unwrap() = p_grid2[index+0]; 
                                                                                                process!($add_j,$add_i,p_grid1[index+0]);  }
                    } }

                    // macro process() -- If a point connectsing point is found, the i and j values are adjusted to go to this next point
                    //                    to then evaluate its neighbors. 
                    //
                    //                     The current point is marked (|= 0x10) so we don't evaluate it again. 
                    //                     The indexes and array locations are updated to the new point. 
                    //
                
                    macro_rules! process{ ($add_j:expr,$add_i:expr,$new_p1:expr) => 
                    { 
                        j += $add_j; i += $add_i;
                        v_grid[p_index] |= 0x10; 
                        p_index          = (i*grid_width + j) as usize;
                        grid_value       = v_grid[p_index];
                        p1_in            = $new_p1;    // Assigned from assigned points because we don't know which order was assigned, and we 
                                                                            // always want the start of the line segment
                        point_count += 1; 
                        continue;      // Continue in the main loop, with this point are the new evaluated point 
                    } }


                    // Look at the eight directions around our current point looking for a connecting line segment. 

                    compare!( 2         ,  1,  0); // Right
                    compare!( p_row + 2 ,  1,  1); // Lower-Right
                    compare!( p_row     ,  0,  1); // Bottom
                    compare!( p_row - 2 , -1,  1); // Bottom-Left
                    compare!(-2         , -1,  0); // Left
                    compare!(-p_row - 2 , -1, -1); // Top-Left     
                    compare!(-p_row     ,  0, -1); // Top 
                    compare!(-p_row + 2 ,  1, -1); // Top-Right     

                    // This is reached when we did not find a connector. 

                    v_grid[p_index] |= 0x10;    // Mark this point as evaluated. 
                    break;                      // Break the look and go to the next consecutive value -- since
                                                // the current loop travels around the array, the next pulled value may be marked as used, 
                                                // in which case it is ignored.
                }

                // If we found a string of line segments we collected, draw the lines out en mass. 
                
                // If we have only 1 point, then we have a straggler from when the buffer overflowed and we didn't
                // find a next point. 
                
                if point_count > 1
                {
                    // Only draw what is needed

                    if self.show_int_outline { gdi.draw_lines_size_f(&pen_int,&self.point_storage1,point_count); }
                    if self.show_raw_outline { gdi.draw_lines_size_f(&pen_raw,&self.point_storage2,point_count); }
                    
                    points1_iter = self.point_storage1.iter_mut();  // Start the output iterations again at the top.
                    points2_iter = self.point_storage2.iter_mut();
                    
                    point_count = 0;
                }
            }
            main_index += 2;        // go to the next consecutive point
        }
        main_index += 4;            // get past edge overflow
    }
}

// draw_marching_square_segments_full() -- Draw the Marching Square Line Segments for the 1:1 optimal curve in as much order as possible to group them
//                                         together as long singular polygon segments. 
//
// This function is almost completely not needed, except for one thing: Performance
//
// It is almost an exact copy of draw_marching_square_segments(), with the following difference:
//
// 1. It only looks at one set of line segments and only draws one set of line segment, where draw_marching_square_segments_full() specifically
//    looks at two line segments (Raw and Interpolated) simultaneously to save a lot of time (which it does).
// 2. Since there is no grid size to deal with (it's defined to be 1 here), some values that used the Grid Size have been
//    converted to 1.0.
// 
// The original idea was to do a generic function (which would be this one) to serve all outline drawing needs.
// However, drawing the Raw and Interpolated outline did cost some time to do individually, with minimal time 
// to convert and draw simultaneously.  Therefore, draw_marching_square_segments_full() was written to process both at the same time,
// leaving this one for the 1:1 output
//
// Note --> See draw_marching_square_segments() for notes.  This function (draw_marching_square_segments_full) is basically a 
//          copy of draw_marching_square_segments() stipped to process one line segment only. 
//
pub (crate) fn draw_marching_square_segments_full(&mut self)
{
    let gdi = &self.win.gdi;

    let grid_width  = self.win_size.x;
    let grid_height = self.win_size.y;

    // We still make this distinction on grid size because we don't generate vGridFull when m_iGridSize is 1
    // to save time.
    
    let v_grid  = if self.grid_size == 1 { self.v_grid.as_mut_slice() } else { self.v_grid_full.as_mut_slice() };   
    let p_grid3 = self.p_grid3.as_mut_slice();       

    let p_row = (grid_width+2)*2; 

    let mut points1_iter = self.point_storage1.iter_mut();

    let mut grid_index  = 0;
    let mut point_count = 0;
    let mut main_index  = (2+ (grid_width+2)*2) as usize; 

    let pen_full     = gdi.new_pen_s(self.color_full_curve,self.outline_pen_size); 
    
    for ii in 0..grid_height
    {
        for jj in 0..grid_width
        {
            let mut grid_value = v_grid[grid_index] & 0x0F;
            grid_index += 1;

            let mut p1_in = p_grid3[main_index + 1];

            if grid_value > 0 && grid_value < 15
            {
                let mut i = ii;
                let mut j = jj;

                *points1_iter.next().unwrap() = p_grid3[main_index + 0];
                *points1_iter.next().unwrap() = p_grid3[main_index + 1];

                point_count += 2;

                let mut p_index = (i*grid_width+j) as usize;

                while grid_value > 0 && grid_value < 15
                {
                    let p_add_index = (i+1) *p_row + j*2 + 2;  // Current Point (Base Point) for each line set
                   
                    macro_rules! compare{($_index:expr,$add_j:expr,$add_i:expr) =>
                    {
                        let index = (p_add_index + $_index) as usize;
                        if (p1_in.0 == p_grid3[index+0].0) && (p1_in.1 == p_grid3[index+0].1) { *points1_iter.next().unwrap() = p_grid3[index+1];  
                                                                                                process!($add_j,$add_i,p_grid3[index+1]);  }
                        
                        if (p1_in.0 == p_grid3[index+1].0) && (p1_in.1 == p_grid3[index+1].1) { *points1_iter.next().unwrap() = p_grid3[index+0];  
                                                                                                process!($add_j,$add_i,p_grid3[index+0]);  }
                    } }

                    // Macro Usage: see draw_marching_square_segments() for comments on the macros below. 

                    macro_rules! process{ ($add_j:expr,$add_i:expr,$new_p1:expr) => 
                    { 
                        j += $add_j; i += $add_i;
                        v_grid[p_index]  = 0; 
                        p_index          = (i*grid_width + j) as usize;
                        grid_value       = v_grid[p_index] & 0x0F;
                        p1_in            = $new_p1;    // Assigned from assigned points because we don't know which order was assigned, and we 
                                                                            // always want the start of the line segment
                        point_count += 1; 
                        continue; 
                    } }

                    // Look at the eight directions around our current point looking for a connecting line segment. 

                    compare!( 2         ,  1,  0); // Right
                    compare!( p_row + 2 ,  1,  1); // Lower-Right
                    compare!( p_row     ,  0,  1); // Bottom
                    compare!( p_row - 2 , -1,  1); // Bottom-Left
                    compare!(-2         , -1,  0); // Left
                    compare!(-p_row - 2 , -1, -1); // Top-Left     
                    compare!(-p_row     ,  0, -1); // Top 
                    compare!(-p_row + 2 ,  1, -1); // Top-Right     
                    
                    // This is reached when we did not find a connector. 

                    v_grid[p_index] |= 0;    // Mark this point as evaluated. 
                    break;                      // Break the look and go to the next consecutive value -- since
                                                // the current loop travels around the array, the next pulled value may be marked as used, 
                                                // in which case it is ignored. 
                }
                
                // If we found a string of line segments we collected, draw the lines out en mass. 
                
                // If we have only 1 point, then we have a straggler from when the buffer overflowed and we didn't
                // find a next point. 
                if point_count > 1
                {
      
                    gdi.draw_lines_size_f(&pen_full,&self.point_storage1,point_count);
                    points1_iter = self.point_storage1.iter_mut();
                    
                    point_count = 0;
                }
            }
            main_index += 2;        // go to the next consecutive point
        }
        main_index += 4;            // get past edge overflow
    }

}

/// ------------------------------
/// draw_marching_squares_faster()
/// ------------------------------
///
/// This is a much faster version of draw_marching_squares_naive() (which is not included in this source code, for brevity), by just doing one thing:
/// 
///     - The filled portions (where it filles in the shape) are collected on each line to form one giant rectangle, rather than
///       drawing each rectangle separately. 
/// 
///       In larger grid sizes, this really doesn't matter.  However, with Grid Sizes of 5x5 or less, this can be thousands of rectangles
///       converted to one rectangle, which makes a large processing difference. 
/// 
///       The main issue is startup time for the graphics functions when using non-GPU-based functions.
/// 
///       In this sense we're doing the same thing: filling an order first, and then transferring it all to the graphics function
///       to perform en-mass, at least as far as rectangle are concerned.
/// 
/// We could use this technique for all of the objects we put out (and do so a little in draw_marching_squares_performance()), where we
/// would be treating these details a lot more the we would for GPU usage.
/// 
/// That's one of the points in this demo, is that we can use some pretty serious graphics without using the GPU, but being a little crafty
/// to speed things up doesn't hurt, either.
/// 
/// As mentioned in draw_marching_squares_performance() notes, this isn't strictly necessary, but is a
/// nice-to-have, and does make a difference with smaller grid sizes (5x5 or less -- where anything larger really doesn't matter).
/// 
/// It's a lot easier to use non-GPU graphics for most things until the GPU makes sense. In this program, though, the GPU really isn't necessary,
/// which makes things a lot easier. 
///
pub (crate) fn _draw_marching_squares_faster(&self)             // Not currently used.  Left here for comparison between higher-performance version
{
    let grid_width  = self.win_size.x/self.grid_size;
    let grid_height = self.win_size.y/self.grid_size;

    let gdi = &self.win.gdi;

    let mut v_index     = 0;                    // Index into Meta Ball result array, telling us the strength of the calculation
                                                //    We could use an iterator, but we'd need 4 of them, so indexing works well here.
    let mut v_grid_iter = self.v_grid.iter();   // Iterator for grid that tells us what Marching Square line type we have
                                                //    for each grid cell.

    let grid_size_f = self.grid_size as f32;
    let grid_div_2  = grid_size_f/2.0;

    let brush_int   = gdi.new_solid_brush_s(self.color_int_interior);
    let brush_raw   = gdi.new_solid_brush_s(self.color_raw_interior);
    let pen_int     = gdi.new_pen_s(self.color_int_outline,self.outline_pen_size); 
    let pen_raw     = gdi.new_pen_s(self.color_raw_outline,self.outline_pen_size); 
    let path_int    = gdi.new_path();
    let path_raw    = gdi.new_path();
  
    let v = self.v.as_slice();                // Get the Meta Ball Array we calculated specifically for a pixel version (i.e. centered, not marching-square corners)

    for i in 0..grid_height
    {
        let mut rect_count = 0;
        let y = (i*self.grid_size) as f32;
        for j in 0 ..grid_width
        {
            let x = (j*self.grid_size) as f32;
            let b = *v_grid_iter.next().unwrap(); // We know it won't go over

            // Collect consecutive rectangles where the cells are completely filled.
            // In this case, we have no line segment to draw, so we don't have to worry about
            // weird precedence issues when we end the line with a pending rectangle to draw

            if b == 15 { rect_count += 1; }
            else
            {
                // if we encountered a non-filled square, and have a pending rectangle set, draw them
                // (in the same order as below: Raw (non-interpolated), then Interpolated

                if rect_count > 0
                {
                    if self.show_int_interior { path_int.add_rectangle((x as i32-rect_count*self.grid_size,y as i32),
                                                                       (self.grid_size*rect_count,self.grid_size)); }
                    if self.show_raw_interior { path_raw.add_rectangle((x as i32-rect_count*self.grid_size,y as i32),
                                                                       (self.grid_size*rect_count,self.grid_size)); }
                    rect_count = 0;
                }
            }

            // Get various point values..

            // These values are the midpoints of each grid cell, that are used in the non-interpolated version
            // (i.e. Raw Outline vs. Interpolated) to draw each line segments
            //
            // the 'c' stands for center, e.g. uc = upper-cent, lc = left-center, bc = bottom-center, rc = right-center


            let uc = (x+grid_div_2,y)                   ;
            let lc = (x,y+grid_div_2)                   ;
            let bc = (x + grid_div_2,y + grid_size_f)   ;
            let rc = (x+grid_size_f,y + grid_div_2)     ;
            
            // These values are used as anchor points when drawwing the interior (when the 'fill interior' option is active), to
            // point to the corners of the grid cell to fill in the shape created by the interpolated line segment. 
            //
            // notation is ul = upper-left, ur = upper-right, bl = bottom-left, br = bottom-right (i.e. corners vs. center values above) 

            let ul = (x,y)                              ;
            let ur = (x + grid_size_f,y)                ;
            let bl = (x,y + grid_size_f)                ;
            let br = (x + grid_size_f,y + grid_size_f)  ;

            // These values are the strength values of the calculated grid cell corners, 
            // Upper-Left, Upper-Right, Bottom-Left, Bottom,Right, etc.
            //
            // These are used to calculate where to move the li

            let f_ul = v[v_index + 0];
            let f_ur = v[v_index + 1];
            let f_bl = v[v_index + (0 + grid_width + 1) as usize];
            let f_br = v[v_index + (1 + grid_width+ 1) as usize];

            // These are the modified line segments, and are explained here:
            //
            //  --> https://jamie-wong.com/2014/08/19/metaballs-and-marching-squares/
            //            

            let p_ul = Point::<f32>::fromf32(ul);
            let p_ur = Point::<f32>::fromf32(ur);
            let p_bl = Point::<f32>::fromf32(bl);
            let p_br = Point::<f32>::fromf32(br);
            let prc = (p_ur + (p_br-p_ur)*((1.0 - f_ur)/(f_br-f_ur))).f32();
            let pbc = (p_bl + (p_br-p_bl)*((1.0 - f_bl)/(f_br-f_bl))).f32();
            let puc = (p_ul + (p_ur-p_ul)*((1.0 - f_ul)/(f_ur-f_ul))).f32();
            let plc = (p_ul + (p_bl-p_ul)*((1.0 - f_ul)/(f_bl-f_ul))).f32();

            // *** Process non-filled rectangles, where we need to draw a line segment and fill in a polygon shape for the specific
            //     grid cell, following the order of interior first, then draw the line segment over it.
            // 

            // Draw the Marching Square Raw version (non-interpolated) interior.
            // --> In this case, we form polygons with the same line segments as above, using the 
            //     corners to form the polygon to fill.
            // 
            //     In a couple cases, we fill two polygons because we drew two line segments, so we use the
            //     opposing corners to fill the two polygons based on the two line segments
            //
            // --> Note: We don't really get these in a dependable order, so we don't know
            //           the shape of the polygon without doing some time-consuming calculations.
            //           Realistically, doing so would probably be faster because it would be faster graphically,
            //           where the time-consuming part is more developing it and testing it, which is out-of-scope here
            //           because it isn't really that much of a benefit for our purposes.
            // 
            //           Fortunately, that's not necessary here, because we just fill in 
            //           each cell one at a time, which, when done, forms the entire shape.
            // 
            //           In cases where there are more than one distinct shape on the screen, we can't even tell that here, and
            //           have really on idea where we are in the shape structure, only where we are in the grid.
            // 
            // Depending on the structure, some polygons have more sides than others.

            if self.show_raw_interior
            {
                match b
                {
                    1 =>  { let xs = [bl,lc,bc]               ; path_raw.add_polygon_f32(&xs); }
                    2 =>  { let xs = [br,rc,bc]               ; path_raw.add_polygon_f32(&xs); }
                    3 =>  { let xs = [rc,lc,bl,br]            ; path_raw.add_polygon_f32(&xs); }
                    4 =>  { let xs = [ur,uc,rc]               ; path_raw.add_polygon_f32(&xs); }
                    5 =>  { let xs = [lc, uc, ur, rc, bc, bl] ; path_raw.add_polygon_f32(&xs); }
                    6 =>  { let xs = [uc,ur,br,bc]            ; path_raw.add_polygon_f32(&xs); }
                    7 =>  { let xs = [lc,uc,ur,br,bl]         ; path_raw.add_polygon_f32(&xs); }
                    8 =>  { let xs = [ul,lc,uc]               ; path_raw.add_polygon_f32(&xs); }
                    9 =>  { let xs = [ul,uc,bc,bl]            ; path_raw.add_polygon_f32(&xs); }
                    10 => { let xs = [lc,ul, uc, rc, br, bc]  ; path_raw.add_polygon_f32(&xs); }
                    11 => { let xs = [ul, uc,rc, br, bl]      ; path_raw.add_polygon_f32(&xs); }
                    12 => { let xs = [lc,ul,ur,rc]            ; path_raw.add_polygon_f32(&xs); }
                    13 => { let xs = [bc,bl, ul, ur, rc]      ; path_raw.add_polygon_f32(&xs); }
                    14 => { let xs = [lc,ul, ur, br, bc]      ; path_raw.add_polygon_f32(&xs); }
                    _ => {}
                }
            }

            // Fill the interior of the Interpolated Marching Square polygon formed by the line segments.
            //
            // This is the same process as the above code that fills in the Raw (non-interpolated version), using
            // the interpolated points vs. the raw points (e.g. plc instead of lc, pbc instead of bc).
            //
            // The same corner points are used, since they don't change.
            //          

            if self.show_int_interior
            {
                match b
                {
                    1 =>  { let xs = [bl,plc,pbc]                 ; path_int.add_polygon_f32(&xs); }
                    2 =>  { let xs = [br,prc,pbc]                 ; path_int.add_polygon_f32(&xs); }
                    3 =>  { let xs = [prc,plc,bl,br]              ; path_int.add_polygon_f32(&xs); }
                    4 =>  { let xs = [ur,puc,prc]                 ; path_int.add_polygon_f32(&xs); }
                    5 =>  { let xs = [plc, puc, ur, prc, pbc, bl] ; path_int.add_polygon_f32(&xs); }
                    6 =>  { let xs = [puc,ur,br,pbc]              ; path_int.add_polygon_f32(&xs); }
                    7 =>  { let xs = [plc,puc,ur,br,bl]           ; path_int.add_polygon_f32(&xs); }
                    8 =>  { let xs = [ul,plc,puc]                 ; path_int.add_polygon_f32(&xs); }
                    9 =>  { let xs = [ul,puc,pbc,bl]              ; path_int.add_polygon_f32(&xs); }
                    10 => { let xs = [plc,ul, puc, prc, br, pbc]  ; path_int.add_polygon_f32(&xs); }
                    11 => { let xs = [ul, puc,prc, br, bl]        ; path_int.add_polygon_f32(&xs); }
                    12 => { let xs = [plc,ul,ur,prc]              ; path_int.add_polygon_f32(&xs); }
                    13 => { let xs = [pbc,bl, ul, ur, prc]        ; path_int.add_polygon_f32(&xs); }
                    14 => { let xs = [plc,ul, ur, br, pbc]        ; path_int.add_polygon_f32(&xs); }
                    _ => {}
                }
            }

           // Show Marching Square Raw Line Segments (i.e. non-interpolated, based only on the boolean of whether a corner is > 1.0 only)
           // (this is the one that shows in red outline)
           // 
           // The interior was drawn first, as this overlaps it by a little bit.
           //
           // For each value (0-15) draw the appropriate line-segment (see url above for explanation)
           //
           // As with the interior, we don't know where we are in the large polygon (or set of polygons) we're forming, or even
           // if there are multiple ones.  As we fill in each line segment, the polygon(s) that is/are the main Meta Ball structure(s)
           // will be completed as a whole.

            if self.show_raw_outline
            {
                match b
                {
                    1  => { gdi.draw_line_f(&pen_raw,lc,bc); }
                    2  => { gdi.draw_line_f(&pen_raw,rc,bc); } 
                    3  => { gdi.draw_line_f(&pen_raw,rc,lc); } 
                    4  => { gdi.draw_line_f(&pen_raw,uc,rc); } 
                    5  => { gdi.draw_line_f(&pen_raw,lc,uc);  
                            gdi.draw_line_f(&pen_raw,bc,rc); }  
                    6  => { gdi.draw_line_f(&pen_raw,uc,bc); } 
                    7  => { gdi.draw_line_f(&pen_raw,lc,uc); } 
                    8  => { gdi.draw_line_f(&pen_raw,lc,uc); } 
                    9  => { gdi.draw_line_f(&pen_raw,uc,bc); } 
                    10 => { gdi.draw_line_f(&pen_raw,lc,bc);      
                            gdi.draw_line_f(&pen_raw,uc,rc); }   
                    11 => { gdi.draw_line_f(&pen_raw,uc,rc); }  
                    12 => { gdi.draw_line_f(&pen_raw,lc,rc); }  
                    13 => { gdi.draw_line_f(&pen_raw,bc,rc); }  
                    14 => { gdi.draw_line_f(&pen_raw,lc,bc); }  
                    _ => {}
                }
            }

            // Interpolated Outline

            if self.show_int_outline
            {
                match b
                {
                    1  => { gdi.draw_line_f(&pen_int,plc,pbc); }
                    2  => { gdi.draw_line_f(&pen_int,prc,pbc); } 
                    3  => { gdi.draw_line_f(&pen_int,prc,plc); } 
                    4  => { gdi.draw_line_f(&pen_int,puc,prc); } 
                    5  => { gdi.draw_line_f(&pen_int,plc,puc);  
                            gdi.draw_line_f(&pen_int,pbc,prc); }  
                    6  => { gdi.draw_line_f(&pen_int,puc,pbc); } 
                    7  => { gdi.draw_line_f(&pen_int,plc,puc); } 
                    8  => { gdi.draw_line_f(&pen_int,plc,puc); } 
                    9  => { gdi.draw_line_f(&pen_int,puc,pbc); } 
                    10 => { gdi.draw_line_f(&pen_int,plc,pbc);      
                            gdi.draw_line_f(&pen_int,puc,prc); }   
                    11 => { gdi.draw_line_f(&pen_int,puc,prc); }  
                    12 => { gdi.draw_line_f(&pen_int,plc,prc); }  
                    13 => { gdi.draw_line_f(&pen_int,pbc,prc); }  
                    14 => { gdi.draw_line_f(&pen_int,plc,pbc); }  
                    _ => {}
                }
            }
                v_index += 1;   // Move to next location in the grid 
        }

        // For the filled interior, fill in any rectangles that ended on the edge (and are, therefore, pending)
        // back to their starting point
        
        if rect_count > 0
        {
            if self.show_int_interior { path_int.add_rectangle((grid_width*self.grid_size-rect_count*self.grid_size, y as i32),
                                                               (self.grid_size*rect_count,self.grid_size)); }
            if self.show_raw_interior { path_raw.add_rectangle((grid_width*self.grid_size-rect_count*self.grid_size,y as i32),
                                                               (self.grid_size*rect_count,self.grid_size)); }
        }
        v_index += 1;   // There is an overlap of 1 value for each line.
    }   
    
    // Draw interior paths/shapes we created, depending on which area we want to show. 

    if self.show_raw_interior { gdi.fill_path(&path_raw, &brush_raw); }
    if self.show_int_interior { gdi.fill_path(&path_int, &brush_int); }
 
}

}