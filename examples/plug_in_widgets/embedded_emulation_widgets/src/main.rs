// -----------------------------------
// Two Widgets / Mini-Emulator Example
// -----------------------------------
//
// This program demonstrates how the Dial and LCD widgets can work together to form the foundation of a  
// simple hardware emulator. 
//
// While this example runs locally, a slightly more abstracted version (unlike the  
// all-in-one, somewhat monolithic version here) can be structured more for real emulation. Simply overload a few  
// functions to connect it to actual hardware -- either to test individual components with the rest emulated,  
// or to peer into the entire system working together in real-time.
// 
// The Emulation Premise
// 
// The setup mimics a local temperature dial (e.g., for a wall or pool heater) and a remote embedded device  
// that receives and displays the data on an LCD -- something Sagebox has been used for in industry to  
// emulate and develop embedded applications.
// 
// What may seem like a small demo is actually how many real emulators begin -- simple at first,  
// as either a proof of concept or a way to test one component, then gradually expanding into fully-featured systems.
//
// -----------------
// TLDR; Main points
// -----------------
//
//  - Demonstrates how the Dial and LCD widgets can be used together to form a basic hardware emulator foundation.
//  - Runs locally -- an abstracted version can connect to real hardware by overloading a few functions.
//  - Allows testing individual components with the rest emulated or running the entire system.
//  - Emulates a local temperature dial (like for a wall or pool heater) and a remote device displaying data on an LCD.

use sagebox::*;

use dial_widget::DialWidget;
use lcd_widget::LcdWidget;


const PGR_FILENAME : &str = "graphic_data.pgr"; // File containing background graphics, button and widget placement details
                                                // Also contains another .PGR which is the information for the About Window

pub struct EmbeddedWidgetsExample
{
    // Window controls

    win                 : Window,       // The main window
    debug_win           : Window,       // the lower debug window, a child window of the main window
    button_close        : Button,       // 'Close' button
    button_about        : Button,       // 'about' button

    valid               : bool,             // true if the .PGR file wasn't found or verified

    // Texture image and control/window locations read from the PGR file (see LoadTextData() comments)

    lcd_pos             : (i32,i32),        // Location of LCD Widget in window
    dial_pos            : (i32,i32),        // Location of Dial Widget in window
    debug_win_pos       : (i32,i32),        // Location of debug window
    debug_win_size      : (i32,i32),        // Size of debug window
    button_pos          : (i32,i32),        // Start location of lower buttons
    background_image    : Bitmap,           // The main texture/bitmap image filling the entire windowow
    about_file          : Vec<u8>,
    debug_win_bg_color  : String,            // background color of the debug window (in string form, e.g. "Black" vs. an RgbColor)
    dial_widget         : DialWidget,
    lcd_widget          : LcdWidget,
}

impl Default for EmbeddedWidgetsExample {
    fn default() -> EmbeddedWidgetsExample {
        EmbeddedWidgetsExample {
        win                 : Window::default(),
        debug_win           : Window::default(),
        button_close        : Button::default(),
        button_about        : Button::default(),
        valid               : true,
        lcd_pos             : (0,0),
        dial_pos            : (0,0),
        debug_win_pos       : (0,0),
        debug_win_size      : (0,0),
        button_pos          : (0,0),
        background_image    : Bitmap::default(),
        about_file          : vec![],
        debug_win_bg_color  : String::new(),
        dial_widget         : DialWidget::default(),
        lcd_widget          : LcdWidget::default(),
        }
    }
}

impl EmbeddedWidgetsExample
{

pub fn new() -> EmbeddedWidgetsExample
{
	let mut this = EmbeddedWidgetsExample { ..Default::default() };
	this.init();
	this
}

/// [is_valid()] - Returns true if the `EmbeddedWidgetsExample` initialization was successful
/// 
/// - The only reason the initialization would fail is if the `graphic_data.pgr` file was not found. 
///
pub fn is_valid(&self) -> bool { self.valid }


/// [init_pgr_data()] -- Open up the PGR file, a profile and data storage format.
///
/// It has the main bitmap but also specified where control objects and the text window need to be placed. 
///
/// .PGR files contain files and declarations to control the data. 
///
/// This is put into the PGR so that it can later be generic and change -- as the 
/// location of various element change, those can be changed in the PGR file so
///
///  - Code here doesn't need to change because a graphic has changed
///  - Different images (i.e. PGR files) can be used for the same process
///           (for example, a different scenario with some or all of the same tools -- the PGR File
///            can contain any value, including dictating which elements to use and which may not be available.
///
///
fn init_pgr_data(&mut self) -> bool
{
    let pgr = Sagebox::read_pgr_file(PGR_FILENAME);
    if !pgr.is_valid() { return false; }        // Invalidate if we did not find the .PGR file, or it was otherwise not valid.

    // Since we've tested the PGR and we know it's loaded, there is no 
    // error-checking on the following values. 
    //
    // Even if something was wrong, all errors would just fall-through with
    // some display problems. 
    //
    // We can optionally check for validity of these values, but it is not done here
    // as a demo, where it might be for an in-house/company or release version.

    self.background_image   = pgr.read_image_file("texture_image");
    self.lcd_pos            = pgr.read_pair_i32_or_0("lcd_loc");
    self.dial_pos           = pgr.read_pair_i32_or_0("dial_loc");
    self.debug_win_pos      = pgr.read_pair_i32_or_0("debug_win_loc");
    self.debug_win_size     = pgr.read_pair_i32_or_0("debug_win_size");
    self.button_pos         = pgr.read_pair_i32_or_0("button_loc");
    self.debug_win_bg_color = pgr.read_str_or_empty("debug_win_bg_color");
    self.about_file         = pgr.read_raw_binary_file("about_file");       // Loads an embedded .PGR that the 'about' function uses to display the about window

    true

}

/// [init_dial()] - Create the Dial Widget.  
///
/// - When the dial knob is rotated, the value is displayed, and the raw value
/// from the dial is sent to the remote device (represented by the LCD). 
///
/// - The dial can be a temperature or other gauge, such as a pool temperature or local heater control.
///
fn init_dial(&mut self)
{
    self.dial_widget = dial_widget::new(&self.win,self.dial_pos); 
    self.dial_widget.set_range((0,150));
    self.dial_widget.set_value(75);
}

/// [init_lcd()] - Create the LCD Widget, which will display the raw data from the Dial Widget. 
///
/// - This represents emulating a remote embedded device, disconnected from the user and
/// main dial control.
///
fn init_lcd(&mut self)
{
    self.lcd_widget = lcd_widget::new(&self.win,self.lcd_pos,0);
}

/// [init_hardware()] - initialize all hardware in the emulation
///
/// - In this specific example, we only have two things we're initlializing: the `dial` and `lcd` widgets
/// - As hardware is added or needs change, this nicely abstracts from the main program without
///   the higher-level part of the program needing to know what hardware is being initialized or how it's done.
///
fn init_hardware(&mut self)
{
    self.init_dial();
    self.init_lcd();
}


/// [update_lcd()] - Updates the LCD to the current Dial/Temperature reading
///
/// - This calls out to the Dial Widget to get the value from the hardware
/// - The value is then displayed on the LCD panel
///
fn update_lcd(&self)
{
    self.lcd_widget.set_value(self.dial_widget.get_value() as i32);
}

/// init_debug_window() - Initialize the lower, large black window, aka 'the debug window'
///
/// - The debug window displays the output of the Dial Widget
/// - This creates a child window in the main window in position and size set
///   via the `graphic_data.pgr`, abstracting it from the code.
///
fn init_debug_window(&mut self)
{
    // Create the debug output window.  Here, it only displays the dial value, representing
    // the output we might get from any device.  In this case, it represents the low-level
    // value we might get from a hardware device such as the dial control.

    self.win.set_drag_window(true);
    self.debug_win = self.win.child_window_s(self.debug_win_pos,self.debug_win_size, kw::pos(self.debug_win_pos) + kw::size(self.debug_win_size) + 
                                            kw::bg_color(&self.debug_win_bg_color) + kw::font_str("courier new,12")); 

    // Set up an "Immediate" update type.  This will update the debug window after each and every write, so we don't
    // have to worry about its state.  This is a slower way to do it and not recommended when you want ultra-speed, but is
    // fine for this purpose.  Otherwise, we'd have to Update() ourselves, as letting the auto-udpate handle it does not
    // guarantee the latest output until an Update() is called.

    self.debug_win.set_auto_update(AutoUpdateType::Immediate);
    self.debug_win.writeln("{g}Emulator Debug Window Ready (OO Version) - {y}Move the dial to see results.\n");

}

/// [init_window()] - Initializes the main window, setting the background bitmap, creating controls, etc.
///
fn init_window(&mut self)
{


    // Open a window sized to the background image & texture.
    // 
    // no_border() -- Specifies no title bar (i.e. client area only) for popup windows. 

    self.win = Sagebox::new_window_s(kw::size(self.background_image.size()) + kw::no_border());
    self.win.display_bitmap((0,0), &self.background_image);         // Display the background image (we only need to do it once)

    // Create a couple buttons.  Since I didn't put a menu, I put an about button here. 

    self.button_close = self.win.new_button_s(self.button_pos,"Exit",kw::width(120));
    self.button_about = self.win.new_button_s(self.button_pos,"About",kw::width(120) + kw::pad_x(125));

    // Add some hover messages (i.e. tooltips) so when the buttons or hovered-over with the mouse, a message displays 

    self.button_close.set_tooltip("Press to Close Window and Exit program.");
    self.button_about.set_tooltip("Show About Box Window explaining this program.");

   // Allow the window be dragged around by any open area in the window (since we don't have the title bar)
   // Otherwise the user wouldn't be able to move the window at all since we specified "NoBorder()" on
   // window creation.

    self.win.set_drag_window(true);

}

/// [init()] -- Init the main program / `EmbeddedWidgetsExample` struct
///
fn init(&mut self)
{
    self.valid = self.init_pgr_data();  // Load the graphics files and settings from the
                                        // external .PGR File

    if self.valid
    {
        self.init_window();     // Create the main window and buttons

   
        self.init_hardware();       // Initialize the Dial and LCD Widgets
        self.init_debug_window();   // Create the bottom, large black window
        self.update_lcd();          // Display the current value of the temperature dial to the LCD
   }
   else { println!("Error: Graphic data file '{}' was not found or is corrupted.",PGR_FILENAME) };
}

/// [display_about_box()] - Displays the main 'About Window', which then waits for the user to press the 'Ok' button
///
/// - This uses a Sagebox function called `draw_simple_doc_file()` which translates a rudimentary .HTML file
///   and displays it in a pop-up window to the screen.
///
/// - The input is .PGR file that contains the HTML file and related images, including the main backdrop images, as well as any commands to
///   the `draw_simple_doc_file()` function.
///
/// - The `draw_simple_doc_file_mem()` function will return false if the file is not found or is corrupted.
///   - We don't check it here, as we just let it fall through passively if it fails
/// - When the window is displayed, it creates an 'ok' button to press, putting the window into a modal 
///   state (i.e. the user can't operate any other windows until this window closes) until the user pressed the 'ok' button or closes the window.
///
fn display_about_box(&self)
{
    // Send the 'about file' .PGR file we loaded into memory from the main `graphic_data.pgr` file loaded when we initialized the program. 

    self.win.draw_simple_doc_file_mem(&self.about_file); // Function returns false if memory was empty or invalid (but we don't care here)
}


// check_dial_event() -- this effectively combines three functions 
//
//  1. Check the hardware event (the dial). 
//  2. Print out changed data to the debug window 
//  3. Set the LCD Value (which could be hardware or emulated)
//
pub fn check_dial_event(&self)
{
    if self.dial_widget.value_changed()
    {
        self.debug_win.writeln(format!("{{g}}Event.{{}}  Value = {{y}}{}{{}} -- Raw Value = {{c}}{}{{}} -- Sent Value to Device (LCD)",
            self.dial_widget.get_value_i32(),self.dial_widget.get_raw_value())); 
        self.update_lcd();
    }

}

// [run()] - Run the main part of the program. 
// 
pub fn run(&self)
{
    if !self.is_valid() { return; }     // Return if we didn't validate the .PGR file

    // The main event loop. 
    //
    // In this case, we're just looking for a value from the dial (which may be an emulation, as here, or real hardware.  We then display the
    // results to the debug window. 
    //

    while self.win.get_event() && !self.button_close.pressed()
    {
        self.check_dial_event();     // Check events we might want to emulate (in this case, just the dial)

        // Check local events -- in this case, we just have the about button, but could grow it over time.

        if self.button_about.pressed() { self.display_about_box(); }
    }
}
}

fn main() 
{
    let embedded_example = EmbeddedWidgetsExample::new();   // Create a new EmbeddedWidgetsExample instance

    embedded_example.run(); // Run it -- if the initialized failed, it just returns. 

    // If it did not initialize correctly, display a message
    // --> Sagebox::info_window_s(), but we can also use a regular OS MessageBox
    //     (e.g. Sagebox::message_box()), which uses system message boxes, based on the OS running the program. 

    if !embedded_example.is_valid() {Sagebox::info_window_s("The Embedded Example Did not Initialize Properly\n\
 The graphics resource .pgr file may not have been found, or some other error occurred.\n\nRefer to the error printed in the console window.",kw::icon_warning());  }
    else
    { Sagebox::exit_button(); } // Since we were in an event loop waiting for the window to close, this function is not needed.
                                // But, it is always nice to get a visible indication the program intended to end, so a button to press
                                // lets the user know everything is as intended, rather than closing down the window and returning the OS 
                                // suddenly. 
}
