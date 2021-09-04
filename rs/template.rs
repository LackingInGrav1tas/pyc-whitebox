use std::io::Read;

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

fn main() {
    let mut functions: Vec<u8> = /**/;
    let mut key: Vec<u8> = /**/;
    let mut magnitudes: Vec<u8> = /**/;
    let mut mappings: Vec<u8> = /**/;

    for op in opcode {
        match op {
            /*opcode*/
        }
    }
}