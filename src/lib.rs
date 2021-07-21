#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};
use bitarray::BitArray;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// This structure allows one to generate word presence hashes.
///
/// `B` is the number of bytes in the codewords.
/// `H` is the number of bytes in the hash.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct HammingHasher<const B: usize, const H: usize> {
    codewords: Vec<BitArray<B>>,
}

impl<const B: usize, const H: usize> HammingHasher<B, H> {
    pub fn new() -> Self {
        Self::new_with_seed(0)
    }

    pub fn new_with_seed(seed: u64) -> Self {
        assert_ne!(H, 0);
        let codewords = hamming_dict::generate_dict_seed(H * 8, seed);

        Self { codewords }
    }

    /// There must be exactly `H * 8` codewords.
    pub fn new_with_codewords(codewords: Vec<BitArray<B>>) -> Self {
        assert_eq!(codewords.len(), H * 8);

        Self { codewords }
    }

    /// Convert the input features into a hash.
    pub fn hash_bag<'a>(&self, features: impl IntoIterator<Item = &'a BitArray<B>>) -> BitArray<H> {
        let mut counts = vec![0usize; H * 8];
        let mut distances = vec![0u32; H * 8];
        for feature in features {
            let mut lowest_distance = u32::MAX;
            // Iterate through all the codewords.
            for (distance, codeword) in distances.iter_mut().zip(self.codewords.iter()) {
                // Compute the distance to the codeword and store it.
                *distance = feature.distance(codeword);
                // If this is lower than the lowest distance, update it.
                if *distance < lowest_distance {
                    lowest_distance = *distance;
                }
            }
            // Go through all the distances.
            for (ix, &distance) in distances.iter().enumerate() {
                // If this distance is equal to the lowest distance, add 1 to that keyword.
                if distance == lowest_distance {
                    counts[ix] += 1;
                }
            }
        }
        // Find the largest count threshold that keeps the number of bits set above half (aside from 0).
        let threshold = (2..)
            .find(|&threshold| counts.iter().filter(|&&count| count >= threshold).count() < H * 4)
            .unwrap()
            - 1;
        // Use the threshold to formulate the hash.
        let mut hash = BitArray::zeros();
        for ix in 0..H * 8 {
            hash[ix >> 3] |= ((counts[ix] >= threshold) as u8) << (ix & 0b111);
        }
        hash
    }
}

impl<const B: usize, const H: usize> Default for HammingHasher<B, H> {
    fn default() -> Self {
        Self::new()
    }
}
