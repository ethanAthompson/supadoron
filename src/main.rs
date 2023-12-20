pub mod components;
pub mod interfaces;
pub mod wrappers;

use crate::interfaces::primary_window::base::ConfigDisplay;
use crate::wrappers::rust_object::RonData;
use bevy::prelude::*;

fn main() {
    App::new().run();

    let config: ConfigDisplay = RonData::new("display.ron");
    println!("{:?}", &config.title);
    println!("{:?}", &config.dimensions);
}
