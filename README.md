# shakelock
A command line tool for text and file encryption. When encrypting text the output is also an equally long text. This makes shakelock suitable for encrypting text that is to be written down, such as cryptocurrency seed phrases.

### Binaries
Binaries for linux and windows [here](https://github.com/krutnicolai22/shakelock/releases).

### About
Shakelock uses Shake256 seeded with a password to generate a keystream. Using the same password more than once on different data makes security vulnerable to ciphertext correlation. If multiple files are to be encrypted, it is safest to zip the files into a single file or use different passwords.
  
### Example
    $ ./shakelock -eio
    Enter input:
    the quick brown fox jumps over the lazy dog
    Enter password:
    password123
    Result:
    luucrczlgagnriocdffrqzuskcmaierdkail czyftk
    
    $ ./shakelock -dio  
    Enter input:  
    luucrczlgagnriocdffrqzuskcmaierdkail czyftk  
    Enter password:  
    password123  
    Result:  
    the quick brown fox jumps over the lazy dog  

### Manual
    Usage:  
      shakelock -b INPUT_FILE  
      shakelock (-e | -d) [-o] [-a ALPHABET_FILE] (-i | INPUT_FILE)  
    Options:  
      -b, --binary        encrypt or decrypt bytes in an input file  
      -e, --encrypt       text encryption mode  
      -d, --decrypt       text decryption mode  
      -i, --prompt-input  in text mode, type the input into the terminal  
      -o, --prompt-output in text mode, print the output on the terminal  
      -a, --alphabet FILE in text mode, use a custom alphabet csv file  
      -h, --help          display this help and exit  

### Donate  
Donations are accepted in Bitcoin cash at bitcoincash:qrpxyalxa2qfpt8akwmq8n5v3gsmslkh8sx93pppvc

### License
Distributed under the GPL license, see [LICENSE](LICENSE) for details.
