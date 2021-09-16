#[derive(Clone)]
pub struct Registers {
    i: u16, // only lower 12 bit used, since we index into 4096 bytes;

    delay: u8,
    sound: u8,

    registers: [u8; 16],

}


impl Registers {
    pub fn new() -> Self {
        Self {
            i: 0,
            sound: 0,
            delay: 0,

            registers : [0; 16],
        }
    }

    #[inline]
    pub fn set_i(&mut self, addr: u16) {
        self.i = addr;
    }

    #[inline]
    pub fn increment_i(&mut self, val: u16) {
        self.i = self.i + val;
    }

    #[inline]
    pub fn get_i(&self) -> u16 {
        self.i
    }

    #[inline]
    pub fn get_value(&self, reg: u8) -> u8 {

        self.registers[reg as usize]
    }

    #[inline]
    pub fn set_value(&mut self, reg: u8, val: u8) {
        self.registers[reg as usize] = val;
    }

    #[inline]
    pub fn get_delay(&self) -> u8 {
        self.delay
    }


    #[inline]
    pub fn set_delay(&mut self, val: u8) {
        self.delay = val;
    }

    #[inline]
    pub fn set_sound(&mut self, val: u8) {
        self.sound = val;
    }


    pub fn bitwise(&mut self, reg_x: u8,
                   reg_y: u8,
                   f : fn(u8, u8) -> u8) {


        let r_x = reg_x as usize;
        let r_y = reg_y as usize;
        self.registers[r_x] = f(self.registers[r_x], self.registers[r_y]);
    }
}
