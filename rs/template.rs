use std::io::Read;

const KSIZE: usize = /*KSIZE*/;

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
}

impl VM {
    fn new() -> Self {
        let k = /*key*/;
        Self {
            functions: /*functions-rep*/,
            key: k.clone(),
            magnitudes: /*magnitudes*/,
            mappings: /*mappings*/,
            opcode: /*opcode*/,
            ksize: k.len(),
        }
    }

    fn op(& mut self, op: u8) {
        match op {
            /*functions*/
        }
    }

    fn run(& mut self) -> Result<(), ()> {

        for op in &self.opcode {

        }

        Ok(())
    }
}

fn main() {
    
}