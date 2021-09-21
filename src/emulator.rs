use crate::instructions;
use crate::instructions::{Instruction};
use crate::chip::*;
use crate::display::Sprite;
use rand::Rng;
use sdl2::{Sdl};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

pub struct Emulator {
    chip: Chip8,
    sdl_context: Sdl,
    frequency: u32,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Emulator {
    pub fn new() -> Self {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // square pixel
        let width = 832;  // 64 * 13
        let height = 416; // 32 * 13
        let window = video_subsystem.window("Chip8", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        Self {
            chip: Chip8::new(),
            sdl_context,
            canvas,
            frequency: 800,
        }
    }

    pub fn load_program(&mut self, program: &Program) {

        self.chip.load_program(program);

    }

    pub fn run(&mut self) {

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut clock_last_instant = Instant::now();
        let clock_rate_millis =  (1.0/(self.frequency as f64) * 1000.0) as u128;

        let mut delay_sound_last_instant = Instant::now();
        let delay_sound_rate_millis = (1.0/60.0 * 1000.0) as u128;

        loop {

            // run a cycle and update display
            let elapsed_clock = clock_last_instant.elapsed();
            if elapsed_clock.as_millis() > clock_rate_millis {
                clock_last_instant = Instant::now();
                let redraw = cycle(&mut self.chip);
                if redraw {
                    self.update_display();
                }
            }

            // update keyboard

            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit {..} => return,
                    Event::KeyDown { keycode: Some(code), ..} => {
                        self.chip.keyboard.check_key(code, true);
                    }
                    Event::KeyUp { keycode: Some(code), ..} => {
                        self.chip.keyboard.check_key(code, false);
                    },
                    _ => {}
                };
            }



            // update timers (delay and sound)
            let elapsed_delay_sound = delay_sound_last_instant.elapsed();

            if elapsed_delay_sound.as_millis() > delay_sound_rate_millis {
                delay_sound_last_instant = Instant::now();
                self.chip.registers.tick();
            }

            // if sound is 1 play a tone we specify

        }
    }


    fn update_display(&mut self) {


        // UPDATE THE DISPLAY
        // set on "pixel" color
        self.canvas.set_draw_color(Color::RGB(255, 210, 0));



        for (i,pixel) in self.chip.display.read_pixels().iter().enumerate() {

            if *pixel {
                self.canvas.set_draw_color(Color::RGB(255, 210, 0));
            }
            else {
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }

            let x = (i % 64) as i32;
            let y = (i / 64) as i32;

            //calc pixel size from canvas resolution

            let width = (self.canvas.window().size().0  / 64) as i32;
            let height = (self.canvas.window().size().1 / 32)  as i32;

            self.canvas.fill_rect(Rect::new(x * width, y * height, width as u32, height as u32));

        }

        // set default to black

        self.canvas.present();
    }
}



fn cycle(chip: &mut Chip8) -> bool {


    let upper = chip.memory[chip.pc as usize];
    let lower = chip.memory[(chip.pc + 1) as usize];


    let instr = instructions::parse(upper, lower);

    let mut redraw = false;

    match execute(instr, chip, &mut redraw) {
        ExecuteRes::SetPc(addr) => {
            chip.pc = addr;
        },
        ExecuteRes::Wait => {}
    };

    redraw

}


enum ExecuteRes {
    SetPc(u16),
    Wait
}


fn execute(instr: Instruction, chip: &mut Chip8, redraw: &mut bool ) -> ExecuteRes {
    use ExecuteRes::*;
    let mut new_pc = chip.pc + 2;
    match instr {
        Instruction::Cls => {
            *redraw = true;
            chip.display.clear();
            SetPc(new_pc)
        },
        Instruction::Ret => {
            chip.sp -= 1;
            new_pc = chip.stack[chip.sp as usize];
            SetPc(new_pc)
        },
        Instruction::Jump(addr) => {
            new_pc = addr;
            SetPc(new_pc)
        },
        Instruction::Call(addr) => {

            chip.stack[chip.sp as usize] = new_pc;
            chip.sp += 1;
            new_pc = addr;
            SetPc(new_pc)
        },
        Instruction::SkipEqConst(reg, byte) => {

            // get reg value by reg
            let reg_val = chip.registers.get_value(reg);
            if reg_val == byte {
                new_pc += 2;
            }

            SetPc(new_pc)
        },
        Instruction::SkipNotEqConst(reg, byte) => {

            // get reg value by reg
            let reg_val = chip.registers.get_value(reg);
            if reg_val != byte {
                new_pc += 2;
            }

            SetPc(new_pc)
        },
        Instruction::SkipEqReg(reg_x, reg_y) => {

            // get reg value by reg
            let x_val = chip.registers.get_value(reg_x);
            let y_val = chip.registers.get_value(reg_y);

            if x_val == y_val {
                new_pc += 2;
            }
            SetPc(new_pc)
        },

        Instruction::LoadConst(reg, byte) => {
            chip.registers.set_value(reg, byte);
            SetPc(new_pc)
        },
        Instruction::AddConst(reg, byte) => {
            let cur = chip.registers.get_value(reg) as u16;
            let res = (byte as u16 + cur) as u8;
            //println!("cur={:?} byte ={}, res={}", cur, byte, res);
            chip.registers.set_value(reg, res);
            SetPc(new_pc)
        },

        Instruction::LoadReg(reg_x, reg_y) => {
            let y_val = chip.registers.get_value(reg_y);
            chip.registers.set_value(reg_x, y_val);
            SetPc(new_pc)
        },

        Instruction::Or(reg_x, reg_y) => {
            chip.registers.bitwise(reg_x, reg_y, |x,y| x | y);
            SetPc(new_pc)
        },

        Instruction::And(reg_x, reg_y) => {
            chip.registers.bitwise(reg_x, reg_y, |x,y| x & y);
            SetPc(new_pc)
        },

        Instruction::Xor(reg_x, reg_y) => {
            chip.registers.bitwise(reg_x, reg_y, |x,y| x ^ y);
            SetPc(new_pc)
        },

        Instruction::Add(reg_x, reg_y) => {
            let x = chip.registers.get_value(reg_x);
            let y = chip.registers.get_value(reg_y);

            let val = (x  as u16) + (y as u16);
            // CHeck val > 255 then set v_f


            if val > 255 {
                chip.registers.set_value(0xF, 1);
            }
            else {
                chip.registers.set_value(0xF, 0);
            }

            chip.registers.set_value(reg_x, val as u8);
            SetPc(new_pc)
        },


        Instruction::Sub(reg_x, reg_y) => {
            let mut x = chip.registers.get_value(reg_x) as u16;
            let y = chip.registers.get_value(reg_y) as u16;


            // CHeck val > 255 then set v_f

            if x > y {
                chip.registers.set_value(0xF, 1);
            }
            else {
                chip.registers.set_value(0xF, 0);
                x += 256;
            }

            let val = x - y;
            chip.registers.set_value(reg_x, val as u8);
            SetPc(new_pc)
        },

        Instruction::ShiftRight(reg_x, reg_y) => {
            let mut x = chip.registers.get_value(reg_x);

            chip.registers.set_value(0xF, x & 1);

            chip.registers.set_value(reg_x, x >> 1);
            SetPc(new_pc)
        },

        Instruction::SkipEqReg(reg_x, reg_y) => {
            let x = chip.registers.get_value(reg_x);
            let y = chip.registers.get_value(reg_y);

            if x == y {
                new_pc += 2;
            }
            SetPc(new_pc)
        },

        Instruction::SubN(reg_x, reg_y) => {
            let x = chip.registers.get_value(reg_x) as u16;
            let mut y = chip.registers.get_value(reg_y) as u16;


            // CHeck val > 255 then set v_f

            if y > x {
                chip.registers.set_value(0xF, 1);
            }
            else {
                chip.registers.set_value(0xF, 0);
                y += 256;
            }

            let val = y - x;
            chip.registers.set_value(reg_x, val as u8);
            SetPc(new_pc)
        },

        Instruction::ShiftLeft(reg_x, reg_y) => {
            let x = chip.registers.get_value(reg_x);

            chip.registers.set_value(0xF, (x & 0x80) >> 7);

            chip.registers.set_value(reg_x, x << 1);
            SetPc(new_pc)
        },

        Instruction::SkipNotEqReg(reg_x, reg_y) => {
            let x = chip.registers.get_value(reg_x);
            let y = chip.registers.get_value(reg_y);

            if x != y {
                new_pc = new_pc + 2;
            }

            SetPc(new_pc)
        },

        Instruction::LoadAddr(addr) => {
            chip.registers.set_i(addr);
            SetPc(new_pc)
        },

        Instruction::JumpOffset(addr) => {

            let v0 = chip.registers.get_value(0) as u16;

            new_pc = v0 + addr;
            SetPc(new_pc)
        },

        Instruction::Rand(reg_x, data) => {

            let mut rng = rand::thread_rng();
            let val = data & rng.gen_range(0..=255);

            chip.registers.set_value(reg_x, val);

            SetPc(new_pc)
        },

        Instruction::Draw(reg_x, reg_y, n) => {
            // get the data and send that to the display to draw


            let x = chip.registers.get_value(reg_x) as usize;
            let y = chip.registers.get_value(reg_y) as usize;

            let mut sprite = Sprite {
                data: [0; 15],
                length: n,
                x,
                y
            };


            for i in 0..(n as usize) {
                //println!("{:?}, {}, {}, {}",reg_x, reg_y, n, chip.registers.get_i() as usize + i);
                let addr = (chip.registers.get_i() as usize + i);
                sprite.data[i] = chip.memory[addr];
            }

            *redraw = true;

            let vf = chip.display.draw_sprite(&sprite);

            chip.registers.set_value(0xf, vf);

            SetPc(new_pc)

        },


        Instruction::SkipOnKeyPressed(reg_x) => {

            let x = chip.registers.get_value(reg_x);
            if chip.keyboard.key_pressed(x) {
                new_pc += 2;
            }

            SetPc(new_pc)
        },

        Instruction::SkipKeyNotPressed(reg_x) => {
            let x = chip.registers.get_value(reg_x);
            if !chip.keyboard.key_pressed(x) {
                new_pc += 2;
            }
            SetPc(new_pc)
        },

        Instruction::LoadDelay(reg_x) => {
            chip.registers.set_value(reg_x, chip.registers.get_delay() );
            SetPc(new_pc)
        },

        Instruction::WaitKeyPress(reg_x) =>
            match chip.keyboard.next_key() {
                None => Wait,
                Some(key) => {
                    chip.registers.set_value(reg_x, key);
                    SetPc(new_pc)
                }
            },


        Instruction::SetDelay(reg_x) => {
            let x = chip.registers.get_value(reg_x);
            chip.registers.set_delay(x);
            SetPc(new_pc)
        },

        Instruction::SetSound(reg_x) => {
            let x = chip.registers.get_value(reg_x);
            chip.registers.set_sound(x);
            SetPc(new_pc)
        },

        Instruction::AddAddr(reg_x) => {
            let x = chip.registers.get_value(reg_x);
            chip.registers.increment_i(x as u16);
            SetPc(new_pc)
        },

        Instruction::SetSpriteAddr(reg_x) => {
            let x = chip.registers.get_value(reg_x);
            chip.registers.set_i((x*5) as u16);

            SetPc(new_pc)
        },

        Instruction::BCD(reg_x) => {
            let x = chip.registers.get_value(reg_x);

            let i_0 = x / 100;
            let i_1 = (x % 100) / 10;
            let i_2 = x % 10;

            let addr = chip.registers.get_i() as usize;

            chip.memory[addr] = i_0;
            chip.memory[addr + 1] = i_1;
            chip.memory[addr + 2] = i_2;

            SetPc(new_pc)
        },

        Instruction::Store(reg_x) => {

            let addr = chip.registers.get_i() as usize;
            for i in 0..=reg_x {
                chip.memory[addr + i as usize] = chip.registers.get_value(i);
            }

            SetPc(new_pc)
        },

        Instruction::Load(reg_x) => {

            let addr = chip.registers.get_i() as usize;
            for i in 0..=reg_x {
                chip.registers.set_value(i, chip.memory[addr + i as usize]);
            }
            SetPc(new_pc)
        },
    }
}



#[cfg(test)]
mod tests {

    use crate::chip::*;
    use super::*;


    #[test]
    fn sub_underflow() {

        let mut chip = Chip8::new();

        // set register 1 to 1
        chip.registers.set_value(1,1);


        chip.set_instruction(0x200, 0x8015);
        cycle(&mut chip);

        // check that register 0 has the value of 255, since that is 0 - 1 with borrow
        assert_eq!(chip.registers.get_value(0), 255);
        assert_eq!(chip.registers.get_value(0xF), 0);
    }


    #[test]
    fn add_overflow() {

        let mut chip = Chip8::new();

        // set register 0 and 1 to 200
        chip.registers.set_value(0,200);
        chip.registers.set_value(1,200);

        chip.set_instruction(0x200, 0x8014);

        cycle(&mut chip);

        assert_eq!(chip.registers.get_value(0), 0x90);
        assert_eq!(chip.registers.get_value(0xF), 1);




    }
}
