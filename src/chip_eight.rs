pub const MEMORY_SIZE: usize = 4096;
const NUM_V_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;

#[derive(Debug)]
pub struct ChipEight {
    memory: Vec<u8>,
    v: Vec<u8>,
    i: u16,
    pc: u16,
    sp: u8,
    stack: Vec<u16>
}

impl ChipEight {
	pub fn new(memory: &[u8]) -> Self {
		assert!(memory.len() == MEMORY_SIZE);
		Self {
			memory: memory.to_vec(),
			v: vec![0; NUM_V_REGISTERS],
			i: 0,
			pc: 0,
			sp: 0,
			stack: vec![0; STACK_SIZE],
		}
	}

    pub fn step(&mut self) -> u16 {
        self.pc += 1;
        self.pc
    }

	pub fn get_memory(&self, idx: usize) -> u8 {
		self.memory[idx]
	}
}
