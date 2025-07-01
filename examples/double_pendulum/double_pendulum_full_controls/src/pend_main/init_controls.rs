use super::PendMain;
use sagebox::*; 

impl PendMain
{

//// init_controls() - Creates and initialize the controls in the Dev Window to the left. 
///
/// - Creates a couple input boxes, sliders, checkboxes, and one set of radio buttons. 
/// - See notes in the code for more information. 
///
pub (crate) fn init_controls(&mut self)
{
    let ref dev_win = self.dev_win; 

    dev_win.new_text("{17}{cyan}Pendulum Controls");     // Simple text with a font of size 17 (Arial) and text color cyan

    // The defaults set are in inline with the defaults for the pendulum.  Some of them hard-coded here for easier coding and 
    // readability when they would otherwse (i.e. not a demo) certain function would call get/set functions in the Pendulum class to separate 
    // (i.e. decouple) context and data.  

    self.input_weight1     = dev_win.new_input_box_s("Weight 1",kw::default(self.pend.mass[0] as i32)); 
    self.input_weight2     = dev_win.new_input_box_s("Weight 2",kw::default(self.pend.mass[1] as i32)); 

    self.slider_dampen     = dev_win.new_slider_s("Dampen Multiplier",kw::default(15));  // consistent with .9985 default value
    self.slider_zoom       = dev_win.new_slider_s("Zoom",kw::range((25,150)) + kw::default(100)); 
    self.slider_pend_size  = dev_win.new_slider_s("Pendulum Size",kw::default(15)); 

    // Add 3 radio buttons.  horz() puts them horizontally, otherwise they are put vertically. Kw::columns() can specifiy specific # of columns

    self.radio_thick_lines      = dev_win.new_radio_buttons_s("Normal\nThick\nThicker",kw::horz() + kw::title("Rod Thickness")); 
    self.button_display         = dev_win.new_checkbox_s("Display Values", kw::default(self.display_values));        // "-" puts it on next line but closer to last checkbox
    self.button_static_length   = dev_win.new_checkbox_s("-Maintain Rod Length",kw::default(self.keep_rod_length));    
    self.button_show_trail      = dev_win.new_checkbox_s("+Show Trail",kw::default(self.pend.show_trail));           // "+" to add checkbox on same line as last
    self.button_single_pendulum = dev_win.new_checkbox_s("-Single Pendulum",kw::default(self.pend.single_pend));   
    self.button_show_timing     = dev_win.new_checkbox_s("-Show Timing",kw::default(self.show_timing)); 
    self.button_start           = dev_win.new_button("   Stop   ");                                                  // Use spaces to pad the display so it is wider. 
    self.quit_button            = dev_win.new_button("+   Quit   ");                                                 // "+" to add button on same line as last

}
}