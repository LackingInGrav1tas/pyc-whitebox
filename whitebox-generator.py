import sys, os, random, json

def bytes_to_string(bytes):
    """turns a byte array into the string format of { 0, 1, 2, ... }"""
    string = "{"
    for byte in bytes:
        string += f"{int.from_bytes(byte, 'big')},"
    string = string[:-1]
    return string + "}"

def read_file_as_bytes(fname):
    """returns a vector of bytes containing specified filedata"""
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


    key = read_file_as_bytes(argv[1])
    
    with open("cpp\\main.cpp", mode='r') as file:
        output = file.read()
    
    # output = output.replace("/*plaintext*/", bytes_to_string(ptxt))
    output = output.replace("/*key*/", bytes_to_string(key))

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

    mappings = [ x for x in range(len(operations))]
    random.shuffle(mappings)
    magnitudes = [ random.randrange(255) for x in range(11) ]
    opcode = [ random.randrange(len(operations)) for x in range(jsondata["opcode-rounds"]) ]
    random.shuffle(operations)
    
    
    mags = "{"
    for i in range(len(magnitudes)):
        mags += str(magnitudes.pop(0)) + ', '
    mags = mags[:-1]
    output = output.replace("/*magnitudes*/", mags + "}")

    s = "{"
    for m in mappings:
        s += str(m) + ','
    s = s[:-1]
    output = output.replace("/*mappings*/", s + '}')
    output = output.replace("/*SIZE*/", str(len(operations)))

    functions_str = ""
    for i in range(len(operations)):
        functions_str += f"""[&](void)->void {{
                /*op{i}*/
            }},"""
    output = output.replace("/*functions*/", functions_str)

    op = "{"
    for o in opcode:
        op += str(o) + ','
    op = op[:-1]
    output = output.replace("/*opcode*/", "{}")

    for i in range(len(operations)):
        r = random.randrange(len(operations))
        output = output.replace(f"/*op{i}*/", operations[r])
        operations.pop(r)

    output = output.replace("/*DEC TYPE*/", "_STREAM();" if jsondata["encryption-scheme"] == "stream" else "_AES();")
    output = output.replace("OUTPUTFILENAME", jsondata["decrypted-filename"])

    with open("compiled.cpp", mode='w') as file:
        file.write(output)
        file.close()
    
    os.system(jsondata["compilation-command"])

if __name__ == "__main__":
    main(sys.argv)