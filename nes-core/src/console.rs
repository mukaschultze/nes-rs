use crate::bus::DataBus;
use crate::cpu::CPU6502;
use crate::ppu::Ppu;
use crate::rom::rom_file::RomFile;
use std::cell::RefCell;
use std::rc::Rc;
use std::u16;

pub struct NesConsole {
    pub cpu: Rc<RefCell<CPU6502>>,
    pub bus: Rc<RefCell<DataBus>>,
    pub ppu: Rc<RefCell<Ppu>>,
}

impl NesConsole {
    pub fn new(rom: Rc<RefCell<RomFile>>) -> NesConsole {
        let bus = Rc::new(RefCell::new(DataBus::new(rom.clone())));
        let cpu = Rc::new(RefCell::new(CPU6502::new(bus.clone())));
        let ppu = Rc::new(RefCell::new(Ppu::new(
            cpu.clone(),
            bus.clone(),
            rom.clone(),
        )));

        {
            let cpu = cpu.clone();
            let bus = bus.clone();
            let mut bus_mut = bus.borrow_mut();
            let mut cpu_mut = cpu.borrow_mut();

            bus_mut.ppu = Some(ppu.clone());

            let pc_high = bus_mut.read(0xFFFD);
            let pc_low = bus_mut.read(0xFFFC);

            cpu_mut.pc = join_bytes!(pc_high, pc_low);
        }

        NesConsole { bus, cpu, ppu }
    }

    pub fn tick(&mut self) {
        self.cpu.borrow_mut().process_next_opcode();
        self.ppu.borrow_mut().tick();
        self.ppu.borrow_mut().tick();
        self.ppu.borrow_mut().tick();
    }
}
