// -----------------------------------------------
// Sagebox Marching Squares Explorer Demo for Rust
// -----------------------------------------------
//
// This example uses traditional (non-GPU) graphics to render a real-time outline of a dynamic 
// Meta Ball structure using the Marching Squares algorithm.
//
// It’s designed as a more complete demonstration of Sagebox in Rust — going beyond basic examples 
// to show how interactive graphics, real-time performance, and UI elements can work together in 
// medium-complexity educational tools, research projects, or early-stage prototypes.
//
// Even in debug mode, it runs at 60fps with a 5x5 grid or larger. Smaller grids benefit from 
// release builds, which are significantly faster.
//
// ------------------
// About this Example
// ------------------
//
// The core Marching Squares and Meta Ball logic took about 2–3 hours to implement, with additional 
// time spent integrating UI controls like sliders and checkboxes to explore behavior dynamically.
//
// The idea here is to show how a using a program using Sagebox can grow naturally — starting with a 
// physics or graphics algorithm and adding interactivity without needing to restructure the program.
//
// -------------------------------
// Notes on Graphics and GPU Usage
// -------------------------------
//
// This version uses CPU-based rendering only — no GPU acceleration. The goal is to demonstrate 
// how much can be done with classic 2D graphics, which can make early-stage development simpler 
// and more accessible.
//
// Moving this to the GPU is certainly possible (and Sagebox will support GPU features), but many 
// types of graphical applications can be prototyped or completed entirely using traditional methods.
//
// -----------------------------
// Using or Exploring the Code
// -----------------------------
//
// This program is a working example of how Sagebox functions can be combined to build larger 
// applications. Feel free to borrow code, experiment, and adapt it for your own use.
//
// For now, the examples themselves act as a kind of living documentation, showing how and why 
// Sagebox was built — focused on simplicity, flexibility, and making UI-heavy or graphics-based 
// applications easier to build from scratch.
//
// ---------------------------------------
// Code Comments and Style in this Example
// ---------------------------------------
//
// The code includes many comments to help explain Sagebox behavior and typical use. In general, 
// the style follows what I use for collaborative environments — readable, clear, and ready for 
// others to pick up.
//
// Some extra comments are added for educational clarity and to highlight how Sagebox fits into 
// the program structure.

pub mod marching_squares;
pub mod zoom_window;
use sagebox::*;

fn main()
{
    let mut m_squares = marching_squares::MarchingSquares::new();
    m_squares.run();

    // Add a window with a button letting the user know the program is over. 
    // This isn't strictly needed since the user dismissed the window themselves, but
    // it can be a nice touch.
    
    Sagebox::exit_button();
}
