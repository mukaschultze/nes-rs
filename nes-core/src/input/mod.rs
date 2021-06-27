pub mod joypad;
pub mod zapper_gun;

pub trait InputBus {
    fn input(&mut self, value: u8);
    fn output(&mut self) -> u8;
}

pub enum InputType {
    Joypad(joypad::Joypad),
    Zapper(zapper_gun::ZapperGun),
    Disconnected,
}

impl InputBus for InputType {
    fn input(&mut self, value: u8) {
        match self {
            InputType::Joypad(joypad) => joypad.input(value),
            InputType::Zapper(zapper) => zapper.input(value),
            InputType::Disconnected => (),
        }
    }

    fn output(&mut self) -> u8 {
        match self {
            InputType::Joypad(joypad) => joypad.output(),
            InputType::Zapper(zapper) => zapper.output(),
            InputType::Disconnected => 0,
        }
    }
}
