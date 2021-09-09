use std::io::Read;

fn increment(n: & mut u8) {
    if *n == 255 {
        *n = 0;
    } else {
        *n += 1;
    }
}

fn decrement(n: & mut u8) {
    if *n == 0 {
        *n = 255;
    } else {
        *n -= 1;
    }
}

fn p(n: u8) -> String {
    return String::from(match n {
        0 => { "L rotate map        " }
        1 => { "R rotate map        " }
        2 => { "L rotate functions  " }
        3 => { "R rotate functions  " }
        4 => { "L rotate magnitudes " }
        5 => { "R rotate magnitudes " }
        6 => { "L rotate key        " }
        7 => { "R rotate key        " }
        8 => { "L bit rotate key    " }
        9 => { "R bit rotate key    " }
        10 => { "XOR key             " }
        11 => { "0++                 " }
        12 => { "1++                 " }
        13 => { "2++                 " }
        14 => { "3++                 " }
        15 => { "4++                 " }
        16 => { "5++                 " }
        17 => { "6++                 " }
        18 => { "7++                 " }
        19 => { "8++                 " }
        20 => { "9++                 " }
        21 => { "10++                " }
        22 => { "0--                 " }
        23 => { "1--                 " }
        24 => { "2--                 " }
        25 => { "3--                 " }
        26 => { "4--                 " }
        27 => { "5--                 " }
        28 => { "6--                 " }
        29 => { "7--                 " }
        30 => { "8--                 " }
        31 => { "9--                 " }
        32 => { "10--                " }
        _ => unreachable!()
    })
}

#[allow(non_snake_case)]
fn _STREAM(key: Vec<u8>, txt: Vec<u8>) -> Vec<u8> {
    if key.len() != txt.len() {
        print!("key size ({}) != text size ({})\npress enter for more info...", key.len(), txt.len());
        let mut _buf = String::new();
        std::io::stdin().read_line(& mut _buf).unwrap();
        print!("\n{:?}\n\n{:?}", key, txt);
        panic!();
    }
    let mut v = vec![];
    for (a, b) in key.iter().zip(txt.iter()) {
        v.push(a ^ b);
    }
    v
}

struct VM {
    functions: Vec<u8>,
    key: Vec<u8>,
    magnitudes: Vec<u8>,
    mappings: Vec<u8>,
    opcode: Vec<u8>,
    ksize: usize,
    fsize: usize
}

impl VM {
    fn new() -> Self {
        let k = /*key*/;
        let f = /*functions-rep*/;
        Self {
            functions: f.clone(),
            fsize: f.len(),
            key: k.clone(),
            magnitudes: /*magnitudes*/,
            mappings: /*mappings*/,
            opcode: /*opcode*/,
            ksize: k.len(),
        }
    }

    fn op(& mut self, op: u8) {
        match op {
            0 => { self.mappings.rotate_left(self.magnitudes[0] as usize % self.functions.len()); }
            1 => { self.mappings.rotate_right(self.magnitudes[1] as usize % self.functions.len()) }
            2 => { self.functions.rotate_left(self.magnitudes[2] as usize % self.fsize); }
            3 => { self.functions.rotate_right(self.magnitudes[3] as usize % self.fsize); }
            4 => { let t = self.magnitudes[7] as usize % self.magnitudes.len(); self.magnitudes.rotate_left(t) }
            5 => { let t = self.magnitudes[8] as usize % self.magnitudes.len(); self.magnitudes.rotate_right(t) }
            6 => { self.key.rotate_left(self.magnitudes[9] as usize % self.ksize) }
            7 => { self.key.rotate_right(self.magnitudes[10] as usize % self.ksize) }
            8 => { for i in 0..self.key.len() { self.key[i] = self.key[i].rotate_left(self.magnitudes[4] as u32); } }
            9 => { for i in 0..self.key.len() { self.key[i] = self.key[i].rotate_right(self.magnitudes[5] as u32); } }
            10 => { for i in 0..self.key.len() { self.key[i] ^= self.magnitudes[6]; } }
            11 => { increment(& mut self.magnitudes[0]); }
            12 => { increment(& mut self.magnitudes[1]); }
            13 => { increment(& mut self.magnitudes[2]); }
            14 => { increment(& mut self.magnitudes[3]); }
            15 => { increment(& mut self.magnitudes[4]); }
            16 => { increment(& mut self.magnitudes[5]); }
            17 => { increment(& mut self.magnitudes[6]); }
            18 => { increment(& mut self.magnitudes[7]); }
            19 => { increment(& mut self.magnitudes[8]); }
            20 => { increment(& mut self.magnitudes[9]); }
            21 => { increment(& mut self.magnitudes[10]); }
            22 => { decrement(& mut self.magnitudes[0]); }
            23 => { decrement(& mut self.magnitudes[1]); }
            24 => { decrement(& mut self.magnitudes[2]); }
            25 => { decrement(& mut self.magnitudes[3]); }
            26 => { decrement(& mut self.magnitudes[4]); }
            27 => { decrement(& mut self.magnitudes[5]); }
            28 => { decrement(& mut self.magnitudes[6]); }
            29 => { decrement(& mut self.magnitudes[7]); }
            30 => { decrement(& mut self.magnitudes[8]); }
            31 => { decrement(& mut self.magnitudes[9]); }
            32 => { decrement(& mut self.magnitudes[10]); }
            _ => unreachable!()
        }
    }

    fn run(& mut self) -> Result<Vec<u8>, ()> {
        println!("START: {:?}, {:?}", self.key, self.opcode);

        for pc in 0..self.opcode.len() {
            for i in 0..self.mappings.len() {
                if self.opcode[pc] == self.mappings[i] {
                    self.op(self.functions[i]);
                    // println!("{} -- {:?}", self.functions[i], self.key);
                    println!("{} |||  {:?} -- {:?}", &p(self.functions[i]), self.magnitudes, self.key);
                    break;
                }
            }
        }

        Ok(self.key.clone())
    }
}

fn main() {
    let key = VM::new().run().unwrap();
    print!("{:?}", key);
}