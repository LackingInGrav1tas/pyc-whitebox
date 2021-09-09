#[allow(unused_variables)]
pub mod c {
    pub const SHIFT_MAP_L: &str = "shift_n(mappings, magnitudes[0] % functions.size(), LEFT);";
    pub const SHIFT_MAP_R: &str = "shift_n(mappings, magnitudes[1] % functions.size(), RIGHT);";
    pub const SHIFT_FNC_L: &str = "shift_n(functions, magnitudes[2] % functions.size(), LEFT);";
    pub const SHIFT_FNC_R: &str = "shift_n(functions, magnitudes[3] % functions.size(), RIGHT);";
    pub const SHIFT_MAG_L: &str = "shift_n(magnitudes, magnitudes[7] % magnitudes.size(), LEFT);";
    pub const SHIFT_MAG_R: &str = "shift_n(magnitudes, magnitudes[8] % magnitudes.size(), RIGHT);";
    pub const SHIFT_KEY_L: &str = "shift_n(key, magnitudes[9] % key.size(), LEFT);";
    pub const SHIFT_KEY_R: &str = "shift_n(key, magnitudes[10] % key.size(), RIGHT);";
    pub const ROT_KEY_L: &str = "for (int i = 0; i < key.size(); i++) { bit_rot_n(key[i], magnitudes[4], LEFT); }";
    pub const ROT_KEY_R: &str = "for (int i = 0; i < key.size(); i++) { bit_rot_n(key[i], magnitudes[5], RIGHT); }";
    pub const XOR_KEY: &str = "for (int i = 0; i < key.size(); i++) { key[i] ^= magnitudes[6]; }";

    pub const INC0: &str = "magnitudes[0]++;";
    pub const INC1: &str = "magnitudes[1]++;";
    pub const INC2: &str = "magnitudes[2]++;";
    pub const INC3: &str = "magnitudes[3]++;";
    pub const INC4: &str = "magnitudes[4]++;";
    pub const INC5: &str = "magnitudes[5]++;";
    pub const INC6: &str = "magnitudes[6]++;";
    pub const INC7: &str = "magnitudes[7]++;";
    pub const INC8: &str = "magnitudes[8]++;";
    pub const INC9: &str = "magnitudes[9]++;";
    pub const INC10: &str = "magnitudes[10]++;";

    pub const DEC0: &str = "magnitudes[0]--;";
    pub const DEC1: &str = "magnitudes[1]--;";
    pub const DEC2: &str = "magnitudes[2]--;";
    pub const DEC3: &str = "magnitudes[3]--;";
    pub const DEC4: &str = "magnitudes[4]--;";
    pub const DEC5: &str = "magnitudes[5]--;";
    pub const DEC6: &str = "magnitudes[6]--;";
    pub const DEC7: &str = "magnitudes[7]--;";
    pub const DEC8: &str = "magnitudes[8]--;";
    pub const DEC9: &str = "magnitudes[9]--;";
    pub const DEC10: &str = "magnitudes[10]--;";
}

#[allow(unused_variables, dead_code)]
pub mod rust {
    pub const SHIFT_MAP_L: &str = "self.mappings.rotate_left(self.magnitudes[0] as usize % self.functions.len());";
    pub const SHIFT_MAP_R: &str = "self.mappings.rotate_right(self.magnitudes[1] as usize % self.functions.len())";
    pub const SHIFT_FNC_L: &str = "self.functions.rotate_left(self.magnitudes[2] as usize % self.mappings.len());";
    pub const SHIFT_FNC_R: &str = "self.functions.rotate_right(self.magnitudes[3] as usize % self.mappings.len());";
    pub const SHIFT_MAG_L: &str = "let t = self.magnitudes[7] as usize % self.magnitudes.len(); self.magnitudes.rotate_left(t)";
    pub const SHIFT_MAG_R: &str = "let t = self.magnitudes[8] as usize % self.magnitudes.len(); self.magnitudes.rotate_right(t)";
    pub const SHIFT_KEY_L: &str = "self.key.rotate_left(self.magnitudes[9] as usize % self.ksize)";
    pub const SHIFT_KEY_R: &str = "self.key.rotate_right(self.magnitudes[10] as usize % self.ksize)";
    pub const ROT_KEY_L: &str = "for i in 0..self.key.len() { self.key[i] = self.key[i].rotate_left(self.magnitudes[4] as u32); }";
    pub const ROT_KEY_R: &str = "for i in 0..self.key.len() { self.key[i] = self.key[i].rotate_right(self.magnitudes[5] as u32); }";
    pub const XOR_KEY: &str = "for i in 0..self.key.len() { self.key[i] ^= self.magnitudes[6]; }";

    pub const INC0: &str = "increment(& mut self.magnitudes[0]);";
    pub const INC1: &str = "increment(& mut self.magnitudes[1]);";
    pub const INC2: &str = "increment(& mut self.magnitudes[2]);";
    pub const INC3: &str = "increment(& mut self.magnitudes[3]);";
    pub const INC4: &str = "increment(& mut self.magnitudes[4]);";
    pub const INC5: &str = "increment(& mut self.magnitudes[5]);";
    pub const INC6: &str = "increment(& mut self.magnitudes[6]);";
    pub const INC7: &str = "increment(& mut self.magnitudes[7]);";
    pub const INC8: &str = "increment(& mut self.magnitudes[8]);";
    pub const INC9: &str = "increment(& mut self.magnitudes[9]);";
    pub const INC10: &str = "increment(& mut self.magnitudes[10]);";

    pub const DEC0: &str = "decrement(& mut self.magnitudes[0]);";
    pub const DEC1: &str = "decrement(& mut self.magnitudes[1]);";
    pub const DEC2: &str = "decrement(& mut self.magnitudes[2]);";
    pub const DEC3: &str = "decrement(& mut self.magnitudes[3]);";
    pub const DEC4: &str = "decrement(& mut self.magnitudes[4]);";
    pub const DEC5: &str = "decrement(& mut self.magnitudes[5]);";
    pub const DEC6: &str = "decrement(& mut self.magnitudes[6]);";
    pub const DEC7: &str = "decrement(& mut self.magnitudes[7]);";
    pub const DEC8: &str = "decrement(& mut self.magnitudes[8]);";
    pub const DEC9: &str = "decrement(& mut self.magnitudes[9]);";
    pub const DEC10: &str = "decrement(& mut self.magnitudes[10]);";

    pub fn to_u8_value(f: &str) -> u8 {
        match f {
            SHIFT_MAP_L => 0,
            SHIFT_MAP_R => 1,
            SHIFT_FNC_L => 2,
            SHIFT_FNC_R => 3,
            SHIFT_MAG_L => 4,
            SHIFT_MAG_R => 5,
            SHIFT_KEY_L => 6,
            SHIFT_KEY_R => 7,

            ROT_KEY_L => 8,
            ROT_KEY_R => 9,

            XOR_KEY => 10,

            INC0 => 11,
            INC1 => 12,
            INC2 => 13,
            INC3 => 14,
            INC4 => 15,
            INC5 => 16,
            INC6 => 17,
            INC7 => 18,
            INC8 => 19,
            INC9 => 20,
            INC10 => 21,

            DEC0 => 22,
            DEC1 => 23,
            DEC2 => 24,
            DEC3 => 25,
            DEC4 => 26,
            DEC5 => 27,
            DEC6 => 28,
            DEC7 => 29,
            DEC8 => 30,
            DEC9 => 31,
            DEC10 => 32,

            _ => panic!("{}", f)
        }
    }
}

pub mod errors {
    pub const LANG_ERROR: &str = "the 'language' parameter in whitebox-settings.json should be either C++ or Rust";
}