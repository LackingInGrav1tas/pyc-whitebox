use rand;

pub struct VM<'a> {
    pub functions: Vec<&'a str>,
    pub mappings: Vec<u8>,
    pub magnitudes: Vec<u8>,
    pub opcode: Vec<u8>,
    pub key: Vec<u8>,
    pc: i32,
}

impl VM<'_> {
    pub fn new(key: Vec<u8>, rounds: i32) -> Self {
        let ops = vec![
            "shift_n(mappings, functions.size(), magnitudes[0] % functions.size(), LEFT);",
            "shift_n(mappings, functions.size(), magnitudes[1] % functions.size(), RIGHT);",
            "shift_n(functions.data(), functions.size(), magnitudes[2] % functions.size(), LEFT);",
            "shift_n(functions.data(), functions.size(), magnitudes[3] % functions.size(), RIGHT);",
            "magnitudes[0]++;",
            "magnitudes[1]++;",
            "magnitudes[2]++;",
            "magnitudes[3]++;",
            "magnitudes[0]--;",
            "magnitudes[1]--;",
            "magnitudes[2]--;",
            "magnitudes[3]--;",
    
            "for (int i = 0; i < (sizeof(key) / sizeof(unsigned char)); i++) { bit_rot_n(key[i], magnitudes[4], LEFT); }",
            "for (int i = 0; i < (sizeof(key) / sizeof(unsigned char)); i++) { bit_rot_n(key[i], magnitudes[5], RIGHT); }",
            "magnitudes[4]++;",
            "magnitudes[5]++;",
            "magnitudes[4]--;",
            "magnitudes[5]--;",
    
            "for (int i = 0; i < (sizeof(key) / sizeof(unsigned char)); i++) { key[i] ^= magnitudes[6]; }",
            "magnitudes[6]++;",
            "magnitudes[6]--;",
    
            "shift_n(magnitudes, (sizeof(magnitudes)/sizeof(int)), magnitudes[7] % (sizeof(magnitudes)/sizeof(int)), LEFT);",
            "shift_n(magnitudes, (sizeof(magnitudes)/sizeof(int)), magnitudes[8] % (sizeof(magnitudes)/sizeof(int)), RIGHT);",
            "magnitudes[7]++;",
            "magnitudes[7]--;",
            "magnitudes[8]++;",
            "magnitudes[8]--;",
    
            "shift_n(key, (sizeof(key) / sizeof(unsigned char)), magnitudes[9] % (sizeof(key) / sizeof(unsigned char)), LEFT);",
            "shift_n(key, (sizeof(key) / sizeof(unsigned char)), magnitudes[10] % (sizeof(key) / sizeof(unsigned char)), RIGHT);",
            "magnitudes[9]++;",
            "magnitudes[9]--;",
            "magnitudes[10]++;",
            "magnitudes[10]--;",
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
                "shift_n(mappings, functions.size(), magnitudes[0] % functions.size(), LEFT);" => {
                    self.mappings.rotate_right(
                        self.magnitudes[0] as usize % self.functions.len()
                    )
                }
                "shift_n(mappings, functions.size(), magnitudes[1] % functions.size(), RIGHT);" => {
                    self.mappings.rotate_left(
                        (self.magnitudes[1] % FSIZE) as usize
                    )
                }
                // SHIFTING functions
                "shift_n(functions.data(), functions.size(), magnitudes[2] % functions.size(), LEFT);" => {
                    self.functions.rotate_right(
                        (self.magnitudes[2] % FSIZE ) as usize
                    )
                }
                "shift_n(functions.data(), functions.size(), magnitudes[3] % functions.size(), RIGHT);" => {
                    self.functions.rotate_left(
                        (self.magnitudes[3] % FSIZE ) as usize
                    )
                }
                // ROTATING key bits
                "for (int i = 0; i < (sizeof(key) / sizeof(unsigned char)); i++) { bit_rot_n(key[i], magnitudes[4], LEFT); }" => {
                    for i in 0..self.key.len() {
                        self.key[i] = self.key[i].rotate_right(self.magnitudes[4] as u32);
                    }
                }
                "for (int i = 0; i < (sizeof(key) / sizeof(unsigned char)); i++) { bit_rot_n(key[i], magnitudes[5], RIGHT); }" => {
                    for i in 0..self.key.len() {
                        self.key[i] = self.key[i].rotate_left(self.magnitudes[5] as u32);
                    }
                }
                // XORING key bits
                "for (int i = 0; i < (sizeof(key) / sizeof(unsigned char)); i++) { key[i] ^= magnitudes[6]; }" => {
                    for i in 0..self.key.len() {
                        self.key[i] ^= self.magnitudes[6];
                    }
                }
                // SHIFTING magnitudes
                "shift_n(magnitudes, (sizeof(magnitudes)/sizeof(int)), magnitudes[7] % (sizeof(magnitudes)/sizeof(int)), LEFT);" => {
                    let v = self.magnitudes[7] % self.magnitudes.len() as u8;
                    self.magnitudes.rotate_right(
                        v as usize
                    )
                }
                "shift_n(magnitudes, (sizeof(magnitudes)/sizeof(int)), magnitudes[8] % (sizeof(magnitudes)/sizeof(int)), RIGHT);" => {
                    let v = self.magnitudes[8] % self.magnitudes.len() as u8;
                    self.magnitudes.rotate_left(
                        v as usize
                    )
                }
                // SHIFTING key
                "shift_n(key, (sizeof(key) / sizeof(unsigned char)), magnitudes[9] % (sizeof(key) / sizeof(unsigned char)), LEFT);" => {
                    self.key.rotate_right(
                        self.magnitudes[9] as usize % KSIZE
                    )
                }
                "shift_n(key, (sizeof(key) / sizeof(unsigned char)), magnitudes[10] % (sizeof(key) / sizeof(unsigned char)), RIGHT);" => {
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
                    self.call(self.functions[i]);
                }
            }
            self.pc -= 1;
        }
    }
}