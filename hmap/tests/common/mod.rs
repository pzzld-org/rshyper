/*
    Appellation: common <tests>
    Contrib: @FL03
*/
//! helpers shared by the integration tests
#![allow(dead_code)]
use core::hash::{BuildHasher, Hasher};

/// deterministic hasher: hashbrown buckets by low hash bits, so the raw iteration order of the
/// maps becomes the reverse of the id order, exposing any reliance on the raw order.
#[derive(Clone, Copy, Default)]
pub struct RevBuild;

#[derive(Default)]
pub struct RevHasher(u64);

impl Hasher for RevHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(u64::from(b));
        }
    }
    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64;
    }
    fn finish(&self) -> u64 {
        !self.0
    }
}

impl BuildHasher for RevBuild {
    type Hasher = RevHasher;

    fn build_hasher(&self) -> RevHasher {
        RevHasher::default()
    }
}

/// splitmix64: a fixed-seed generator so tests need no new dependency
pub struct SplitMix(pub u64);

impl SplitMix {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// uniform in `0..n` (n > 0)
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}
