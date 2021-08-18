#include <iostream>

unsigned char* stream_cipher(unsigned char* key, unsigned char* plaintext, int size) {
    unsigned char* result = new unsigned char[size];
    for (int i = 0; i < size; i++) {
        result[i] = key[i] ^ plaintext[i];
        // std::cout << (char)result[i] << " = " << (char)key[i] << " ^ " << (char)plaintext[i] << std::endl;
    }
    return result;
}