#include <fstream>

#include "c_templates/AES.h"

void write_to_file(unsigned char *bytes, const char *fname) {
    ofstream fout;
    fout.open(fname, ios::binary | ios::out);

    fout.write((const char *)bytes, strlen((const char *)bytes));

    fout.close();
}

int main(int argc, char** argv) {
    int pc = 0;
    unsigned char plain[] = /*plaintext*/;
    unsigned char key[] = /*key*/;

    // deobfuscate

    unsigned int size = 16 * sizeof(unsigned char);
    AES aes(128);
    unsigned char *decrypted = aes.DecryptECB(plain, size, key);
    
    write_to_file(decrypted, "dec.bin");
}