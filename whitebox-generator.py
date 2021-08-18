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
    # ptxt = read_file_as_bytes(argv[1])
    key = read_file_as_bytes(argv[1])
    
    with open("c_templates\\main.cpp", mode='r') as file:
        output = file.read()
    
    # output = output.replace("/*plaintext*/", bytes_to_string(ptxt))
    output = output.replace("/*key*/", bytes_to_string(key))

    mags = "{"
    for i in range(11):
        mags += str(random.randrange(255)) + ","
    mags = mags[:-1]
    output = output.replace("/*magnitudes*/", mags + "}")

    operations = [
        "shift_n(mappings, SIZE, magnitudes[0] % SIZE, LEFT);",
        "shift_n(mappings, SIZE, magnitudes[1] % SIZE, RIGHT);",
        "shift_n(functions.data(), functions.size(), magnitudes[2] % SIZE, LEFT);",
        "shift_n(functions.data(), functions.size(), magnitudes[3] % SIZE, RIGHT);",
        "magnitudes[0]++;",
        "magnitudes[1]++;",
        "magnitudes[2]++;",
        "magnitudes[3]++;",
        "magnitudes[0]--;",
        "magnitudes[1]--;",
        "magnitudes[2]--;",
        "magnitudes[3]--;",

        "for (int i = 0; i < 128; i++) { key[i] <<= magnitudes[4]; }",
        "for (int i = 0; i < 128; i++) { key[i] >>= magnitudes[5]; }",
        "magnitudes[4]++;",
        "magnitudes[5]++;",
        "magnitudes[4]--;",
        "magnitudes[5]--;",

        "for (int i = 0; i < 128; i++) { key[i] ^= magnitudes[6]; }",
        "magnitudes[6]++;",
        "magnitudes[6]--;",

        "for (int i = 0; i < 128; i++) { key[i] = ~key[i]; }",

        "shift_n(magnitudes, SIZE, magnitudes[7] % (sizeof(magnitudes)/sizeof(int)), LEFT);",
        "shift_n(magnitudes, SIZE, magnitudes[8] % (sizeof(magnitudes)/sizeof(int)), RIGHT);",
        "magnitudes[7]++;",
        "magnitudes[7]--;",
        "magnitudes[8]++;",
        "magnitudes[8]--;",

        "shift_n(key, 16, magnitudes[9] % (sizeof(magnitudes)/sizeof(int)), LEFT);",
        "shift_n(key, 16, magnitudes[10] % (sizeof(magnitudes)/sizeof(int)), RIGHT);",
        "magnitudes[9]++;",
        "magnitudes[9]--;",
        "magnitudes[10]++;",
        "magnitudes[10]--;",
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

    output = output.replace("/*opcode*/", "{}")

    for i in range(len(operations)):
        r = random.randrange(len(operations))
        output = output.replace(f"/*op{i}*/", operations[r])
        operations.pop(r)

    with open("compiled.cpp", mode='w') as file:
        file.write(output)
        file.close()
    
    os.system('g++ compiled.cpp c_templates/AES.cpp c_templates/STREAM.cpp -o whitebox.exe && strip whitebox.exe')

if __name__ == "__main__":
    main(sys.argv)