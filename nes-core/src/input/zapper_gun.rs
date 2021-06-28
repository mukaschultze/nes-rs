use crate::input::InputBus;

pub struct ZapperGun {
    pub trigger_pulled: bool,
    pub light_sense: bool,
}

impl ZapperGun {
    pub fn new() -> Self {
        Self {
            trigger_pulled: false,
            light_sense: false,
        }
    }
}

impl InputBus for ZapperGun {
    fn input(&mut self, _value: u8) {}

    // https://wiki.nesdev.com/w/index.php/Zapper
    fn output(&mut self) -> u8 {
        let light_sensor = if self.light_sense { 0 } else { 1 << 3 }; // (0: detected; 1: not detected)
        let trigger = if self.trigger_pulled { 1 << 4 } else { 0 }; // (0: released; 1: pulled)
        light_sensor | trigger
    }
}
