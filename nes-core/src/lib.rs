#[macro_use]
extern crate bitflags;

#[macro_use]
mod macros;

pub mod bus;
pub mod console;
pub mod cpu;
pub mod input;
pub mod palette;
pub mod ppu;
pub mod rom;
pub mod xbr;

#[cfg(test)]
mod test;
