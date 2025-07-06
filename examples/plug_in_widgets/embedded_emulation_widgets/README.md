# Two Widgets / Mini-Emulator Example

This program demonstrates how the Dial and LCD widgets can work together to form the foundation of a simple hardware emulator. 

While this example runs locally, a slightly more abstracted version (unlike the all-in-one, somewhat monolithic version here) can be structured more for real emulation. 

Simply overload a few functions to connect it to actual hardware -- either to test individual components with the rest emulated, or to peer into the entire system working together in real-time.

## Projects in this directory

| Project Name | Description |
|--|--|
| `dial_widget_plain` | A simple program displaying a standalone dial control styled like a thermostat, with live value display. |
| `lcd_widget_plain` | A basic animated LCD-style counter, counting from 0 to 1,000,000, modeled after an actual LCD component on the market. |
| `lcd_widget_full` | Adding on the last example, this adds toggle buttons, button-text changes, styling options (e.g. blue LED mode), and performance modes. |
| `embedded_emulation_widgets` | A larger example showing use as an embedded emulator or embedded control program.  Combines the dial and LCD into a unified window, simulating an embedded control system with live debug output and a custom graphical About screen.
 |

## The Emulation Premise

The setup mimics a local temperature dial (e.g., for a wall or pool heater) and a remote embedded device  
that receives and displays the data on an LCD -- something Sagebox has been used for in industry to  
emulate and develop embedded applications.

What may seem like a small demo is actually how many real emulators begin -- simple at first,  
as either a proof of concept or a way to test one component, then gradually expanding into fully-featured systems.

## TLDR; Main points

 - Demonstrates how the Dial and LCD widgets can be used together to form a basic hardware emulator foundation.
 - Runs locally -- an abstracted version can connect to real hardware by overloading a few functions.
 - Allows testing individual components with the rest emulated or running the entire system.
 - Emulates a local temperature dial (like for a wall or pool heater) and a remote device displaying data on an LCD.
