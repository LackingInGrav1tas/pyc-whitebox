#include "AES.h"

std::string decrypt(unsigned char *ciphertext, int ciphertext_len, unsigned char *key) {
    /*EVP_CIPHER_CTX *ctx;
    unsigned char *plaintexts;
    int len;
    int plaintext_len;
    unsigned char* plaintext = new unsigned char[ciphertext_len];
    bzero(plaintext,ciphertext_len);
  
    if(!(ctx = EVP_CIPHER_CTX_new())) exit(1);
  
    if(1 != EVP_DecryptInit_ex(ctx, EVP_aes_128_ecb(), NULL, key, NULL))
      exit(1);
    EVP_CIPHER_CTX_set_key_length(ctx, EVP_MAX_KEY_LENGTH);
  
    if(1 != EVP_DecryptUpdate(ctx, plaintext, &len, ciphertext, ciphertext_len))
      exit(1);
    plaintext_len = len;
    
    int pad_len;
    if(1 != EVP_DecryptFinal_ex(ctx, plaintext + len, &len)) exit(1);
    plaintext_len += len;
    
    plaintext[plaintext_len] = 0;
    
    EVP_CIPHER_CTX_free(ctx);
    std::string ret = (char*)plaintext;
    delete [] plaintext;
    return ret;*/
    AES_cbc_encrypt(NULL, NULL, 16, NULL, NULL, 0);
    return "";
}