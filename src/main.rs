mod sim;

use sim::VM;

use json;

use std::fs;
use std::env;
use std::fs::File;
use std::io::Read;

fn vec_to_cstr(v: Vec<u8>) -> String {
    let mut s = String::from("{");
    for byte in v {
        s += &(byte.to_string() + ",")
    }
    s.pop().expect("");
    s + "}"
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
        &fs::read_to_string("whitebox-settings.json").unwrap()
    ).unwrap();

    println!("generating opcode...");
    // GENERATING OPCODE
    let mut vm = VM::new(key, settings["opcode-rounds"].as_i32().expect("could not parse opcode-rounds in whitebox-settings.json"));
    vm.generate();

    println!("writing to file...");
    println!("key: {:?}\n", vm.key);
    // WRITING TO FILE
    fs::write("compiled.cpp",
        fs::read_to_string("cpp/main.cpp").unwrap()
        .replace("/*key*/", &vec_to_cstr(vm.key))
        .replace("/*magnitudes*/", &vec_to_cstr(vm.magnitudes))
        .replace("/*mappings*/", &vec_to_cstr(vm.mappings))
        .replace("/*opcode*/", &vec_to_cstr(vm.opcode))
        .replace("/*functions*/", &{
            let mut s = String::from("");
            for i in 0..vm.functions.len() {
                s += &format!(r"[&](void)->void {{
                {}{}
            }},", vm.functions.get(i).unwrap(), format!("std::cout << \"{}\";", vm.functions.get(i).unwrap()))
            }
            s
        })
        .replace("/*matching*/", &{
            let mut s = String::from(r"if (MATCH(0)) {
                functions[0]();
            }");
            for i in 1..vm.functions.len() {
                s += &format!(r" else if (MATCH({})) {{
                functions[{}]();
            }}", i, i)
            }
            s + " else { std::cout << \"COULD NOT IDENTIFY: \" << (int)opcode[pc] << std::endl; for (int i = 0; i < mappings.size(); i++) {std::cout << (int)mappings[i] << \" \";} std::cout << std::endl; exit(1); }"
        })
        .replace("/*DEC TYPE*/", if settings["encryption-scheme"] == "AES" {
            "_AES();"
        } else {
            "_STREAM();"
        })
    ).unwrap();

    println!("compiling file...");
    // COMPILING FILE
    let mut c = std::process::Command::new(
        settings["command-service"].as_str().expect("couldn't parse command-service in whitebox-settings.json")
    );
    c.arg("/c");
    c.args(
        settings["compilation-command"].as_str().unwrap().split(" ")
    );
    println!("CMD: {:?}", c);
    print!("{}",
        match c.status() {
            Ok(status)  => format!("DONE: COMPILATION {}",
                if status.success() {
                    "SUCCESSFUL"
                } else {
                    "FAILED"
                }
            ),
            Err(_) => String::from("DONE: CALL FAILED")
        }
    );
}