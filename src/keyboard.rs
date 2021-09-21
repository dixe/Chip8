use sdl2::keyboard::Keycode;

pub struct Keyboard {
    data: [bool; 16],
    mapping: [(Keycode, usize); 16]
}

impl Keyboard {

    pub fn new() -> Self {

        Self {
            data: [false; 16],
            mapping: [
                (Keycode::Kp0, 0),
                (Keycode::Kp1, 1),
                (Keycode::Kp2, 2),
                (Keycode::Kp3, 3),

                (Keycode::Kp4, 4),
                (Keycode::Kp5, 5),
                (Keycode::Kp6, 6),
                (Keycode::Kp7, 7),

                (Keycode::Kp8, 8),
                (Keycode::Kp9, 9),
                (Keycode::A, 10),
                (Keycode::B, 11),

                (Keycode::C, 12),
                (Keycode::D, 13),
                (Keycode::E, 14),
                (Keycode::F, 15),
            ]
        }
    }

    #[inline]
    pub fn key_pressed(&self, key: u8) -> bool {
        self.data[key as usize]
    }

    pub fn next_key(&self) -> Option<u8> {

        for i in 0..16 {
            if self.data[i] {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn check_key(&mut self, code: Keycode, down: bool) {
        for (k,i) in self.mapping.iter() {
            if *k == code {
                self.data[*i] = down;
                return;
            }
        }
    }
}
