#include <iostream>

unsigned char* stream_cipher(unsigned char* key, unsigned char* plaintext, int size) {
    unsigned char* result = new unsigned char[size];
    for (int i = 0; i < size; i++) {
        result[i] = key[i] ^ plaintext[i];
        std::cout << (int)result[i] << " = " << (int)key[i] << " ^ " << (int)plaintext[i] << std::endl;
    }
    return result;
}