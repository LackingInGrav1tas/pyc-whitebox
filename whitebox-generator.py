import sys, os, random

def bytes_to_string(bytes):
    string = "{"
    for byte in bytes:
        string += f"{int.from_bytes(byte, 'big')},"
    string = string[:-1]
    return string + "}"

def read_file_as_bytes(fname):
    bytevec = []
    with open(fname, mode='rb') as file:
        while True:
            x = file.read(1)
            if x != b"":
                bytevec.append(x)
            else:
                break
    return bytevec

def main(argv):
    ptxt = read_file_as_bytes(argv[1])
    key = read_file_as_bytes(argv[2])
    
    with open("c_templates\\main.cpp", mode='r') as file:
        output = file.read()
    
    output = output.replace("/*plaintext*/", bytes_to_string(ptxt))
    output = output.replace("/*key*/", bytes_to_string(key))
    output = output.replace("/*opcode*/", "{}")

    output = output.replace("/*MAGFL*/", str(random.randrange(1, 8)))
    output = output.replace("/*MAGFR*/", str(random.randrange(1, 8)))
    output = output.replace("/*MAGMR*/", str(random.randrange(1, 8)))
    output = output.replace("/*MAGML*/", str(random.randrange(1, 8)))
    output = output.replace("/*MAGKL*/", str(random.randrange(1, 255)))
    output = output.replace("/*MAGKR*/", str(random.randrange(1, 255)))

    output = output.replace("/*MAGXOR*/", str(random.randrange(255)))

    operations = [
        "shift_n(mappings, SIZE, mag_M_L, LEFT);",
        "shift_n(mappings, SIZE, mag_M_R, RIGHT);",
        "shift_n(functions.data(), functions.size(), mag_F_L, LEFT);",
        "shift_n(functions.data(), functions.size(), mag_F_R, RIGHT);",
        "mag_M_L++;",
        "mag_M_R++;",
        "mag_F_L++;",
        "mag_F_R++;",
        "mag_M_L--;",
        "mag_M_R--;",
        "mag_F_L--;",
        "mag_F_R--;",

        "for (int i = 0; i < 128; i++) { key[i] <<= mag_K_L; }",
        "for (int i = 0; i < 128; i++) { key[i] >>= mag_K_R; }",
        "mag_K_L++;",
        "mag_K_R++;",
        "mag_K_L--;",
        "mag_K_R--;",

        "for (int i = 0; i < 128; i++) { key[i] ^= mag_xor; }",
        "for (int i = 0; i < 128; i++) { key[i] = ~key[i]; }",
    ]

    s = "{"
    for i in range(len(operations)):
        s += str(i) + ','
    s = s[:-1]
    output = output.replace("/*mappings*/", s + '}')
    output = output.replace("/*SIZE*/", str(len(operations)))

    functions_str = ""
    for i in range(len(operations)):
        functions_str += f"""[&](void)->void {{
                /*op{i}*/
            }},"""
    output = output.replace("/*functions*/", functions_str)

    for i in range(len(operations)):
        r = random.randrange(len(operations))
        output = output.replace(f"/*op{i}*/", operations[r])
        operations.pop(r)

    with open("compiled.cpp", mode='w') as file:
        file.write(output)
        file.close()
    
    os.system('g++ compiled.cpp c_templates/AES.cpp -o whitebox.exe && strip whitebox.exe')

if __name__ == "__main__":
    main(sys.argv)