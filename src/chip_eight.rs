use wasm_bindgen::prelude::*;

use std::default::Default;

const MEMORY_SIZE: usize = 4096;
const NUM_V_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;

#[derive(Debug)]
pub struct ChipEight {
    memory: [u8; MEMORY_SIZE],
    v: [u8; NUM_V_REGISTERS],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
}

impl Default for ChipEight {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            v: [0; NUM_V_REGISTERS],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; STACK_SIZE]
        }
    }
}

impl ChipEight {
    pub fn step(&mut self) -> u16 {
        self.pc += 1;
        self.pc
    }
}
