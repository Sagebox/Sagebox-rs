use sagebox::Sagebox;

use super::PendMain;

impl PendMain
{
/// handle_mouse_drag() - Handle when the mouse is down and moving and we're dragging a pendulum.
///
fn handle_mouse_drag(&mut self)
{
    let delta = self.win.get_mouse_pos().to_pf64() - self.pend.rod_vertex[self.dragging-1];
    
    self.pend.angle[self.dragging-1] = f64::atan2(delta.x,delta.y);
    if !self.keep_rod_length { self.pend.length[self.dragging-1] = f64::sqrt(delta.x*delta.x + delta.y*delta.y)/self.pend.zoom; }
    self.pend.reset();
}

/// handle_mouse_click() - If the mouse was clicked, Pause or Unpause the display.  Also set a dragging indicator if the user pressed on 
/// one of the pendulum bobs. 
///
fn handle_mouse_click(&mut self)
{
    self.display_instructions = false;
    let force_hold = !self.pend.pause;
  
    let mouse_pos = self.win.get_mouse_pos().to_pf64();


    let radius1 = self.pend.top_radius*self.pend.zoom*self.pend.circle_mult;
    let radius2 = self.pend.bot_radius*self.pend.zoom*self.pend.circle_mult;

    self.dragging = 0; 

    if mouse_pos.within_rect(self.pend.rod_vertex[1]-radius1,self.pend.rod_vertex[1] + radius1) { self.dragging = 1; }
    if mouse_pos.within_rect(self.pend.rod_vertex[2]-radius2,self.pend.rod_vertex[2] + radius2) { self.dragging = 2; }

    if self.dragging == 0 { self.pend.pause = force_hold; }
    else { self.pend.pause = true; }

}

/// handle_events() - Handle events occuring, such as mouse clicks, movements, etc.
///
pub (crate) fn handle_events(&mut self)
{
    if self.win.mouse_clicked() { self.handle_mouse_click() }
    if self.win.mouse_drag_event(false) && self.dragging != 0 { self.handle_mouse_drag(); }

    if self.win.mouse_wheel_moved()
    {
        let dist = self.win.get_mouse_wheel_move(); 
        self.pend.zoom *= if dist < 0 { 0.95 } else { 1.0/0.95 };
    }
    if self.win.mouse_r_button_down() { self.pend.rod_vertex[0].y = self.win.get_mouse_pos().y as f64; }

    // --------------------
    // Look at the controls
    // --------------------

    if self.input_weight1.return_pressed()
    {
        let value = self.input_weight1.get_f64();
        if value > 0.0 { self.pend.mass[0] = value; }
    }

    if self.input_weight2.return_pressed() { self.pend.mass[1] = self.input_weight2.get_f64() }

    if self.quit_button.pressed() { self.quit_program = true; }
    if self.button_show_timing.pressed()
    {
        self.show_timing = self.button_show_timing.checked();
        Sagebox::show_debug_window_s(self.show_timing);
    }
    if self.slider_pend_size.moved() { self.pend.circle_mult = self.slider_pend_size.get_pos() as f64/100.0*5.0 + 0.25; }
    if self.slider_dampen.moved() 
    { 
        let pos = self.slider_dampen.get_pos();
        self.pend.damp1 = 1.0-pos as f64/10000.0; 
        self.pend.damp2 = self.pend.damp1; 
        if pos == 0 { self.pend.reset_overflow(); }
    }
    if self.slider_zoom.moved() { self.pend.zoom = self.slider_zoom.get_pos() as f64/100.0; }

}
}