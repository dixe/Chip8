use crate::registers::*;
use crate::keyboard::*;
use crate::display::*;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub stack: [u16; 16],
    pub registers: Registers,
    pub keyboard: Keyboard,
    pub pc: u16,
    pub sp: u8,
    pub display: Display
}


impl Chip8 {
    pub fn new() -> Self {
        let mut res = Self {
            memory : [0; 4096],
            stack : [0; 16],
            registers: Registers::new(),
            keyboard: Keyboard::new(),
            pc: 0x200,
            sp: 0,
            display: Display::new()
        };


        // initialize the char sprites, store them from 0 to 80 (5*16)
        /* Standard 4x5 font */
        /* '0' */ res.memory[0] = 0xF0; res.memory[0 + 1] = 0x90; res.memory[0 + 2] = 0x90; res.memory[0 + 3] = 0x90; res.memory[0 + 4] = 0xF0;
        /* '1' */ res.memory[5] = 0x20; res.memory[5 + 1] = 0x60; res.memory[5 + 2] = 0x20; res.memory[5 + 3] = 0x20; res.memory[5 + 4] = 0x70;
        /* '2' */ res.memory[10] = 0xF0; res.memory[10 + 1] = 0x10; res.memory[10 + 2] = 0xF0; res.memory[10 + 3] = 0x80; res.memory[10 + 4] = 0xF0;
        /* '3' */ res.memory[15] = 0xF0; res.memory[15 + 1] = 0x10; res.memory[15 + 2] = 0xF0; res.memory[15 + 3] = 0x10; res.memory[15 + 4] = 0xF0;
        /* '4' */ res.memory[20] = 0x90; res.memory[20 + 1] = 0x90; res.memory[20 + 2] = 0xF0; res.memory[20 + 3] = 0x10; res.memory[20 + 4] = 0x10;
        /* '5' */ res.memory[25] = 0xF0; res.memory[25 + 1] = 0x80; res.memory[25 + 2] = 0xF0; res.memory[25 + 3] = 0x10; res.memory[25 + 4] = 0xF0;
        /* '6' */ res.memory[30] = 0xF0; res.memory[30 + 1] = 0x80; res.memory[30 + 2] = 0xF0; res.memory[30 + 3] = 0x90; res.memory[30 + 4] = 0xF0;
        /* '7' */ res.memory[35] = 0xF0; res.memory[35 + 1] = 0x10; res.memory[35 + 2] = 0x20; res.memory[35 + 3] = 0x40; res.memory[35 + 4] = 0x40;
        /* '8' */ res.memory[40] = 0xF0; res.memory[40 + 1] = 0x90; res.memory[40 + 2] = 0xF0; res.memory[40 + 3] = 0x90; res.memory[40 + 4] = 0xF0;
        /* '9' */ res.memory[45] = 0xF0;  res.memory[45 + 1] = 0x90;  res.memory[45 + 2] = 0xF0;  res.memory[45 + 3] = 0x10; res.memory[45 + 4] =  0xF0;
        /* 'A' */ res.memory[50] = 0xF0; res.memory[50 + 1] = 0x90; res.memory[50 + 2] = 0xF0; res.memory[50 + 3] = 0x90; res.memory[50 + 4] = 0x90;
        /* 'B' */ res.memory[55] = 0xE0; res.memory[55 + 1] = 0x90; res.memory[55 + 2] = 0xE0; res.memory[55 + 3] = 0x90; res.memory[55 + 4] = 0xE0;
        /* 'C' */ res.memory[60] = 0xF0; res.memory[60 + 1] = 0x80; res.memory[60 + 2] = 0x80; res.memory[60 + 3] = 0x80; res.memory[60 + 4] = 0xF0;
        /* 'D' */ res.memory[65] = 0xE0; res.memory[65 + 1] = 0x80; res.memory[65 + 2] = 0x80; res.memory[65 + 3] = 0x80; res.memory[65 + 4] = 0xE0;
        /* 'E' */ res.memory[70] = 0xF0; res.memory[70 + 1] = 0x80; res.memory[70 + 2] = 0xF0; res.memory[70 + 3] = 0x80; res.memory[70 + 4] = 0xF0;
        /* 'F' */ res.memory[75] = 0xF0; res.memory[75 + 1] = 0x80; res.memory[75 + 2] = 0xF0; res.memory[75 + 3] = 0x80; res.memory[75 + 4] = 0x80;

        res
    }

    pub fn set_instruction(&mut self, start: usize, instr: u16) {
        self.memory[start] = (instr >> 8) as u8;
        self.memory[start + 1] =  instr as u8;
    }

    pub fn load_program(&mut self, program: &Program) {

        for (i,byte) in program.get_binary_data().iter().enumerate() {
            self.memory[0x200 + i] = *byte;

        }
    }
}

pub enum Program {
    Text(String),
    Binary([u8;0xDFF]) //  programs start at 0x200 and ends at 0xFFF
}

impl Program {

    pub fn get_binary_data(&self) -> Vec::<u8> {

        match self {
            Program::Binary(ref data) => data.to_vec(),
            Program::Text(t) => {

                let mut text = t.clone().to_lowercase();
                // remove lines starting with '//' that is comment
                text = text.split('\n').filter(|line| !line.starts_with("//")).collect();

                // replace whitespace with nothing
                text.retain(|c| (c  >= 'a'  && c <= 'f')
                            || (c >= '0' && c <= '9'));

                println!("{:?}", text);

                let t_values: Vec<u8> = text.chars().map(|c| {
                    if c  >= 'a'  && c <= 'f' {
                        return (c as u8) - 87;
                    }

                    (c as u8) - 48
                }).collect();

                let mut res = Vec::new();
                for i in (0..t_values.len()).step_by(2) {

                    let upper = t_values[i] << 4;
                    let val = upper + t_values[i + 1];

                    res.push(val);
                }
                res
            },
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_from_text() {
        let input = "60FF F015 6000 6900 6E00 6000 3001 3000 1392 7E01 6001 4001 4000 1392 7E01 6101 6200 5020 5010 1392 7E01";

        let program = Program::Text(input.to_string());

        let binary = program.get_binary_data();

        let expected = [0x60, 0xFF, 0xF0, 0x15, 0x60, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x60, 0x00, 0x30, 0x01, 0x30, 0x00, 0x13, 0x92, 0x7E, 0x01, 0x60, 0x01, 0x40, 0x01, 0x40, 0x00, 0x13, 0x92, 0x7E, 0x01, 0x61, 0x01, 0x62, 0x00, 0x50, 0x20, 0x50, 0x10, 0x13, 0x92, 0x7E, 0x01];

        assert_eq!(binary, expected);
    }

    #[test]
    fn program_from_text_comments() {
        let input = "60FF F015 6000 6900 6E00 6000 3001\n//hello i am comment\n3000 1392 7E01 6001 4001 4000 1392 7E01 6101 6200 5020 5010 1392 7E01";

        let program = Program::Text(input.to_string());

        let binary = program.get_binary_data();

        let expected = [0x60, 0xFF, 0xF0, 0x15, 0x60, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x60, 0x00, 0x30, 0x01, 0x30, 0x00, 0x13, 0x92, 0x7E, 0x01, 0x60, 0x01, 0x40, 0x01, 0x40, 0x00, 0x13, 0x92, 0x7E, 0x01, 0x61, 0x01, 0x62, 0x00, 0x50, 0x20, 0x50, 0x10, 0x13, 0x92, 0x7E, 0x01];

        assert_eq!(binary, expected);
    }
}
