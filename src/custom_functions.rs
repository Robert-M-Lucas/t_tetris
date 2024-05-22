//! A simple example


use bevy::prelude::*;
use bevy_dev_console::builtin_parser::Environment;
use bevy_dev_console::register;

fn echo(string: String) {
    info!("Echo: {string}");
}


// Register our functions by creating and inserting our own environment
pub fn dev_console_environment() -> Environment {
    let mut environment = Environment::default();

    // The register macro allows us to easily add functions to the environment.
    register!(&mut environment => {
        fn echo;
    });

    environment
}