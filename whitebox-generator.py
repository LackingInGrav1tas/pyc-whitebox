import sys, os, random, json

def bytes_to_string(bytes):
    """turns a byte array into a string in the format of { 0, 1, 2, ... }"""
    string = "{"
    for byte in bytes:
        string += f"{int.from_bytes(byte, 'big')},"
    string = string[:-1]
    return string + "}"

def read_file_as_bytes(fname):
    """returns a byte array containing specified file data"""
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
    # getting settings from whitebox-settings.py
    with open('whitebox-settings.json', 'r') as file:
        jsondata = json.loads(file.read())

    # reading key to bytearray
    key = read_file_as_bytes(argv[1])
    
    # getting C++ template code
    with open("cpp\\main.cpp", mode='r') as file:
        code = file.read()

    # initializing opcodes
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

    # initializing values for key obfuscation
    mappings = [ x for x in range(len(operations))]
    random.shuffle(mappings)
    magnitudes = [ random.randrange(255) for x in range(11) ]
    opcode = [ random.randrange(len(operations)) for x in range(jsondata["opcode-rounds"]) ]
    random.shuffle(operations)
    
    # writing magnitudes to code
    mags = "{"
    for i in range(len(magnitudes)):
        mags += str(magnitudes.pop(0)) + ', '
    mags = mags[:-1]
    code = code.replace("/*magnitudes*/", mags + "}")

    # writing mappings to code
    s = "{"
    for m in mappings:
        s += str(m) + ','
    s = s[:-1]
    code = code.replace("/*mappings*/", s + '}')
    code = code.replace("/*SIZE*/", str(len(operations)))

    # writing functions to code
    functions_str = ""
    for i in range(len(operations)):
        functions_str += f"""[&](void)->void {{
                /*op{i}*/
            }},"""
    code = code.replace("/*functions*/", functions_str)

    # writing opcode to code
    op = "{"
    for o in opcode:
        op += str(o) + ','
    op = op[:-1]
    code = code.replace("/*opcode*/", "{}")

    for i in range(len(operations)):
        r = random.randrange(len(operations))
        code = code.replace(f"/*op{i}*/", operations[r])
        operations.pop(r)

    # inserting obfuscated key
    code = code.replace("/*key*/", bytes_to_string(key))

    # setting decryption type / custom settings
    code = code.replace("/*DEC TYPE*/", "_STREAM();" if jsondata["encryption-scheme"] == "stream" else "_AES();")
    code = code.replace("OUTPUTFILENAME", jsondata["decrypted-filename"])

    # writing to file
    with open("compiled.cpp", mode='w') as file:
        file.write(code)
        file.close()
    
    # compiling file
    os.system(jsondata["compilation-command"])

if __name__ == "__main__":
    main(sys.argv)