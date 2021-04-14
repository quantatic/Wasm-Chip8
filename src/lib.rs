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
	pub fn new() -> Self {
		Self {
			chip_eight: Default::default(),
		}
	}

    pub fn step(&mut self) -> u16 {
        self.chip_eight.step()
    }
}
