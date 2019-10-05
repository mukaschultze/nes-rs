#![feature(test)]

#[macro_use]
extern crate bitflags;

#[macro_use]
mod macros;

pub mod bus;
pub mod console;
pub mod controller;
pub mod cpu;
pub mod palette;
pub mod ppu;
pub mod rom;

#[cfg(test)]
mod test;
