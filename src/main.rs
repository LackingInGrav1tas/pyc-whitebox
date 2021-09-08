mod sim;
mod constants;

use sim::VM;
use constants::errors::*;
use constants::rust::to_u8_value;

use json;


use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;

fn vec_to_str(v: Vec<u8>, f: &str) -> String {
    if f == "C++" {
        let mut s = String::from("{");
        for byte in v {
            s += &(byte.to_string() + ",")
        }
        s.pop().expect("");
        s + "}"
    } else {
        let mut s = String::from("vec![");
        for byte in v {
            s += &(byte.to_string() + ",")
        }
        s.pop().expect("");
        s + "]"
    }
}

fn main() {
    println!("getting key...");
    // HANDLING ARGS
    let argv = env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        println!("ERROR - correct format: whitebox-generator.exe <key-file>");
        std::process::exit(1);
    }
    
    let mut file = File::open(argv[1].clone()).unwrap();
    let mut key = vec![];
    file.read_to_end(& mut key).expect(&format!("could not open key file \"{}\"", argv[1]));

    println!("fetching settings...");
    // GETTING SETTINGS
    let settings = json::parse(
        &fs::read_to_string("whitebox-settings.json").expect("could not find whitebox-settings.json")
    ).expect("could not parse json");

    println!("generating opcode...\n\n");
    // GENERATING OPCODE
    let mut vm = VM::new(key, settings["opcode-rounds"].as_i32().expect("could not parse opcode-rounds in whitebox-settings.json"), settings["language"].as_str().unwrap());
    vm.generate();

    println!("building whitebox...");
    println!("key: {:?}\n", vm.key);
    // WRITING TO FILE

    fs::write(

        if settings["language"].as_str().unwrap() == "C++" {
            "compiled.cpp"
        } else {
            std::process::Command::new("cargo").args(&["new", "whitebox-build"]).output().unwrap();
            
            fs::write(
                "whitebox-build/Cargo.toml",
                "[package]
name = \"whitebox-build\"
version = \"0.1.0\"
edition = \"2018\"
                
[dependencies]
aes = \"0.7.5\""
            ).unwrap();
            
            "whitebox-build/src/main.rs"
        },

        fs::read_to_string(
            if settings["language"].as_str().unwrap() == "C++" {
                "cpp/main.cpp"
            } else {
                "rs/template.rs"
            }
        ).unwrap()
        .replace("/*key*/", &vec_to_str(vm.key, settings["language"].as_str().unwrap()))
        .replace("/*magnitudes*/", &vec_to_str(vm.magnitudes, settings["language"].as_str().unwrap()))
        .replace("/*mappings*/", &vec_to_str(vm.mappings, settings["language"].as_str().unwrap()))
        .replace("/*opcode*/", &vec_to_str(vm.opcode.clone(), settings["language"].as_str().unwrap()))
        .replace("/*functions*/", &{
            match settings["language"].as_str().unwrap() {
                "C++" => {
                    let mut s = String::from("");
                    for i in 0..vm.functions.len() {
                        s += &format!(r"[&](void)->void {{
                        {}{}
                    }},", vm.functions.get(i).unwrap(), format!("std::cout << \"{}\";", vm.functions.get(i).unwrap()))
                    }
                    s
                }
                "Rust" | "rust" => {
                    String::new()
                    // vec_to_str((0_u8..vm.functions.len() as u8).collect::<Vec<u8>>(), "Rust")
                    /*let mut s = String::from("");
                    for i in 0..vm.functions.len() {
                        s += &format!("{} => {{ {} }}\n            ", to_u8_value(vm.functions.get(i).unwrap()), vm.functions.get(i).unwrap())
                    }
                    s*/
                }
                _ => {
                    panic!("{}", LANG_ERROR)
                }
            }
        })
        .replace("/*functions-rep*/", &{
            let mut s = String::from("vec![");
            for function in &vm.functions {
                s += &(to_u8_value(function).to_string() + ",")
            }
            s.pop();
            s + "]"
        }) // only in Rust
        .replace("/*matching*/", &{
            match settings["language"].as_str().unwrap() {
                "C++" => {
                    let mut s = String::from(r"if (MATCH(0)) {
                        functions[0]();
                    }");
                    for i in 1..vm.functions.len() {
                        s += &format!(r" else if (MATCH({})) {{
                        functions[{}]();
                    }}", i, i)
                    }
                    s + " else { std::cout << \"COULD NOT IDENTIFY: \" << (int)opcode[pc] << std::endl; for (int i = 0; i < mappings.size(); i++) {std::cout << (int)mappings[i] << \" \";} std::cout << std::endl; exit(1); }"
                }
                "Rust" | "rust" => {
                    String::from("")
                }
                _ => {
                    panic!("{}", LANG_ERROR)
                }
            }
            
        })
        .replace("/*DEC TYPE*/", if settings["encryption-scheme"] == "AES" {
            "_AES();"
        } else {
            "_STREAM();"
        })
    ).unwrap();
    println!("opcode: {:?}", vm.opcode);
    print!("DONE");
}