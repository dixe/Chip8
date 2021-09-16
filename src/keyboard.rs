pub struct Keyboard {
    data: [bool; 16]
}

impl Keyboard {

    pub fn new() -> Self {

        Self {
            data: [false; 16]
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
}
