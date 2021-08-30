mod sim;

use sim::VM;

use json;

use std::fs;

fn vec_to_cstr(v: Vec<u8>) -> String {
    let mut s = String::from("{");
    for byte in v {
        s += &(byte.to_string() + ",")
    }
    s.pop().expect("");
    s + "}"
}

fn main() {
    println!("fetching settings...");
    // GETTING SETTINGS
    let settings = json::parse(
        &fs::read_to_string("whitebox-settings.json").unwrap()
    ).unwrap();

    println!("generating opcode...");
    // GENERATING OPCODE
    let mut vm = VM::new(vec![], settings["opcode-rounds"].as_i32().expect("could not parse opcode-rounds in whitebox-settings.json"));
    vm.generate();

    println!("writing to file...");
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
                    {}
                }},", &vm.functions.get(i).unwrap())
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
            s
        })
        .replace("/*DEC TYPE*/", if settings["encryption-scheme"] == "AES" {
            "_AES();"
        } else {
            "_STREAM();"
        })
    ).unwrap();

    println!("compiling file...");
    // COMPILING FILE
    print!("{}",
        match std::process::Command::new(
            settings["command-service"].as_str().expect("couldn't parse command-service in whitebox-settings.json")
        )
        .arg("/c")
        .args(
            settings["compilation-command"].as_str().unwrap().split(" ")
        ).status() {
            Ok(_)  => "DONE: COMPILATION SUCCESSFUL",
            Err(_) => "DONE: COMPILATION FAILED"
        }
    );
}