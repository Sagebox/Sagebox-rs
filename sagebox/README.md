# Sagebox Core Library

This directory contains the core implementation of Sagebox for Rust.

## Layout

- `lib.rs` — Main library interface
- `colors.rs` — Definitions for pan_color and sage_color types, e.g. pan_color::ForestGreen
- `ext_func.rs` — FFI bindings
- `keywords.rs` — Compile-time keyword parsing and resolution
- `point.rs` — Advanced types for point operations, e.g. c = a*b, rather than c.x = a.x*b.x; c.y = a.y*b.y, etc. 

The `lib.rs` file serves as the main entry point, and includes the documentation that appears on `docs.rs`.

---

Most users will not need to dive into these internals unless they are extending or contributing to Sagebox itself.

## Work in Progress

As a release beta, these files are still being organized into basic structures and separate modules. 

The interface will grow significantly and become more refined in the next few months.

In addition to memory safety and overall architectural integrity, one of the key goals of the interface development
is to ensure comprehensive documentation — not just for individual functions, but for how controls, types, 
and functions relate to one another.

If you're interested in the internals or design of Sagebox, keep an eye on this space — and feel free to ask questions on the GitHub discussion page.


