#ifndef _AES_H_
#define _AES_H_

#include <openssl/aes.h>
#include <openssl/crypto.h>
#include <openssl/buffer.h>
#include <openssl/cmac.h>
#include <openssl/ecdsa.h>
#include <openssl/evp.h>

#include <cstring>
#include <string>
#define bzero(b,len) (memset((b), '\0', (len)), (void) 0)

std::string decrypt(unsigned char *ciphertext, int ciphertext_len, unsigned char *key);

#endif