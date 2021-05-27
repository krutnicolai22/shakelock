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

/// Holds the state and supporting variables for a keccak sponge
pub struct Keccak {
    state: [u64; 25],
    rate_in_bytes: usize,
    delimited_suffix: u8,
    absorbed: usize,
    squeezed: usize,
}

impl Keccak {

    /// Initializes a keccak sponge with the chosen parameters
    pub fn new(rate: usize, capacity: usize, delimited_suffix: u8) -> Keccak {
        if ((rate + capacity) != 1600) || ((rate % 8) != 0) {
            panic!("Erronous Keccak parameters")
        }
        Keccak {
            state: [0; 25],
            rate_in_bytes: rate/8,
            delimited_suffix,
            absorbed: 0,
            squeezed: rate/8
        }
    }

    /// XOR a byte in the state array stored in u64 with the input byte
    fn xor_state_byte(&mut self, i: usize, to_xor: u8) {
        let mut state_bytes = self.state[i/8].to_le_bytes();
        state_bytes[i%8] ^= to_xor;
        self.state[i/8] = u64::from_le_bytes(state_bytes);
    }

    /// Absorbs input bytes into the state, calling the permutation function if necessary
    pub fn absorb(&mut self, input: &[u8]) {
        if self.squeezed != self.rate_in_bytes {
            panic!("Cannot absorb more after squeezing")
        }
        for i in 0..input.len() {
            self.xor_state_byte(self.absorbed, input[i]);
            self.absorbed += 1;
            if self.absorbed == self.rate_in_bytes {
                f1600(&mut self.state);
                self.absorbed = 0;
            }
        }
    }

    /// Pads the input when shifting from absorbtion to squeezing
    fn pad(&mut self) {
        self.xor_state_byte(self.absorbed, self.delimited_suffix);
        if ((self.delimited_suffix & 0x80) != 0) && (self.absorbed == (self.rate_in_bytes-1)) {
            f1600(&mut self.state);
        }
        self.xor_state_byte(self.rate_in_bytes-1,0x80);
        f1600(&mut self.state);
    }

    /// Squeezes output bytes from the state, calling the permutation function if necessary
    pub fn squeeze(&mut self, output: &mut [u8]) {
        if self.squeezed == self.rate_in_bytes {
            self.pad();
            self.squeezed = 0;
        }
        for i in 0..output.len() {
            output[i] = self.state[self.squeezed/8].to_le_bytes()[self.squeezed % 8];
            self.squeezed += 1;
            if self.squeezed == self.rate_in_bytes {
                f1600(&mut self.state);
                self.squeezed = 0;
            }
        }
    }
}

/// Keccak-f[1600] permutation
fn f1600(a: &mut [u64; 25]) {
    const RC: [u64; 24] = [0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
        0x8000000080008000, 0x000000000000808b, 0x0000000080000001, 0x8000000080008081,
        0x8000000000008009, 0x000000000000008a, 0x0000000000000088, 0x0000000080008009,
        0x000000008000000a, 0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
        0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 0x000000000000800a,
        0x800000008000000a, 0x8000000080008081, 0x8000000000008080, 0x0000000080000001,
        0x8000000080008008,
    ];
    for i in 0..24 {
        let mut c: [u64; 5] = [0; 5];
        let mut d: [u64; 5] = [0; 5];

        // Theta
        c[0] = a[0] ^ a[5] ^ a[10] ^ a[15] ^ a[20];
        c[1] = a[1] ^ a[6] ^ a[11] ^ a[16] ^ a[21];
        c[2] = a[2] ^ a[7] ^ a[12] ^ a[17] ^ a[22];
        c[3] = a[3] ^ a[8] ^ a[13] ^ a[18] ^ a[23];
        c[4] = a[4] ^ a[9] ^ a[14] ^ a[19] ^ a[24];
        d[0] = c[4] ^ c[1].rotate_left(1);
        d[1] = c[0] ^ c[2].rotate_left(1);
        d[2] = c[1] ^ c[3].rotate_left(1);
        d[3] = c[2] ^ c[4].rotate_left(1);
        d[4] = c[3] ^ c[0].rotate_left(1);

        // Rho, pi
        a[0] ^= d[0];
        let a16 = a[16];
        a[16] = (a[5] ^ d[0]).rotate_left(36);
        a[5] = (a[3] ^ d[3]).rotate_left(28);
        a[3] = (a[18] ^ d[3]).rotate_left(21);
        a[18] = (a[17] ^ d[2]).rotate_left(15);
        a[17] = (a[11] ^ d[1]).rotate_left(10);
        a[11] = (a[7] ^ d[2]).rotate_left(6);
        a[7] = (a[10] ^ d[0]).rotate_left(3);
        a[10] = (a[1] ^ d[1]).rotate_left(1);
        a[1] = (a[6] ^ d[1]).rotate_left(44);
        a[6] = (a[9] ^ d[4]).rotate_left(20);
        a[9] = (a[22] ^ d[2]).rotate_left(61);
        a[22] = (a[14] ^ d[4]).rotate_left(39);
        a[14] = (a[20] ^ d[0]).rotate_left(18);
        a[20] = (a[2] ^ d[2]).rotate_left(62);
        a[2] = (a[12] ^ d[2]).rotate_left(43);
        a[12] = (a[13] ^ d[3]).rotate_left(25);
        a[13] = (a[19] ^ d[4]).rotate_left(8);
        a[19] = (a[23] ^ d[3]).rotate_left(56);
        a[23] = (a[15] ^ d[0]).rotate_left(41);
        a[15] = (a[4] ^ d[4]).rotate_left(27);
        a[4] = (a[24] ^ d[4]).rotate_left(14);
        a[24] = (a[21] ^ d[1]).rotate_left(2);
        a[21] = (a[8] ^ d[3]).rotate_left(55);
        a[8] = (a16 ^ d[1]).rotate_left(45);

        // Chi
        let a0 = a[0];
        let a1 = a[1];
        a[0] ^= !a[1] & a[2];
        a[1] ^= !a[2] & a[3];
        a[2] ^= !a[3] & a[4];
        a[3] ^= !a[4] & a0;
        a[4] ^= !a0 & a1;
        let a5 = a[5];
        let a6 = a[6];
        a[5] ^= !a[6] & a[7];
        a[6] ^= !a[7] & a[8];
        a[7] ^= !a[8] & a[9];
        a[8] ^= !a[9] & a5;
        a[9] ^= !a5 & a6;
        let a10 = a[10];
        let a11 = a[11];
        a[10] ^= !a[11] & a[12];
        a[11] ^= !a[12] & a[13];
        a[12] ^= !a[13] & a[14];
        a[13] ^= !a[14] & a10;
        a[14] ^= !a10 & a11;
        let a15 = a[15];
        let a16 = a[16];
        a[15] ^= !a[16] & a[17];
        a[16] ^= !a[17] & a[18];
        a[17] ^= !a[18] & a[19];
        a[18] ^= !a[19] & a15;
        a[19] ^= !a15 & a16;
        let a20 = a[20];
        let a21 = a[21];
        a[20] ^= !a[21] & a[22];
        a[21] ^= !a[22] & a[23];
        a[22] ^= !a[23] & a[24];
        a[23] ^= !a[24] & a20;
        a[24] ^= !a20 & a21;

        // Iota
        a[0] ^= RC[i];
    }
}