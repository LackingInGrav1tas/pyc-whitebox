# rsc-whitebox

Rust whitebox generator which creates an obfuscated C++ file containing a specified key and decryption scheme. Also, the obfuscation techinque used is completely untested and idk how secure it is. Use at your own risk (preferably not at all).

---
### HOW TO USE

generator format: ``` whitebox-generator.exe <key-file>```
whitebox format: ```whitebox.exe <encrypted-file> <output-file>```

Options (i.e. compilation, obfucsation rounds, scheme, etc.) are found and editable in the ```whitebox.json``` file.

Please note that compilation of the c++ whitebox requires OpenSSL to be installed. I used Strawberry in the default ```whitebox-settings.json```. The compilation can be done by hand if that's easier, as the Rust program just creates the ```compiled.cpp``` file.


---

This product includes software developed by the OpenSSL Project 
for use in the OpenSSL Toolkit (http://www.openssl.org/)
