use bitarray::BitArray;
use hamming_bow::HammingHasher;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn test_balance_on<const B: usize, const H: usize>(hasher: &HammingHasher<B, H>, seed: u64) -> u32 {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

    let features: Vec<_> = (0..32768)
        .map(|_| {
            let mut arr = [0u8; B];
            for byte in &mut arr {
                *byte = rng.gen();
            }
            arr
        })
        .map(BitArray::new)
        .collect();

    let baghash = hasher.hash_bag(&features);

    eprintln!("bag hash: {:02X?}", baghash);
    eprintln!("bag hash weight: {}", baghash.weight());
    baghash.weight()
}

fn main() {
    let hasher = HammingHasher::<32, 32>::new();
    let mut total_weight = 0;
    let num = 128;
    for seed in 0..num {
        total_weight += test_balance_on::<32, 32>(&hasher, seed);
    }
    eprintln!("average hash weight: {}", total_weight as f64 / num as f64);
}
