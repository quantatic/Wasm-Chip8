const BUILTIN_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0x0
    0x20, 0x60, 0x20, 0x20, 0x70, // 0x1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 0x2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 0x3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 0x4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 0x5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 0x6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 0x7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 0x8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 0x9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // 0xA
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // 0xB
    0xF0, 0x80, 0x80, 0x80, 0xF0, // 0xC
    0xE0, 0x90, 0x90, 0x90, 0xE0, // 0xD
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // 0xE
    0xF0, 0x80, 0xF0, 0x80, 0x80, // 0xF
];

const MEMORY_SIZE: usize = 4096;
const NUM_V_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;

const PROGRAM_OFFSET: u16 = 0x200;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Sys(u16),
    Cls,
    Ret,
    Jp(u16),
    Call(u16),
    SeVxByte(u8, u8),
    SneVxByte(u8, u8),
    SeVxVy(u8, u8),
    LdVxByte(u8, u8),
    AddVxByte(u8, u8),
    LdVxVy(u8, u8),
    OrVxVy(u8, u8),
    AndVxVy(u8, u8),
    XorVxVy(u8, u8),
    AddVxVy(u8, u8),
    SubVxVy(u8, u8),
    ShrVx(u8),
    SubnVxVy(u8, u8),
    ShlVx(u8),
    SneVxVy(u8, u8),
    LdIAddr(u16),
    JpV0Addr(u16),
    RndVxByte(u8, u8),
    DrwVxVyNibble(u8, u8, u8),
    SkpVx(u8),
    SknpVx(u8),
    LdVxDt(u8),
    LdVxK(u8),
    LdDtVx(u8),
    LdStVx(u8),
    AddIVx(u8),
    LdFVx(u8),
    LdBVx(u8),
    LdIVx(u8),
    LdVxI(u8),
}

#[derive(Debug)]
pub struct ChipEight {
    memory: Vec<u8>,
    v: Vec<u8>,
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    stack: Vec<u16>,
    display_buffer: Box<[[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH]>,
    rng_state: u32,
}

impl ChipEight {
    pub fn new(program: &[u8], seed: u32) -> Self {
        let mut memory = vec![0; MEMORY_SIZE];

        for i in 0..BUILTIN_SPRITES.len() {
            memory[i] = BUILTIN_SPRITES[i];
        }

        for i in 0..program.len() {
            memory[i + usize::from(PROGRAM_OFFSET)] = program[i];
        }

        Self {
            memory,
            v: vec![0; NUM_V_REGISTERS],
            i: 0,
            pc: PROGRAM_OFFSET,
            sp: 0,
            dt: 0,
            st: 0,
            stack: vec![0; STACK_SIZE],
            display_buffer: Box::new([[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH]),
            rng_state: seed,
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch_decode();
        self.execute(instruction);
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn fetch_decode(&mut self) -> Instruction {
        let byte_high = self.memory[usize::from(self.pc)];
        let byte_low = self.memory[usize::from(self.pc) + 1];

        let nibbles = (
            (byte_high >> 4) & 0xF,
            byte_high & 0xF,
            (byte_low >> 4) & 0xF,
            byte_low & 0xF,
        );

        let nnn = ((u16::from(byte_high) & 0x0F) << 8) | u16::from(byte_low);
        let kk = byte_low;

        let instruction = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Cls,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Ret,
            (0x0, _, _, _) => Instruction::Sys(nnn),
            (0x1, _, _, _) => Instruction::Jp(nnn),
            (0x2, _, _, _) => Instruction::Call(nnn),
            (0x3, x, _, _) => Instruction::SeVxByte(x, kk),
            (0x4, x, _, _) => Instruction::SneVxByte(x, kk),
            (0x5, x, y, 0x0) => Instruction::SeVxVy(x, y),
            (0x6, x, _, _) => Instruction::LdVxByte(x, kk),
            (0x7, x, _, _) => Instruction::AddVxByte(x, kk),
            (0x8, x, y, 0x0) => Instruction::LdVxVy(x, y),
            (0x8, x, y, 0x1) => Instruction::OrVxVy(x, y),
            (0x8, x, y, 0x2) => Instruction::AndVxVy(x, y),
            (0x8, x, y, 0x3) => Instruction::XorVxVy(x, y),
            (0x8, x, y, 0x4) => Instruction::AddVxVy(x, y),
            (0x8, x, y, 0x5) => Instruction::SubVxVy(x, y),
            (0x8, x, _, 0x6) => Instruction::ShrVx(x),
            (0x8, x, y, 0x7) => Instruction::SubnVxVy(x, y),
            (0x8, x, _y, 0xE) => Instruction::ShlVx(x),
            (0x9, x, y, 0x0) => Instruction::SneVxVy(x, y),
            (0xA, _, _, _) => Instruction::LdIAddr(nnn),
            (0xB, _, _, _) => Instruction::JpV0Addr(nnn),
            (0xC, x, _, _) => Instruction::RndVxByte(x, kk),
            (0xD, x, y, n) => Instruction::DrwVxVyNibble(x, y, n),
            (0xE, x, 0x9, 0xE) => Instruction::SkpVx(x),
            (0xE, x, 0xA, 0x1) => Instruction::SknpVx(x),
            (0xF, x, 0x0, 0x7) => Instruction::LdVxDt(x),
            (0xF, x, 0x0, 0xA) => Instruction::LdVxK(x),
            (0xF, x, 0x1, 0x5) => Instruction::LdDtVx(x),
            (0xF, x, 0x1, 0x8) => Instruction::LdStVx(x),
            (0xF, x, 0x1, 0xE) => Instruction::AddIVx(x),
            (0xF, x, 0x2, 0x9) => Instruction::LdFVx(x),
            (0xF, x, 0x3, 0x3) => Instruction::LdBVx(x),
            (0xF, x, 0x5, 0x5) => Instruction::LdIVx(x),
            (0xF, x, 0x6, 0x5) => Instruction::LdVxI(x),
            _ => unreachable!("Unrecognized instruction nibbles: {:x?}", nibbles),
        };

        self.pc += 2;

        instruction
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Cls => self.cls(),
            Instruction::Ret => self.ret(),
            Instruction::Sys(_) => unreachable!("Sys should never be called"),
            Instruction::Jp(addr) => self.jp(addr),
            Instruction::Call(addr) => self.call(addr),
            Instruction::SeVxByte(x, byte) => self.se_vx_byte(x, byte),
            Instruction::SneVxByte(x, byte) => self.sne_vx_byte(x, byte),
            Instruction::SeVxVy(x, y) => self.se_vx_vy(x, y),
            Instruction::LdVxByte(x, byte) => self.ld_vx_byte(x, byte),
            Instruction::AddVxByte(x, byte) => self.add_vx_byte(x, byte),
            Instruction::LdVxVy(x, y) => self.ld_vx_vy(x, y),
            Instruction::OrVxVy(x, y) => self.or_vx_vy(x, y),
            Instruction::AndVxVy(x, y) => self.and_vx_vy(x, y),
            Instruction::XorVxVy(x, y) => self.xor_vx_vy(x, y),
            Instruction::AddVxVy(x, y) => self.add_vx_vy(x, y),
            Instruction::SubVxVy(x, y) => self.sub_vx_vy(x, y),
            Instruction::ShrVx(x) => self.shr_vx(x),
            Instruction::SubnVxVy(x, y) => self.subn_vx_vy(x, y),
            Instruction::ShlVx(x) => self.shl_vx(x),
            Instruction::SneVxVy(x, y) => self.sne_vx_vy(x, y),
            Instruction::LdIAddr(addr) => self.ld_i_addr(addr),
            Instruction::JpV0Addr(addr) => self.jp_v0_addr(addr),
            Instruction::RndVxByte(x, byte) => self.rnd_vx_byte(x, byte),
            Instruction::DrwVxVyNibble(x, y, nibble) => self.drw_vx_vy_nibble(x, y, nibble),
            Instruction::SkpVx(x) => self.skp_vx(x),
            Instruction::SknpVx(x) => self.sknp_vx(x),
            Instruction::LdVxDt(x) => self.ld_vx_dt(x),
            Instruction::LdVxK(x) => self.ld_vx_k(x),
            Instruction::LdDtVx(x) => self.ld_dt_vx(x),
            Instruction::LdStVx(x) => self.ld_st_vx(x),
            Instruction::AddIVx(x) => self.add_i_vx(x),
            Instruction::LdFVx(x) => self.ld_f_vx(x),
            Instruction::LdBVx(x) => self.ld_b_vx(x),
            Instruction::LdIVx(x) => self.ld_i_vx(x),
            Instruction::LdVxI(x) => self.ld_vx_i(x),
        }
    }

    fn get_reg(&self, reg: u8) -> u8 {
        self.v[usize::from(reg)]
    }

    fn set_reg(&mut self, reg: u8, val: u8) {
        self.v[usize::from(reg)] = val;
    }

    fn get_vf(&self) -> u8 {
        self.get_reg(0xF)
    }

    fn set_vf(&mut self, val: bool) {
        self.set_reg(0xF, val as u8)
    }

    pub fn buffer_width() -> usize {
        DISPLAY_WIDTH
    }

    pub fn buffer_height() -> usize {
        DISPLAY_HEIGHT
    }

    pub fn get_buffer(&self, x: usize, y: usize) -> bool {
        self.display_buffer[usize::from(x)][usize::from(y)]
    }

    fn set_buffer(&mut self, x: usize, y: usize, val: bool) {
        self.display_buffer[usize::from(x)][usize::from(y)] = val;
    }

    fn cls(&mut self) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                self.set_buffer(x, y, false);
            }
        }
    }

    fn ret(&mut self) {
        self.pc = self.stack[usize::from(self.sp)];
        self.sp -= 1;
    }

    fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.sp += 1;
        self.stack[usize::from(self.sp)] = self.pc;
        self.pc = addr;
    }

    fn se_vx_byte(&mut self, x: u8, byte: u8) {
        if self.get_reg(x) == byte {
            self.pc += 2;
        }
    }

    fn sne_vx_byte(&mut self, x: u8, byte: u8) {
        if self.get_reg(x) != byte {
            self.pc += 2;
        }
    }

    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.get_reg(x) == self.get_reg(y) {
            self.pc += 2;
        }
    }

    fn ld_vx_byte(&mut self, x: u8, byte: u8) {
        self.set_reg(x, byte)
    }

    fn add_vx_byte(&mut self, x: u8, byte: u8) {
        let new_val = self.get_reg(x).wrapping_add(byte);
        self.set_reg(x, new_val);
    }

    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        let new_val = self.get_reg(y);
        self.set_reg(x, new_val);
    }

    fn or_vx_vy(&mut self, x: u8, y: u8) {
        let new_val = self.get_reg(x) | self.get_reg(y);
        self.set_reg(x, new_val);
    }

    fn and_vx_vy(&mut self, x: u8, y: u8) {
        let new_val = self.get_reg(x) & self.get_reg(y);
        self.set_reg(x, new_val);
    }

    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        let new_val = self.get_reg(x) ^ self.get_reg(y);
        self.set_reg(x, new_val);
    }

    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let (new_val, carry) = self.get_reg(x).overflowing_add(self.get_reg(y));
        self.set_vf(carry);

        self.set_reg(x, new_val);
    }

    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        let (new_val, carry) = self.get_reg(x).overflowing_sub(self.get_reg(y));
        self.set_vf(!carry);

        self.set_reg(x, new_val);
    }

    fn shr_vx(&mut self, x: u8) {
        let old_val = self.get_reg(x);
        let new_val = old_val >> 1;
        self.set_vf((old_val & 0x01) != 0);

        self.set_reg(x, new_val);
    }

    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        let (new_val, carry) = self.get_reg(y).overflowing_sub(self.get_reg(x));
        self.set_vf(!carry);

        self.set_reg(x, new_val);
    }

    fn shl_vx(&mut self, x: u8) {
        let old_val = self.get_reg(x);
        let new_val = old_val << 1;
        self.set_vf((old_val & 0x80) != 0);

        self.set_reg(x, new_val);
    }

    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.get_reg(x) != self.get_reg(y) {
            self.pc += 2;
        }
    }

    fn ld_i_addr(&mut self, addr: u16) {
        self.i = addr;
    }

    fn jp_v0_addr(&mut self, addr: u16) {
        self.pc = u16::from(self.get_reg(0)) + addr;
    }

    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    //
    // Use glibc values:
    // m: 2^31
    // a: 1103515245
    // c: 12345
    // We get the middle 8 bits here because the last 8 bits seem to alternate
    //   between even and odd, and we don't want that.
    fn rnd_vx_byte(&mut self, x: u8, byte: u8) {
        self.rng_state =
            self.rng_state.wrapping_mul(1103515245).wrapping_add(12345) % u32::pow(2, 31);

        let rand_byte = (self.rng_state >> 8) as u8;
        let new_val = rand_byte & byte;

        self.set_reg(x, new_val);
    }

    fn drw_vx_vy_nibble(&mut self, x: u8, y: u8, nibble: u8) {
        let mut collision = false;
        let base_x_coord = self.get_reg(x);
        let base_y_coord = self.get_reg(y);
        for dy in 0..nibble {
            let byte = self.memory[usize::from(self.i) + usize::from(dy)];
            for dx in 0..8 {
                let x = usize::from(base_x_coord + dx) % DISPLAY_WIDTH;
                let y = usize::from(base_y_coord + dy) % DISPLAY_HEIGHT;
                let flip = byte & (1 << (7 - dx)) != 0;
                if flip {
                    let flipped_pixel = !self.get_buffer(x, y);

                    // Collision is set to true iff flipped pixel is now off.
                    collision |= !flipped_pixel;

                    self.set_buffer(x, y, flipped_pixel);
                }
            }
        }

        self.set_vf(collision);
    }

    fn skp_vx(&mut self, _x: u8) {
        unimplemented!()
    }

    fn sknp_vx(&mut self, _x: u8) {
        unimplemented!()
    }

    fn ld_vx_dt(&mut self, x: u8) {
        self.set_reg(x, self.dt);
    }

    fn ld_vx_k(&mut self, _x: u8) {
        unimplemented!()
    }

    fn ld_dt_vx(&mut self, x: u8) {
        self.dt = self.get_reg(x);
    }

    fn ld_st_vx(&mut self, x: u8) {
        self.st = self.get_reg(x);
    }

    fn add_i_vx(&mut self, x: u8) {
        self.i += u16::from(self.get_reg(x));
    }

    fn ld_f_vx(&mut self, x: u8) {
        let digit_wanted = self.get_reg(x);
        assert!(x <= 0xF);
        self.i = u16::from(digit_wanted) * 5;
    }

    fn ld_b_vx(&mut self, x: u8) {
        let value = self.get_reg(x);

        let hundreds = value / 100;
        let tens = (value % 100) / 10;
        let ones = value % 10;

        self.memory[usize::from(self.i)] = hundreds;
        self.memory[usize::from(self.i) + 1] = tens;
        self.memory[usize::from(self.i) + 2] = ones;
    }

    fn ld_i_vx(&mut self, x: u8) {
        for idx in 0..=x {
            self.memory[usize::from(self.i) + usize::from(idx)] = self.get_reg(idx);
        }
    }

    fn ld_vx_i(&mut self, x: u8) {
        for idx in 0..=x {
            let new_val = self.memory[usize::from(self.i) + usize::from(idx)];
            self.set_reg(idx, new_val);
        }
    }
}
