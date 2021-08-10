#include <fstream>
#include <vector>
#include <functional>

#include "c_templates/AES.h"

void write_to_file(unsigned char *bytes, const char *fname) {
    ofstream fout;
    fout.open(fname, ios::binary | ios::out);

    fout.write((const char *)bytes, strlen((const char *)bytes));

    fout.close();
}

const int SIZE = 11;

enum Direction {
    LEFT, RIGHT
};

template <typename T>
void shift_n(T *p, int n, int size, Direction d) {
    if (d == LEFT) {
        for (int _ = 0; _ < n; _++) {
            int temp = *p;
            for (int i = 0; i < size-1; i++) {
                *(p+i) = *(p+i+1);
            }
            *(p+size-1) = temp;
        }
    } else {
        for (int _ = 0; _ < n; _++) {
            for (int __ = 0; __ < size-1; __++) {
                int temp = *p;
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
        char mappings[SIZE] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 };
        std::vector<char> opcode = /*opcode*/;

        std::vector<std::function<void(void)>> functions = {
            [&](void)->void {
                /*op1*/
            },
            [&](void)->void {
                /*op2*/
            },
            [&](void)->void {
                /*op3*/
            },
            [&](void)->void {
                /*op4*/
            },
            [&](void)->void {
                /*op5*/
            },
            [&](void)->void {
                /*op6*/
            },
            [&](void)->void {
                /*op7*/
            },
            [&](void)->void {
                /*op8*/
            },
            [&](void)->void {
                /*op9*/
            },
            [&](void)->void {
                /*op10*/
            },
            [&](void)->void {
                /*op11*/
            }
        };

        #define MATCH(n) opcode[i] == mappings[n]
        for (size_t i = 0; i < opcode.size(); i++) {
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
            } 
        }
    }
    unsigned int size = 16 * sizeof(unsigned char);
    AES aes(128);
    unsigned char *decrypted = aes.DecryptECB(plain, size, key);

    write_to_file(decrypted, "dec.bin");
}