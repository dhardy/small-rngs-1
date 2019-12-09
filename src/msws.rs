// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Middle Square Weyl Sequence RNG

use rand_core::{RngCore, SeedableRng, Error, impls, le};

/// Middle Square Weyl Sequence RNG
///
/// - Author: Bernard Widynski
/// - License: GPL
/// - Source: https://mswsrng.wixsite.com/rand
/// - Period: 2<sup>64</sup>
/// - State: 192 bits
/// - Word size: 64 bits
/// - Seed size: 128 bits
#[derive(Clone)]
pub struct MswsRng {
    x: u64,
    w: u64,
    s: u64,
}

impl SeedableRng for MswsRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        // The constant s should be set to a random 64-bit pattern with the
        // upper 32 bits non-zero and the least significant bit set to 1
        let stream = seed_u64[0] | 1;
        if stream & 0xffffffff_00000000 == 0 { panic!("bad seed: high bits are zero"); }
        Self { x: seed_u64[1], w: 0, s: stream }
    }

    fn from_rng<R: RngCore>(mut other: R) -> Result<Self, Error> {
        let mut stream;
        loop {
            // The constant s should be set to a random 64-bit pattern with the
            // upper 32 bits non-zero and the least significant bit set to 1
            stream = other.next_u64() | 1;
            if stream & 0xffffffff_00000000 != 0 { break; }
        }
        Ok(Self { x: other.next_u64(), w: 0, s: stream })
    }
}

impl RngCore for MswsRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.x = self.x.wrapping_mul(self.x);
        self.w = self.w.wrapping_add(self.s);
        self.x = self.x.wrapping_add(self.w);
        self.x.rotate_left(32)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
