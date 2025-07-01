#[cfg(feature = "link_dynamic")]
use std::env;
#[cfg(feature = "link_dynamic")]
use std::path::PathBuf;

fn main()
{

// DLL Linkage -- Make sure DLL is found in executable path or top-level directory

#[cfg(feature = "link_dynamic")]
{
    println!("cargo:warning=Linking with DLL (.lib + .dll) library.");

    // Set specific path to .DLL

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = manifest_dir.join("lib/dynamic");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=lcd_widget");
    println!("cargo:rustc-link-search=lib/dynamic");


}

// Static .LIB linkage
//
// --> For static linking, add 'features = [ "link_static"] after the library in the dependency, 
//     - e.g. sagebox = { path = "..\\sagebox", features = [ "link static "] }

#[cfg(not(feature = "link_dynamic"))]
{
    println!("cargo:warning=Linking with static (.lib) library.");

    // We only need a relative path to the .LIB

    println!("cargo:rustc-link-search=static=lib/static");
    println!("cargo:rustc-link-lib=static=lcd_widget");
    println!("cargo:rustc-link-search=lib/static");
}

}