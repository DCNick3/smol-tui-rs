#![no_std]
// Used for implementation of Frame storage (we create an array that is Width * Height long)
#![feature(generic_const_exprs)]

mod frame_accessor;
mod scene;
mod widget;
pub mod widgets;

pub use frame_accessor::*;
pub use scene::*;
pub use smol_tui_derive::*;
pub use widget::*;

pub use enum_dispatch::*;
