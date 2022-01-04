use crate::types::{Byte, Word};

const ROM_SIZE: u16 = 32768;

pub struct Cartridge {
    pub name: String,
    pub rom_size: u16,
    rom_bank: [Byte; ROM_SIZE as usize],
}

impl Cartridge {
    pub fn new(name: &str, _data: &[Byte]) -> Self {
        Self {
            name: name.into(),
            rom_size: ROM_SIZE as u16,
            rom_bank: [0; ROM_SIZE as usize],
        }
    }

    pub fn read_rom(&self, address: Word) -> Byte {
        self.rom_bank[address as usize]
    }

    pub fn read_ram(&self, address: Word) -> Byte {
        todo!()
    }
}
