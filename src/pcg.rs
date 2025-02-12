// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! PCG random number generators

use rand_core::{RngCore, SeedableRng, Error, impls, le};

/// A PCG random number generator (XSH 64/32 (LCG) variant).
///
/// Permuted Congruential Generators, "xorshift high (bits), random rotation"
/// using an underlying Linear congruential generator
#[derive(Clone)]
pub struct PcgXsh64LcgRng {
    state: u64,
    increment: u64,
}

impl SeedableRng for PcgXsh64LcgRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        // We only have to make sure increment is odd.
        let mut ctx = Self { state: seed_u64[0],
                             increment: seed_u64[1] | 1 };
        // Prepare for the first round
        ctx.state = ctx.state.wrapping_mul(6364136223846793005)
                             .wrapping_add(ctx.increment);
        ctx
    }
}

impl RngCore for PcgXsh64LcgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        // prepare the LCG for the next round
        self.state = state.wrapping_mul(6364136223846793005)
                          .wrapping_add(self.increment);

        // output function XSH RR: xorshift high (bits), followed by a random rotate
        // good for 64-bit state, 32-bit output
        const IN_BITS: u32 = 64;
        const OUT_BITS: u32 = 32;
        const OP_BITS: u32 = 5; // log2(OUT_BITS)

        const ROTATE: u32 = IN_BITS - OP_BITS; // 59
        const XSHIFT: u32 = (OUT_BITS + OP_BITS) / 2; // 18
        const SPARE: u32 = IN_BITS - OUT_BITS - OP_BITS; // 27

        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right((state >> ROTATE) as u32)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
       impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}



/// A PCG random number generator (XSL 64/32 (LCG) variant).
///
/// Permuted Congruential Generators, "xorshift low (bits), random rotation"
/// using an underlying Linear congruential generator
#[derive(Clone)]
pub struct PcgXsl64LcgRng {
    state: u64,
    increment: u64,
}

impl SeedableRng for PcgXsl64LcgRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        // We only have to make sure increment is odd.
        let mut ctx = Self { state: seed_u64[0],
                             increment: seed_u64[1] | 1 };
        // Prepare for the first round
        ctx.state = ctx.state.wrapping_mul(6364136223846793005)
                             .wrapping_add(ctx.increment);
        ctx
    }
}

impl RngCore for PcgXsl64LcgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        // prepare the LCG for the next round
        self.state = state.wrapping_mul(6364136223846793005)
                          .wrapping_add(self.increment);

        // Output function XSL RR ("xorshift low (bits), random rotation"):
        const IN_BITS: u32 = 64;
        const OUT_BITS: u32 = 32;
        const SPARE_BITS: u32 = IN_BITS - OUT_BITS;
        const OP_BITS: u32 = 5; // log2(OUT_BITS)

        const XSHIFT: u32 = (SPARE_BITS + OUT_BITS) / 2; // 32
        const ROTATE: u32 = IN_BITS - OP_BITS; // 59

        let xsl = ((state >> XSHIFT) as u32) ^ (state as u32);
        xsl.rotate_right((state >> ROTATE) as u32)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
       impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}



/// A PCG random number generator (XSL 128/64 (MCG) variant).
///
/// Permuted Congruential Generators, "xorshift low (bits), random rotation"
/// using an underlying multiplicative congruential generator
#[derive(Clone)]
pub struct PcgXsl128McgRng {
    state: u128,
}

const MULTIPLIER: u128 = 2549297995355413924u128 << 64 | 4865540595714422341;

impl SeedableRng for PcgXsl128McgRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        // We only have to make sure increment is odd.
        let mut ctx = Self { state: (seed_u64[0] as u128) << 64 |
                                    (seed_u64[1] as u128) };
        // Prepare for the first round
        ctx.state = ctx.state.wrapping_mul(MULTIPLIER);
        ctx
    }
}

impl RngCore for PcgXsl128McgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let state = self.state;
        // prepare for the next round
        self.state = state.wrapping_mul(MULTIPLIER);

        // Output function XSL RR ("xorshift low (bits), random rotation"):
        const IN_BITS: u32 = 128;
        const OUT_BITS: u32 = 64;
        const SPARE_BITS: u32 = IN_BITS - OUT_BITS;
        const OP_BITS: u32 = 6; // log2(OUT_BITS)

        const XSHIFT: u32 = (SPARE_BITS + OUT_BITS) / 2; // 64
        const ROTATE: u32 = IN_BITS - OP_BITS; // 122

        let xsl = ((state >> XSHIFT) as u64) ^ (state as u64);
        xsl.rotate_right((state >> ROTATE) as u32)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}



#[derive(Clone)]
pub struct MwpRng {
    m: u64,
    w: u64,
}

impl SeedableRng for MwpRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        Self { m: seed_u64[0] | 1, w: seed_u64[1] }
    }
}

impl RngCore for MwpRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.m = self.m.wrapping_mul(6364136223846793005);
        self.w = self.w.wrapping_add(1442695040888963407);
        let state = self.m ^ self.w;

        // output function XSH RR: xorshift high (bits), followed by a random rotate
        const IN_BITS: u32 = 64;
        const OUT_BITS: u32 = 32;
        const OP_BITS: u32 = 5; // log2(OUT_BITS)

        const ROTATE: u32 = IN_BITS - OP_BITS; // 59
        const XSHIFT: u32 = (OUT_BITS + OP_BITS) / 2; // 18
        const SPARE: u32 = IN_BITS - OUT_BITS - OP_BITS; // 27

        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right((state >> ROTATE) as u32)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        // MCG
        self.m = self.m.wrapping_mul(6364136223846793005);
        // Weyl sequence
        self.w = self.w.wrapping_add(1442695040888963407);
        let mut state = self.m ^ self.w;

        // output function RXS M XS:
        // random xorshift, mcg multiply, fixed xorshift
        const BITS: u64 = 64;
        const OP_BITS: u64 = 5; // log2(BITS)
        const MASK: u64 = BITS - 1;

        let rshift = (state >> (BITS - OP_BITS)) & MASK;
        state ^= state >> (OP_BITS + rshift);
        state = state.wrapping_mul(6364136223846793005);
        state ^ (state >> ((2 * BITS + 2) / 3))
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
