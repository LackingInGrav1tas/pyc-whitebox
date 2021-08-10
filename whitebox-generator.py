import sys, os

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

    operations = ["shift_n(mappings, SIZE, %MAG%, LEFT);", "", "", "", "", "", "", "", "", "", "", ""]

    with open("compiled.cpp", mode='w') as file:
        file.write(output)
        file.close()
    
    os.system('g++ compiled.cpp c_templates/AES.cpp -o whitebox.exe && strip whitebox.exe')

if __name__ == "__main__":
    main(sys.argv)