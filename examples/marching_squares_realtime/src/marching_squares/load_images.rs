
// --------------------------------------
// Loading Images -- Memory vs. PGR Files
// --------------------------------------
//
// This functions defaults to using a Sagebox PGR file, which stores files (graphics, others) as well as v program variables, created
// from the text file GraphicFiles.txt
//
// Using the PGR file is a later-step process, before which files are loaded from memory.
// This functions chooses the method based on kUsePGR (set to true), just to show the difference.
//
// PGR files are useful, but loading from files is the way most things get done until it is time to pack resources into 
// a common file or set of memory, so I thought it was important to show both methods (e.g. development where we load files, and a later refactoring stage
// where we look at collecting files into a common resource such as a PGR or some other type of common resource).
//
// --> Note: The PGR file in memory has the advantage that the program does not need to look for and verify other files -- only the program itself
//           is required to run, since the files are loaded into memory.
//

use super::MarchingSquares;
use sagebox::*;

//const IMAGE_PATH : &str = "C:\\Users\\robne\\source\\repos\\Marching Squares\\Marching Squares";    // Image path for image files or .PGR fil
const IMAGE_PATH : &str = "./";    // Image path for image files or .PGR fil

macro_rules! get_file { ($file:expr) => { format!("{}\\{}",IMAGE_PATH,$file).as_str() } }           // Macro to make getting image path easier.

impl MarchingSquares
{

/// load_pgr_images() -- Load images (and a couple variables) from the PGR file in memory.
///
/// The original source was GraphicFiles.txt, which is a JSON-like text file that is compiled into .PGR file, containing the
/// graphics files and program variables.
///
/// - PGR files are a nice way to collect and store separate files, rather than loading them from disk independently.
/// 
/// - As a collection of separate files in memory, this means we don't need to distribute any more files that can get lost or not downloaded, and otherwise
///   need to be checked for loading with each item.
/// 
/// - With file-based PGR files (from disk, where the one in the code below is in memory), we just need to check the is_valid() function to ensure proper loading, 
///   rather than check each file individually (though we can do that, too, to ensure the names were spelled correctly in the call to load each individual item).
/// 
/// - PGR files are is also a great place to keep certain variables that related to the loaded files or profiles, which would otherwise be
/// hard-coded in the code or interface.
///
/// This file is loaded from memory, and, since we know it all works, there is no error-checking performed. 
/// Should there be any error, nothing will crash, and the only result would be that some bitmaps just don't display (as they would be empty).
///
/// In this case, we know it all works, and since it's not a larger-scale, multi-person effort (where we would want to be more stringent, such as in the
/// case of misspellings in the text-based file names, for example), we don't have to worry about it once we verified it works once.
///
/// When loading a file-based PGR File (i.e. from disk), we just need to check the is_valid() function for the returned object to ensure the PGR file
/// was loaded and pass it's internal CRC-check successfully.
///
/// --> See load_pgr_files() for a version that loads the files from disk individually, since the PGR inclusion is a later-step, and
///     loading files individually is part of the development process before it gets refactored to use a PGR or other method to mass-load resources.
///
/// --> How to load from the GraphicFiles.pgr on disk, instead (when we don't want to put it into memory):
///       -> Change Sagebox::load_pgr_mem() to Sagebox::load_pgr_file("GraphicFiles.pgr"), which will return the same exact set of memory.
///
pub (crate) fn load_pgr_images(&mut self) -> bool
{
//    let pgr = Sagebox::read_pgr_file(format!("{}\\GraphicFiles.pgr",IMAGE_PATH).as_str()); 
    let pgr = Sagebox::read_pgr_file(get_file!("marching_squares_images.pgr")); 

    // Load the files by their name in the GraphicFiles.txt -- in a larger, refactored project, these names would
    // be symbolic (e.g. kBitmapZoomWindow) to avoid spelling mistakes. 

    self.bitmap_top         = pgr.read_image_file32("Bitmaps:SplashTop");   // These two images have masks along with them and are loaded as
    self.bitmap_bot         = pgr.read_image_file32("Bitmaps:SplashBot");   // 32-bit images. $$ .PNG would be the better (smaller) format. 
    self.bitmap_zoom_window = pgr.read_image_file("Bitmaps:ZoomWindow");
    self.dev_win_background = pgr.read_image_file("Bitmaps:DevWinBackground");

    pgr.is_valid()      // We don't need to check, since we know the files are there.  The only thing that can go wrong here is 
                        // if there is a misspelling in the text names above, but once it works, it works ok, so 
                        // we just let it go here, since it's not a larger multi-person project where such stringent checking
                        // would make more sense (and essentially be mandated anyway).
}

/// load_image_files() -- Load the image files from disk, and set some variables related to them.
///
/// This loads the bitmaps from disk (before the PGR memory file was included). 
/// 
/// This function sets a couple values that relate to using the loaded bitmaps -- in the PGR file, these are set in the GraphicFiles.txt next to
/// the inclusion of the graphic files. 
///
pub (crate)  fn load_image_files(&mut self) -> bool
{
    // Load the bitmaps from disk individually -- these would also be symbolic names (e.g. kBitmapSplashTop) in a larger project, or in 
    // a later refactoring step. 

    self.bitmap_top = Sagebox::read_image_file32(get_file!("images/splash-top.bmp"));
    self.bitmap_bot = Sagebox::read_image_file32(get_file!("images/splash-bot.bmp"));
    self.bitmap_zoom_window = Sagebox::read_image_file(get_file!("images/zoomwindow.jpg"));
    self.dev_win_background = Sagebox::read_image_file(get_file!("images/devwinbackground.jpg"));
    // Check validity of files (i.e. make sure they were found and read properly). 

    self.bitmap_top.is_valid() && self.bitmap_top.is_valid() && self.bitmap_zoom_window.is_valid() && self.dev_win_background.is_valid()
}


/// load_images() -- Load all images needed for the program to work. 
/// This function returns true of all images are loaded correctly; false if something was not loaded.
///
pub (crate) fn load_images(&mut self) -> bool
{
    let use_pgr_file = true;
    if use_pgr_file { self.load_pgr_images() } else { self.load_image_files() }
}

}