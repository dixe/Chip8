pub struct Display {
    pixels: [bool; 64*32]
}



impl Display {

    pub fn new() -> Self {
        Self {
            pixels: [false; 64*32]
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixels.iter_mut() {
            *p = false;
        }
    }


    pub fn draw_sprite(&mut self, sprite: &Sprite) -> u8 {

        let mut flipped = false;
        for row in 0..(sprite.length as usize) {
            for (col, pixel) in sprite.data[row].bits().iter().enumerate() {

                let x = (sprite.x + col) % 64;
                let y = (sprite.y + row) % 64;
                //println!("({}, {}) = {}", x,y, self.pixels[y * 64 + x] ^ *pixel);
                let old_pixel = self.pixels[y * 64 + x] ;
                self.pixels[y * 64 + x] ^= *pixel;

                // if any is on then not on we set flipped (v_f)
                flipped |= old_pixel  && !self.pixels[y * 64 + x]
            }
        }

        flipped as u8
    }


    pub fn read_pixels(&self) ->  &[bool; 64*32] {
        &self.pixels
    }
}


trait Bits {
    fn bits(&self) -> [bool; 8];
}


impl Bits for u8 {
    fn bits(&self) -> [bool;8] {

        [ self & 0b10000000 != 0,
          self & 0b01000000 != 0,
          self & 0b00100000 != 0,
          self & 0b00010000 != 0,
          self & 0b00001000 != 0,
          self & 0b00000100 != 0,
          self & 0b00000010 != 0,
          self & 0b00000001 != 0,
        ]
    }
}

pub struct Sprite {
    pub data: [u8; 15],
    pub length: u8,
    pub x: usize,
    pub y: usize
}
