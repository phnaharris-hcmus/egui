#![deny(warnings)]

extern crate rusttype;
extern crate serde;

#[macro_use] // TODO: get rid of this
extern crate serde_derive;

pub mod color;
mod emigui;
pub mod example_app;
mod font;
mod fonts;
mod layers;
mod layout;
pub mod math;
mod memory;
pub mod mesher;
mod region;
mod style;
mod texture_atlas;
mod types;
pub mod widgets;
mod window;

pub use {
    crate::emigui::Emigui,
    color::Color,
    fonts::{FontDefinitions, Fonts, TextStyle},
    layers::*,
    layout::{Align, Id},
    math::*,
    memory::Memory,
    mesher::{Mesh, Vertex},
    region::Region,
    style::Style,
    texture_atlas::Texture,
    types::*,
    window::Window,
};
