

// use ctrl+s for compile dependencies via cargo.toml

use std::env::var;
use log::info; // a way of importing packages/dependencies, and you can specify what you want inside from them.

// use std::io::Result; // another example



fn main() -> anyhow::Result<()>{

    // oh my god a comment, #turtles


    fern::Dispatch::default()

        .chain(std::io::stdout())
        .apply()?;

    log::info!("Hello nerds."); // this won't work because the logger has nothing attached to it.

    let variable = "value";
    let mut mut_variable = "value";
    mut_variable = "omg new variable";


    info!("{:?}", "value".as_ptr() == variable.as_ptr());

    //
    // assert_eq!("value".as_ptr(), variable.as_ptr());

    Ok(())

    // println!("Hello, world!");
}


// Rust was bridged by C and then became Rust after compilation