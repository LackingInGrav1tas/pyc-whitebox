#include <fstream>
#include <vector>

#include "c_templates/AES.h"

void write_to_file(unsigned char *bytes, const char *fname) {
    ofstream fout;
    fout.open(fname, ios::binary | ios::out);

    fout.write((const char *)bytes, strlen((const char *)bytes));

    fout.close();
}

int main(int argc, char** argv) {
    unsigned char plain[] = /*plaintext*/;
    unsigned char key[] = /*key*/;    

    // use self editing VM working backwards in the python program to use here
    {
        // shiftable opcode mappings
        char mappings[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA };
        std::vector<char> opcode = /*opcode*/;
        
    }

    unsigned int size = 16 * sizeof(unsigned char);
    AES aes(128);
    unsigned char *decrypted = aes.DecryptECB(plain, size, key);

    write_to_file(decrypted, "dec.bin");
}