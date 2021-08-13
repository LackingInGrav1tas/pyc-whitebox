#include <fstream>
#include <vector>
#include <functional>
#include <iostream>

#include "c_templates/AES.h"

void write_to_file(unsigned char *bytes, const char *fname) {
    ofstream fout;
    fout.open(fname, ios::binary | ios::out);

    fout.write((const char *)bytes, strlen((const char *)bytes));

    fout.close();
}

const int SIZE = /*SIZE*/;

enum Direction {
    LEFT, RIGHT
};

template <typename T>
void shift_n(T *p, int n, int size, Direction d) {
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

int main(int argc, char** argv) {
    unsigned char plain[] = /*plaintext*/;
    unsigned char key[] = /*key*/;    

    // use self editing VM working backwards in the python program to use here
    {
        // shiftable opcode mappings
        char mappings[SIZE] = /*mappings*/;
        int magnitudes[] = /*magnitudes*/;
        std::vector<char> opcode = /*opcode*/;
        int mag_M_L = /*MAGML*/;
        int mag_M_R = /*MAGMR*/;
        int mag_F_L = /*MAGFL*/;
        int mag_F_R = /*MAGFR*/;
        int mag_K_L = /*MAGKL*/;
        int mag_K_R = /*MAGKR*/;
        int mag_xor = /*MAGXOR*/;

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

    unsigned int size = 16 * sizeof(unsigned char);
    AES aes(128);
    unsigned char *decrypted = aes.DecryptECB(plain, size, key);

    write_to_file(decrypted, "dec.bin");
}