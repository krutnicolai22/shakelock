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

use super::keccak::Keccak;

// Standard alphabet a to z plus space
const ALPHABET: &str = "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z, ";

// Name of output file
const OUTPUT_FILENAME: &str = "shakelock-output";

// Shake256 parameters for Keccak
const RATE: usize = 1088;
const CAPACITY: usize = 512;
const SUFFIX: u8 = 0x1F;

/// Prompts user for a password in the terminal and absorbs this into a returned shake256 instance
fn prompt_password() -> Keccak {
    let mut password_string = String::new();
    println!("Enter password:");
    std::io::stdin().read_line(&mut password_string).unwrap();
    password_string = password_string.lines().next().unwrap().to_string();
    let password_bytes = password_string.as_bytes();
    let mut shake = Keccak::new(RATE, CAPACITY, SUFFIX);
    shake.absorb(password_bytes);
    shake
}

/// Does binary encryption/decryption of a file by xor-ing all bytes with shake256(password)
pub fn binary(filename: &String) {
    let input_bytes = std::fs::read(filename).unwrap();
    let mut shake = prompt_password();
    let mut output_bytes: Vec<u8> = Vec::new();
    for byte in input_bytes {
        let mut shake_byte = [0];
        shake.squeeze(&mut shake_byte);
        output_bytes.push(byte ^ shake_byte[0]);
    }
    std::fs::write(OUTPUT_FILENAME, output_bytes).unwrap();
}

/// Does textual encryption/decryption by adding/subtracting random numbers from shake256(password)
pub fn textual(in_file: &String, alphabet: &String, add: bool, prompt_in: bool, prompt_out: bool) {
    // Set up alphabet
    let mut alphabet_string = String::from(ALPHABET);
    if !alphabet.is_empty() {
        alphabet_string = std::fs::read_to_string(alphabet).unwrap();
        alphabet_string = alphabet_string.lines().next().unwrap().to_string();
    }
    let split = alphabet_string.split(",");
    let mut alphabet = Vec::new();
    for char in split {
        if char.len() == 0 {
            alphabet.push(',');
        } else {
            alphabet.push(char.chars().next().unwrap());
        }
    }
    let alphabet_length = alphabet.len() as i32;
    if alphabet_length > 255 {
        panic!("Cannot use an alphabet with more than 255 characters");
    }
    // Get input
    let mut input_string = String::new();
    if prompt_in {
        println!("Enter input:");
        std::io::stdin().read_line(&mut input_string).unwrap();
    } else {
        input_string = std::fs::read_to_string(in_file).unwrap();
    }
    input_string = input_string.lines().next().unwrap().to_string();
    // Do the encryption
    let mut shake = prompt_password();
    let mut output_string = String::new();
    let mut warning_given = false;
    for char in input_string.chars() {
        // First find index in 0 to alphabet.len() - 1 of the character
        let mut input_index = -1;
        for i in 0..alphabet_length {
            if char.eq(&alphabet[i as usize]) {
                input_index = i;
            }
        }
        // Foreign characters will be pushed unencrypted, with a warning to the user
        if input_index == -1 {
            if !warning_given {
                println!("Warning: some characters of the input are not in the alphabet");
                warning_given = true;
            }
            output_string.push(char);
            continue;
        }
        // Next get a random number from 0 to alphabet.len() - 1 from the shake output
        let mut key = -1;
        while key < 0 || key >= alphabet_length {
            let mut shake_byte = [0];
            shake.squeeze(&mut shake_byte);
            key = shake_byte[0] as i32;
        }
        // Finally, add or subtract modulo alphabet.len() to get the encrypted/decrypted character
        let output_index = if add {
            (input_index + key).rem_euclid(alphabet_length)
        } else {
            (input_index - key).rem_euclid(alphabet_length)
        };
        output_string.push(alphabet[output_index as usize]);
    }
    if prompt_out {
        println!("Result:");
        println!("{}", output_string);
    } else {
        std::fs::write(OUTPUT_FILENAME, output_string).unwrap();
    }
}