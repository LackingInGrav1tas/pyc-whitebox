import sys, os, random, json, math

def bytes_to_string(bytes, readable=False):
    """turns a byte array into a string in the format of { 0, 1, 2, ... }"""
    string = "{"
    for byte in bytes:
        string += f"{byte if readable else int.from_bytes(byte, 'big')},"
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

def shift_arr(l, mag, direction):
    for _ in range(mag):
        l.append(l.pop(0)) if direction == "LEFT" else l.insert(0, l.pop())
    return l

def main(argv):
    # getting settings from whitebox-settings.py
    with open('whitebox-settings.json', 'r') as file:
        jsondata = json.loads(file.read())

    # reading key to array
    key = read_file_as_bytes(argv[1])
    for i in range(len(key)):
        key[i] = int.from_bytes(key[i], 'big')
    
    # getting C++ template code
    with open("cpp\\main.cpp", mode='r') as file:
        code = file.read()

    # initializing opcodes
    functions = [
        "shift_n(mappings, SIZE, magnitudes[0] % SIZE, LEFT);",
        "shift_n(mappings, SIZE, magnitudes[1] % SIZE, RIGHT);",
        "shift_n(functions.data(), functions.size(), magnitudes[2] % functions.size(), LEFT);",
        "shift_n(functions.data(), functions.size(), magnitudes[3] % functions.size(), RIGHT);",
        "magnitudes[0]++;",
        "magnitudes[1]++;",
        "magnitudes[2]++;",
        "magnitudes[3]++;",
        "magnitudes[0]--;",
        "magnitudes[1]--;",
        "magnitudes[2]--;",
        "magnitudes[3]--;",

        "for (int i = 0; i < 16; i++) { key[i] <<= magnitudes[4]; }",
        "for (int i = 0; i < 16; i++) { key[i] >>= magnitudes[5]; }",
        "magnitudes[4]++;",
        "magnitudes[5]++;",
        "magnitudes[4]--;",
        "magnitudes[5]--;",

        "for (int i = 0; i < 16; i++) { key[i] ^= magnitudes[6]; }",
        "magnitudes[6]++;",
        "magnitudes[6]--;",

        "for (int i = 0; i < 16; i++) { key[i] = ~key[i]; }",

        "shift_n(magnitudes, (sizeof(magnitudes)/sizeof(int)), magnitudes[7] % (sizeof(magnitudes)/sizeof(int)), LEFT);",
        "shift_n(magnitudes, (sizeof(magnitudes)/sizeof(int)), magnitudes[8] % (sizeof(magnitudes)/sizeof(int)), RIGHT);",
        "magnitudes[7]++;",
        "magnitudes[7]--;",
        "magnitudes[8]++;",
        "magnitudes[8]--;",

        "shift_n(key, 16, magnitudes[9] % 16, LEFT);",
        "shift_n(key, 16, magnitudes[10] % 16, RIGHT);",
        "magnitudes[9]++;",
        "magnitudes[9]--;",
        "magnitudes[10]++;",
        "magnitudes[10]--;",
    ]

    # initializing values for key obfuscation
    mappings = [ x for x in range(len(functions))]
    random.shuffle(mappings)
    magnitudes = [ random.randrange(255) for _ in range(11) ]
    opcode = [ random.randrange(len(functions)) for _ in range(jsondata["opcode-rounds"]) ]
    random.shuffle(functions)

    # does the opposite of the op (going backwards)
    def call_op(op):
        nonlocal mappings
        nonlocal functions
        nonlocal magnitudes

        # pattern matching
        def matches(s, back=False):
            nonlocal op

            if len(op) < len(s): return False
            return ( op[0:len(s)] if not back else op[len(op)-len(s):] ) == s

        if matches("magnitudes["):
            # magnitudes[x]??;
            index = int( ( op.split('[')[1] ).split(']')[0] ) # retrieving the value between [ and ]
            if matches("+;", True): magnitudes[index] -= 1
            else: magnitudes[index] += 1
        
        elif matches("shift_n(mappings"):
            # shift_n(mappings, SIZE, magnitudes[?] % SIZE, DIR);
            if matches("LEFT);", True):
                shift_arr(mappings, magnitudes[0] % len(mappings), "RIGHT")
            else:
                shift_arr(mappings, magnitudes[1] % len(mappings), "LEFT")

        elif matches("shift_n(functions"):
            # shift_n(functions, SIZE, magnitudes[?] % SIZE, DIR);
            if matches("LEFT);", True):
                shift_arr(functions, magnitudes[2] % len(functions), "RIGHT")
            else:
                shift_arr(functions, magnitudes[3] % len(functions), "LEFT")

        elif matches("shift_n(magnitudes"):
            # shift_n(magnitudes, SIZE, magnitudes[?] % SIZE, DIR);
            if matches("LEFT);", True):
                shift_arr(functions, magnitudes[7] % len(magnitudes), "RIGHT")
            else:
                shift_arr(functions, magnitudes[8] % len(magnitudes), "LEFT")

        elif matches("shift_n(key"):
            # shift_n(key, SIZE, magnitudes[?] % 16, DIR);
            if matches("LEFT);", True):
                shift_arr(key, magnitudes[9] % len(key), "RIGHT")
            else:
                shift_arr(key, magnitudes[10] % len(key), "LEFT")

        elif matches("{ key[i] = ~key[i]; }", True):
            # for (int i = 0; i < 16; i++) { key[i] = ~key[i]; }
            for i in range(len(key)):
                key[i] =  ( 0 - (key[i]+1) )

        elif matches("{ key[i] ^= magnitudes[6]; }", True):
            # for (int i = 0; i < 16; i++) { key[i] ^= magnitudes[6]; }
            for i in range(len(key)):
                key[i] ^= magnitudes[6]

        elif matches("{ key[i] <<= magnitudes[4]; }", True):
            # for (int i = 0; i < 16; i++) { key[i] <<= magnitudes[4]; }
            for i in range(len(key)):
                key[i] >>= magnitudes[4]
        
        elif matches("{ key[i] >>= magnitudes[4]; }", True):
            # for (int i = 0; i < 16; i++) { key[i] >>= magnitudes[4]; }
            for i in range(len(key)):
                key[i] <<= magnitudes[4]


    # reverse engineering obfuscation
    print("generating obfuscation [          ] 0.0%", end='')
    for i in range(len(opcode)):
        # display
        done = i / ( len(opcode) - 1)
        fraction = math.floor( ( i / ( len(opcode) - 1) ) * 10 )
        print(
            f"\rgenerating obfuscation [{'#' * fraction}{' ' * (10 - fraction)}] {'{:.1%}'.format(done)}",
            end=''
        )
        # calling op
        call_op(functions[opcode[i]])
    print() # flushing
     
    
    # writing magnitudes to code
    print("writing magnitudes...")
    mags = "{"
    for i in range(len(magnitudes)):
        mags += str(magnitudes.pop(0)) + ', '
    mags = mags[:-1]
    code = code.replace("/*magnitudes*/", mags + "}")

    # writing mappings to code
    print("writing mappings...")
    s = "{"
    for m in mappings:
        s += str(m) + ','
    s = s[:-1]
    code = code.replace("/*mappings*/", s + '}')
    code = code.replace("/*SIZE*/", str(len(functions)))

    # writing functions to code
    print("writing functions...")
    functions_str = ""
    for i in range(len(functions)):
        functions_str += f"""[&](void)->void {{
                /*op{i}*/
            }},"""
    code = code.replace("/*functions*/", functions_str)

    # reversing then writing opcode to code
    print("writing opcode...")
    opcode.reverse()
    code = code.replace("/*opcode*/", f"{bytes_to_string(opcode, True)}")

    for i in range(len(functions)):
        r = random.randrange(len(functions))
        code = code.replace(f"/*op{i}*/", functions[r])
        functions.pop(r)

    # inserting obfuscated key
    print("writing obfuscated key...")
    code = code.replace("/*key*/", bytes_to_string(key, True))

    # setting decryption type / custom settings
    code = code.replace("/*DEC TYPE*/", "_STREAM();" if jsondata["encryption-scheme"] == "stream" else "_AES();")
    code = code.replace("OUTPUTFILENAME", jsondata["decrypted-filename"])

    # writing to file
    print("creating C++ file...")
    with open("compiled.cpp", mode='w') as file:
        file.write(code)
        file.close()
    
    # compiling file
    print("compiling file...")
    os.system(jsondata["compilation-command"])
    print("DONE", end='')

if __name__ == "__main__":
    main(sys.argv)