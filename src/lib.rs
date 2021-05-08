mod chip_eight;

use wasm_bindgen::prelude::*;

use chip_eight::ChipEight;

const EXAMPLE_PROGRAM: &'static [u8] = include_bytes!("maze.ch8");

#[wasm_bindgen]
pub struct WasmChipEight {
    chip_eight: ChipEight,
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
impl WasmChipEight {
    #[wasm_bindgen(constructor)]
    pub fn new(program: &[u8], seed: u32) -> Self {
        Self {
            chip_eight: ChipEight::new(program, seed),
        }
    }

    pub fn get_example_program() -> Box<[u8]> {
        EXAMPLE_PROGRAM.into()
    }

    pub fn step(&mut self) -> String {
        let pc = self.chip_eight.pc();
        let instruction = self.chip_eight.fetch_decode();
        self.chip_eight.execute(instruction);
        format!("0x{:03x}: {:x?}", pc, instruction)
    }

    pub fn buffer_width() -> usize {
        ChipEight::buffer_width()
    }

    pub fn buffer_height() -> usize {
        ChipEight::buffer_height()
    }

    pub fn get_buffer(&self, x: usize, y: usize) -> bool {
        self.chip_eight.get_buffer(x, y)
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
