#include <fstream>
#include <vector>
#include <functional>
#include <iostream>
#include <string>
#include <cstring>

#include "cpp/AES.h"
#include "cpp/STREAM.h"

void write_to_file(unsigned char *bytes, const char *fname) {
    std::ofstream fout;
    fout.open(fname, std::ios::binary | std::ios::out);

    fout.write((const char *)bytes, strlen((const char *)bytes));

    fout.close();
}

const int SIZE = /*SIZE*/;

enum Direction {
    LEFT, RIGHT
};

template <typename T>
void shift_n(T *p, int n, int size, Direction d) {
    n %= size;
    if (d == LEFT) {
        for (int _ = 0; _ < n; _++) {
            T temp = *p;
            for (int i = 0; i < size-1; i++) {
                *(p+i) = *(p+i+1);
            }
            *(p+size-1) = temp;
        }
    } else {
        for (int _ = 0; _ < n; _++) {
            for (int __ = 0; __ < size-1; __++) {
                T temp = *p;
                for (int i = 0; i < size-1; i++) {
                    *(p+i) = *(p+i+1);
                }
                *(p+size-1) = temp;
            }
        }
    }
}

void bit_rot_n(unsigned char &c, int n, Direction d) {
    n %= 8;
    if (d == LEFT) {
        c = (c << n)|(c >> (8 - n));
    } else {
        n = 8-n;
        c = (c << n)|(c >> (8 - n));
    }
}

int main(int argc, char** argv) {
    unsigned char key[] = /*key*/;    

    // use self editing VM working backwards in the python program to use here
    {
        // shiftable opcode mappings
        char mappings[SIZE] = /*mappings*/;
        int magnitudes[] = /*magnitudes*/;
        std::vector<char> opcode = /*opcode*/;

        std::vector<std::function<void(void)>> functions = {
            /*functions*/
        };

        #define MATCH(n) opcode[pc] == mappings[n]
        for (int pc = 0; pc < opcode.size(); pc++) {
            if (MATCH(0)) {
                functions[0]();
            } else if (MATCH(1)) {
                functions[1]();
            } else if (MATCH(2)) {
                functions[2]();
            } else if (MATCH(3)) {
                functions[3]();
            } else if (MATCH(4)) {
                functions[4]();
            } else if (MATCH(5)) {
                functions[5]();
            } else if (MATCH(6)) {
                functions[6]();
            } else if (MATCH(7)) {
                functions[7]();
            } else if (MATCH(8)) {
                functions[8]();
            } else if (MATCH(9)) {
                functions[9]();
            } else if (MATCH(10)) {
                functions[10]();
            } else if (MATCH(11)) {
                functions[11]();
            }  else if (MATCH(12)) {
                functions[12]();
            }  else if (MATCH(13)) {
                functions[13]();
            }  else if (MATCH(14)) {
                functions[14]();
            }  else if (MATCH(15)) {
                functions[15]();
            }  else if (MATCH(16)) {
                functions[16]();
            }  else if (MATCH(17)) {
                functions[17]();
            }  else if (MATCH(18)) {
                functions[18]();
            } 
        }
    }

    for (int i = 0; i < 128; i++) {
        std::cout << (int)key[i] << " ";
    }
    std::cout << std::endl;

    if (argc != 2) {
        std::cout << "expected a file to decrypt" << std::endl;
        exit(1);
    }

    unsigned char *plain;
    std::ifstream file(argv[1], std::ios::in|std::ios::binary|std::ifstream::ate);
    unsigned int size = file.tellg();
    plain = new unsigned char[size];
    file.seekg(0, std::ios::beg);
    file.read((char*)plain, size);

    std::cout << "plaintext(" << size << "): ";
    for (int i = 0; i < size; i++) {
        std::cout << (int)plain[i] << " ";
    }
    std::cout << std::endl;

    auto _AES = [&](void)->void {
        /*AES aes(128);
        unsigned char *decrypted = aes.DecryptECB(plain, 16 * sizeof(unsigned char), key);
        
        std::cout << "ciphertext(" << std::to_string(strlen((char*) decrypted)) << "): ";
        for (int i = 0; i < strlen((char*) decrypted); i++) {
            std::cout << (int)decrypted[i] << " ";
        }
        std::cout << std::endl;

        write_to_file(decrypted, "dec.bin");

        delete plain;
        delete decrypted;
        std::cout << "DONE" << std::endl;*/
        std::ofstream("OUTPUTFILENAME") << decrypt(plain, size, key);
    };
    auto _STREAM = [&](void) -> void {
        if (size != (sizeof(key)/sizeof(unsigned char))) {
            
            std::cerr << "\nERROR: keysize (" << (sizeof(key)/sizeof(unsigned char)) << ") != ciphertext size (" << size << ")" << std::endl;
            exit(1);
        }
        write_to_file(
            stream_cipher(key, plain, (sizeof(key)/sizeof(unsigned char))),
            argv[2]
        );
    };
    /*DEC TYPE*/
}