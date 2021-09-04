use rand;

use crate::constants::c::*;

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
    pub fn new(key: Vec<u8>, rounds: i32) -> Self {
        let ops = vec![
            SHIFT_MAP_L,
            SHIFT_MAP_R,
            SHIFT_FNC_L,
            SHIFT_FNC_R,
            SHIFT_MAG_L,
            SHIFT_MAG_R,
            SHIFT_KEY_L,
            SHIFT_KEY_R,
            
            ROT_KEY_L,
            ROT_KEY_R,

            XOR_KEY,

            INC0,
            INC1,
            INC2,
            INC3,
            INC4,
            INC5,
            INC6,
            INC7,
            INC8,
            INC9,
            INC10,

            DEC0,
            DEC1,
            DEC2,
            DEC3,
            DEC4,
            DEC5,
            DEC6,
            DEC7,
            DEC8,
            DEC9,
            DEC10,
        ];
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
                SHIFT_MAP_L => {
                    self.mappings.rotate_right(
                        self.magnitudes[0] as usize % self.functions.len()
                    )
                }
                SHIFT_MAP_R => {
                    self.mappings.rotate_left(
                        (self.magnitudes[1] % FSIZE) as usize
                    )
                }
                // SHIFTING functions
                SHIFT_FNC_L => {
                    self.functions.rotate_right(
                        (self.magnitudes[2] % FSIZE ) as usize
                    )
                }
                SHIFT_FNC_R => {
                    self.functions.rotate_left(
                        (self.magnitudes[3] % FSIZE ) as usize
                    )
                }
                // ROTATING key bits
                ROT_KEY_L => {
                    for i in 0..self.key.len() {
                        self.key[i] = self.key[i].rotate_right({
                            let a = self.magnitudes[4] as u32;
                            print!("{}", a);
                            a
                        });
                    }
                }
                ROT_KEY_R => {
                    for i in 0..self.key.len() {
                        self.key[i] = self.key[i].rotate_left({
                            let a = self.magnitudes[5] as u32;
                            print!("{}", a);
                            a
                        });
                    }
                }
                // XORING key bits
                XOR_KEY => {
                    for i in 0..self.key.len() {
                        self.key[i] ^= self.magnitudes[6];
                    }
                }
                // SHIFTING magnitudes
                SHIFT_MAG_L => {
                    let v = self.magnitudes[7] % self.magnitudes.len() as u8;
                    self.magnitudes.rotate_right(
                        v as usize
                    )
                }
                SHIFT_MAG_R => {
                    let v = self.magnitudes[8] % self.magnitudes.len() as u8;
                    self.magnitudes.rotate_left(
                        v as usize
                    )
                }
                // SHIFTING key
                SHIFT_KEY_L => {
                    self.key.rotate_right(
                        self.magnitudes[9] as usize % KSIZE
                    )
                }
                SHIFT_KEY_R => {
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