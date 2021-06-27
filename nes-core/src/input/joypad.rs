use crate::input::InputBus;

bitflags! {
    #[derive(Default)]
    pub struct JoypadDataLine: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const SELECT = 1 << 2;
        const START = 1 << 3;
        const UP = 1 << 4;
        const DOWN = 1 << 5;
        const LEFT = 1 << 6;
        const RIGHT = 1 << 7;
    }
}

pub struct Joypad {
    shift: u8,
    strobe: bool,
    pub data: JoypadDataLine,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            shift: 0,
            strobe: false,
            data: Default::default(),
        }
    }
}

impl InputBus for Joypad {
    // https://wiki.nesdev.com/w/index.php/Standard_controller#Input_.28.244016_write.29
    fn input(&mut self, value: u8) {
        self.strobe = (value & 1) != 0;

        if self.strobe {
            self.shift = 0;
        }
    }

    // https://wiki.nesdev.com/w/index.php/Standard_controller#Output_.28.244016.2F.244017_read.29
    fn output(&mut self) -> u8 {
        if self.shift >= 8 {
            return 1;
        }

        let result = (self.data.bits() >> self.shift) & 1;

        if !self.strobe {
            self.shift += 1;
        }

        result
    }
}
