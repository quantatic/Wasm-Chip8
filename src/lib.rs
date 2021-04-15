mod chip_eight;

use wasm_bindgen::prelude::*;

use chip_eight::ChipEight;

#[wasm_bindgen]
pub struct WasmChipEight {
    chip_eight: ChipEight,
}

#[wasm_bindgen]
impl WasmChipEight {
	#[wasm_bindgen(constructor)]
	pub fn new(memory: &[u8]) -> Self {
		assert!(memory.len() == chip_eight::MEMORY_SIZE);

		Self {
			chip_eight: ChipEight::new(memory)
		}
	}

    pub fn step(&mut self) -> u16 {
        self.chip_eight.step()
    }

	pub fn get_memory(&self, idx: usize) -> u8 {
		self.chip_eight.get_memory(idx)
	}
}

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace=console, js_name=log)]
	pub fn console_log(val: &str);
}

#[wasm_bindgen(start)]
pub fn do_something() {
	console_log("Hello, World!\n");
}
