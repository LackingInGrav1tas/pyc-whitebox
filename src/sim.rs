use rand;

use crate::constants::{c, rust};

pub struct VM<'a> {
    pub functions: Vec<&'a str>,
    pub mappings: Vec<u8>,
    pub magnitudes: Vec<u8>,
    pub opcode: Vec<u8>,
    pub key: Vec<u8>,
    pc: i32,
}

#[allow(non_snake_case)]
impl VM<'_> {
    pub fn new(key: Vec<u8>, rounds: i32, lang: &str) -> Self {
        let ops = if lang == "C++" {
            vec![
                c::SHIFT_MAP_L,
                c::SHIFT_MAP_R,
                c::SHIFT_FNC_L,
                c::SHIFT_FNC_R,
                c::SHIFT_MAG_L,
                c::SHIFT_MAG_R,
                c::SHIFT_KEY_L,
                c::SHIFT_KEY_R,
                
                c::ROT_KEY_L,
                c::ROT_KEY_R,

                c::XOR_KEY,

                c::INC0,
                c::INC1,
                c::INC2,
                c::INC3,
                c::INC4,
                c::INC5,
                c::INC6,
                c::INC7,
                c::INC8,
                c::INC9,
                c::INC10,

                c::DEC0,
                c::DEC1,
                c::DEC2,
                c::DEC3,
                c::DEC4,
                c::DEC5,
                c::DEC6,
                c::DEC7,
                c::DEC8,
                c::DEC9,
                c::DEC10,
            ]
        } else {
            vec![
                rust::SHIFT_MAP_L,
                rust::SHIFT_MAP_R,
                rust::SHIFT_FNC_L,
                rust::SHIFT_FNC_R,
                rust::SHIFT_MAG_L,
                rust::SHIFT_MAG_R,
                rust::SHIFT_KEY_L,
                rust::SHIFT_KEY_R,
                
                rust::ROT_KEY_L,
                rust::ROT_KEY_R,

                rust::XOR_KEY,

                rust::INC0,
                rust::INC1,
                rust::INC2,
                rust::INC3,
                rust::INC4,
                rust::INC5,
                rust::INC6,
                rust::INC7,
                rust::INC8,
                rust::INC9,
                rust::INC10,

                rust::DEC0,
                rust::DEC1,
                rust::DEC2,
                rust::DEC3,
                rust::DEC4,
                rust::DEC5,
                rust::DEC6,
                rust::DEC7,
                rust::DEC8,
                rust::DEC9,
                rust::DEC10,
            ]
        };
        Self {
            functions: ops.clone(),
            mappings: {
                // creating vec
                let mut v = vec![];
                for i in 0..ops.len() {
                    v.push(i as u8);
                }
                // shuffling
                let mut n = vec![];
                while v.len() > 0 {
                    n.push(
                        v.remove(
                            (rand::random::<u8>() % v.len() as u8) as usize
                        )
                    )
                }
                n
            },
            magnitudes: {
                let mut v = vec![];
                for _ in 0..11 {
                    v.push(rand::random::<u8>());
                }
                v
            },
            opcode: {
                let mut v = vec![];
                for _ in 0..rounds {
                    v.push(rand::random::<u8>() % ops.len() as u8);
                }
                v
            },
            key: key,
            pc: rounds - 1,
        }
    }

    fn done(&self) -> bool {
        self.pc == -1
    }

    fn call(& mut self, op: &str) {
        let FSIZE = self.functions.len() as u8;
        let KSIZE = self.key.len();
        if op.contains("++;") {
            // parsing index, then doing opposite
            let n = op.replace("magnitudes[", "")
                .replace("]++;", "")
                .parse::<usize>().unwrap();
            if self.magnitudes[n] == 0 {
                self.magnitudes[n] = 255;
            } else {
                self.magnitudes[n] -= 1;
            }
        } else if op.contains("--;") {
            // parsing index, then doing opposite
            let n = op.replace("magnitudes[", "")
                .replace("]--;", "")
                .parse::<usize>().unwrap();
            if self.magnitudes[n] == 255 {
                self.magnitudes[n] = 0;
            } else {
                self.magnitudes[n] += 1;
            }
        } else {
            match op {
                // SHIFTING mappings
                c::SHIFT_MAP_L | rust::SHIFT_MAP_L => {
                    self.mappings.rotate_right(
                        self.magnitudes[0] as usize % self.functions.len()
                    )
                }
                c::SHIFT_MAP_R | rust::SHIFT_MAP_R => {
                    self.mappings.rotate_left(
                        (self.magnitudes[1] % FSIZE) as usize
                    )
                }
                // SHIFTING functions
                c::SHIFT_FNC_L | rust::SHIFT_FNC_L => {
                    self.functions.rotate_right(
                        (self.magnitudes[2] % FSIZE ) as usize
                    )
                }
                c::SHIFT_FNC_R | rust::SHIFT_FNC_R => {
                    self.functions.rotate_left(
                        (self.magnitudes[3] % FSIZE ) as usize
                    )
                }
                // ROTATING key bits
                c::ROT_KEY_L | rust::ROT_KEY_L => {
                    for i in 0..self.key.len() {
                        self.key[i] = self.key[i].rotate_right({
                            let a = self.magnitudes[4] as u32;
                            print!("{}", a);
                            a
                        });
                    }
                }
                c::ROT_KEY_R | rust::ROT_KEY_R => {
                    for i in 0..self.key.len() {
                        self.key[i] = self.key[i].rotate_left({
                            let a = self.magnitudes[5] as u32;
                            print!("{}", a);
                            a
                        });
                    }
                }
                // XORING key bits
                c::XOR_KEY | rust::XOR_KEY => {
                    for i in 0..self.key.len() {
                        self.key[i] ^= self.magnitudes[6];
                    }
                }
                // SHIFTING magnitudes
                c::SHIFT_MAG_L | rust::SHIFT_MAG_L => {
                    let v = self.magnitudes[7] % self.magnitudes.len() as u8;
                    self.magnitudes.rotate_right(
                        v as usize
                    )
                }
                c::SHIFT_MAG_R | rust::SHIFT_MAG_R => {
                    let v = self.magnitudes[8] % self.magnitudes.len() as u8;
                    self.magnitudes.rotate_left(
                        v as usize
                    )
                }
                // SHIFTING key
                c::SHIFT_KEY_L | rust::SHIFT_KEY_L => {
                    self.key.rotate_right(
                        self.magnitudes[9] as usize % KSIZE
                    )
                }
                c::SHIFT_KEY_R | rust::SHIFT_KEY_R => {
                    self.key.rotate_left(
                        self.magnitudes[10] as usize % KSIZE
                    )
                }

                _ => unimplemented!()
            }
        }
    }

    pub fn generate(& mut self) {
        while !self.done() {
            for i in 0..self.mappings.len() {
                if self.opcode[self.pc as usize] == self.mappings[i] {
                    print!("{}", self.functions[i]);
                    self.call(self.functions[i]);
                    break;
                }
            }
            self.pc -= 1;
            println!(" {:?}", self.key);
        }
    }
}