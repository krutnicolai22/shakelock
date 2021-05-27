/*
    Copyright 2021  krutnicolai22   krut_nicolai_22@protonmail.com

    This file is part of Shakelock.

    Shakelock is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Shakelock is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Shakelock.  If not, see <https://www.gnu.org/licenses/>.
 */

mod shakelock;
use crate::shakelock::api;

/// Parses command line parameters and executes appropriate functions
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return help();
    }
    let mut binary = false;
    let mut encrypt = false;
    let mut decrypt = false;
    let mut filename = &String::new();
    let mut alphabet = &String::new();
    let mut prompt_in = false;
    let mut prompt_out = false;
    let mut alphabet_next = false;
    for i in 1..args.len() {
        let arg = &args[i];
        if alphabet_next {
            alphabet = arg;
            alphabet_next = false;
        } else if i == args.len() - 1 && !arg.starts_with("-") {
            filename = arg;
        } else {
            if !arg.starts_with("-") || arg.eq("--help") {
                return help();
            } else if arg.eq("--prompt-input") {
                prompt_in = true;
            } else if arg.eq("--prompt-output") {
                prompt_out = true;
            } else if arg.eq("--binary") {
                binary = true;
            } else if arg.eq("--encrypt") {
                encrypt = true;
            } else if arg.eq("--decrypt") {
                decrypt = true;
            } else if arg.eq("--alphabet") {
                alphabet_next = true;
            } else {
                let mut skip_first = true;
                for option in arg.chars() {
                    if skip_first {
                        skip_first = false;
                    } else if option.eq(&'i') {
                        prompt_in = true;
                    } else if option.eq(&'o') {
                        prompt_out = true;
                    } else if option.eq(&'b') {
                        binary = true;
                    } else if option.eq(&'e') {
                        encrypt = true;
                    } else if option.eq(&'d') {
                        decrypt = true;
                    } else if option.eq(&'a') {
                        alphabet_next = true;
                    } else {
                        return help();
                    }
                }
            }
        }
    }
    // Wrong usage
    if (binary && (encrypt || decrypt || prompt_in || prompt_out || filename.is_empty()))
        || ((encrypt || decrypt) && ((encrypt && decrypt) || (filename.is_empty() && !prompt_in))) {
        return help();
    }
    // Correct usage
    return if binary {
        api::binary(filename)
    } else if encrypt || decrypt {
        api::textual(filename, alphabet, encrypt, prompt_in, prompt_out)
    } else {
        help()
    }
}

/// Prints the --help message
fn help() {
    println!("Usage:");
    println!("  shakelock -b INPUT_FILE");
    println!("  shakelock (-e | -d) [-o] [-a ALPHABET_FILE] (-i | INPUT_FILE)");
    println!("Options:");
    println!("  -b, --binary        encrypt or decrypt bytes in an input file");
    println!("  -e, --encrypt       text encryption mode");
    println!("  -d, --decrypt       text decryption mode");
    println!("  -i, --prompt-input  in text mode, type the input into the terminal");
    println!("  -o, --prompt-output in text mode, print the output on the terminal");
    println!("  -a, --alphabet FILE in text mode, use a custom alphabet csv file");
    println!("  -h, --help          display this help and exit");
    println!("Examples:");
    println!("  shakelock --binary file_to_encrypt.bin");
    println!("  shakelock -eoa alphabet.csv text_to_encrypt.txt");
    println!("  shakelock -dio");
    println!("About:");
    println!("  Shakelock uses Shake256 seeded with a password to generate a keystream.");
    println!("  Using the same password more than once on different data makes security");
    println!("  vulnerable to ciphertext correlation. If multiple files are to be encrypted,");
    println!("  it is safest to zip the files into a single file or use different passwords.");
    println!("Donate:");
    println!("  Donations are entirely voluntary and kindly accepted in Bitcoin cash.");
    println!("  bitcoincash:qrpxyalxa2qfpt8akwmq8n5v3gsmslkh8sx93pppvc");
}
