mod bios;
mod cartridge;

use crate::memory::bios::GB_BIOS;
use crate::memory::cartridge::Cartridge;
use crate::types::{to_word, Byte, Word};

pub struct Memory {
    bios: Vec<Byte>,
    in_bios: bool,
    cartridge: Option<Cartridge>,
    sprite_memory: [Byte; 160],
    video_memory: [Byte; 8192],
    working_memory: [Byte; 65536],
    zero_page_memory: [Byte; 128],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bios: GB_BIOS,
            in_bios: true,
            cartridge: None,
            sprite_memory: [0; 160],
            video_memory: [0; 8192],
            working_memory: [0; 65536],
            zero_page_memory: [0; 128],
        }
    }

    pub fn read_byte(&self, address: Word) -> Byte {
        let Self {
            bios,
            in_bios,
            cartridge,
            sprite_memory,
            video_memory,
            working_memory,
            zero_page_memory,
        } = self;

        let cartridge = cartridge
            .as_ref()
            .expect("Called read_byte without loading cartridge first.");

        // need to adjust idx per memory array
        let idx = address as usize;

        match address {
            // cartridge - rom
            0x0000..0x8000 => {
                if *in_bios && address < 0x0100 {
                    bios[idx]
                } else {
                    cartridge.read_rom(address)
                }
            }
            // video
            0x8000..0xa000 => video_memory[idx],
            // cartridge - ram
            0xa000..0xc000 => cartridge.read_ram(address),
            // working memory
            0xc000..0xe000 => working_memory[idx],
            // working memory echo
            0xe000..0xfe00 => self.read_byte(address - 0x2000),
            // sprite attribute memory
            0xfe00..0xfea0 => sprite_memory[idx],
            // empty
            0xfea0..0xff00 => 0,
            // io
            0xff00..0xff80 => todo!(),
            // zero page ram
            0xff80..0xffff => zero_page_memory[idx],
            _ => panic!("Illegal read_byte address: {}", address),
        }
    }

    pub fn read_word(&self, address: Word) -> Word {
        let b1 = self.read_byte(address);
        let b2 = self.read_byte(address + 1);

        to_word(b1, b2)
    }
}
