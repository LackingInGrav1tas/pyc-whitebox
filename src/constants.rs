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

#[allow(unused_variables)]
pub mod rust {

}

pub mod errors {
    pub const LANG_ERROR: &str = "the 'language' parameter in whitebox-settings.json should be either C++ or Rust";
}