pub trait Mapper {
    fn get_id(&self) -> u8;
    fn get_name(&self) -> &'static str;

    fn read(&self, address: u16) -> u8;
    fn write(&self, address: u16, value: u8);
}

pub mod mapper0;
