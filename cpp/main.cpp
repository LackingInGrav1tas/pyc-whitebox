#include <fstream>
#include <vector>
#include <functional>
#include <iostream>
#include <string>
#include <cstring>

#include "cpp/STREAM.h"

void write_to_file(unsigned char *bytes, const char *fname) {
    std::ofstream fout;
    fout.open(fname, std::ios::binary | std::ios::out);

    fout.write((const char *)bytes, strlen((const char *)bytes));

    fout.close();
}

enum Direction {
    LEFT, RIGHT
};

/*template <typename T>
void shift_n(T *p, int n, int size, Direction d) {
    std::cout << "shiftn" << std::endl;
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
    std::cout << "shiftn end" << std::endl;
}*/
template <typename T>
void shift_n(std::vector<T> &v, int n, Direction d) {
    std::cout << n << "|" << d;
    if (d == LEFT) {
        T temp;
        for (int _ = 0; _ < n; _++) {
            temp = v[0];
            for (int i = 1; i < v.size(); i++) {
                v[i-1] = v[i];
            }
            v.back() = temp;
        }
    } else {
        T temp;
        for (int _ = 0; _ < n; _++) {
            temp = v.back();
            for (int i = v.size()-1; i > 0; i--) {
                v[i] = v[i-1];
            }
            v[0] = temp;
        }
    }
}

void bit_rot_n(unsigned char &c, int n, Direction d) {
    std::cout << "-" << n << "|" << d;
    n %= 8;
    if (d == LEFT) {
        c = (c << n)|(c >> (8 - n));
    } else {
        n = 8-n;
        c = (c << n)|(c >> (8 - n));
    }
}

int main(int argc, char** argv) {
    std::vector<unsigned char> key= /*key*/;
    std::cout << "\nobfuscated: ";
    for (int i = 0; i < key.size(); i++) {
        std::cout << (int)key[i] << " ";
    }
    std::cout << "\n" << std::endl;

    // use self editing VM working backwards in the python program to use here
    {
        // shiftable opcode mappings
        
        std::vector<unsigned char> magnitudes = /*magnitudes*/;
        std::vector<unsigned char> opcode = /*opcode*/;
        std::vector<unsigned char> mappings = /*mappings*/;
        std::vector<std::function<void(void)>> functions = {
            /*functions*/
        };

        #define MATCH(n) opcode[pc] == mappings[n]
        for (int pc = 0; pc < opcode.size(); pc++) {
            for (int i = 0; i < key.size(); i++) {
                std::cout << (int)key[i] << " ";
            }
            /*matching*/
            std::cout << std::endl;
        }
    }

    std::cout << "\n\nunobfuscated: ";
    for (int i = 0; i < key.size(); i++) {
        std::cout << (int)key[i] << " ";
    }
    std::cout << std::endl;

    if (argc != 3) {
        std::cout << "format: whitebox.exe <encrypted> <outfile>" << std::endl;
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

    /*auto _AES = [&](void)->void {
        /*AES aes((sizeof(key)/sizeof(unsigned char)));
        unsigned char *decrypted = aes.DecryptECB(plain, 16 * sizeof(unsigned char), key);
        
        std::cout << "ciphertext(" << std::to_string(strlen((char*) decrypted)) << "): ";
        for (int i = 0; i < strlen((char*) decrypted); i++) {
            std::cout << (int)decrypted[i] << " ";
        }
        std::cout << std::endl;

        write_to_file(decrypted, "dec.bin");

        delete plain;
        delete decrypted;
        std::cout << "DONE" << std::endl;* /
        std::ofstream("OUTPUTFILENAME") << decrypt(plain, size, key);
    };*/
    auto _STREAM = [&](void) -> void {
        if (size != key.size()) {
            
            std::cerr << "\nERROR: keysize (" << key.size() << ") != ciphertext size (" << size << ")" << std::endl;
            exit(1);
        }
        unsigned char *d = stream_cipher(key.data(), plain, key.size());
        write_to_file(
            d,
            argv[2]
        );
        delete d;
    };
    /*DEC TYPE*/
    delete plain;
}