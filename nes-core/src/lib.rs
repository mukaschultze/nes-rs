#[macro_use]
mod macros;
pub mod bus;
pub mod console;
pub mod cpu;
pub mod rom;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Nes {
    pub cpu: Rc<RefCell<Cpu>>,
    pub bus: Rc<RefCell<Bus>>,
}

struct Cpu {
    pub bus: Rc<RefCell<Bus>>,
}

struct Bus {}

impl Nes {
    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus {}));
        let cpu = Rc::new(RefCell::new(Cpu { bus: bus.clone() }));

        Nes {
            cpu: cpu.clone(),
            bus: bus.clone(),
        }
    }

    pub fn init(&mut self) {}
}

impl Cpu {
    pub fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }
    pub fn write(&mut self, address: u16, value: u8) {
        self.bus.borrow_mut().write(address, value);
    }
}

impl Bus {
    pub fn read(&self, address: u16) -> u8 {
        0
    }
    pub fn write(&mut self, address: u16, value: u8) {}
}
