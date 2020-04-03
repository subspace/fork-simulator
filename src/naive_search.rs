#![allow(dead_code)]

mod lib;
use lib::*;

pub const PLOT_SEED: &[u8] = "SUBSPACE".as_bytes();
pub const CHALLENGE_SEED: &[u8] = "LEDGER".as_bytes();
pub const PIECE_COUNT: usize = 256 * 1_000;
pub const CHALLENGE_COUNT: usize = 10;

fn main() {
    println!("Hello, Subspace!");

    // init empty hash array for storing piece hashes
    let mut hash_array: [[u8; 8]; PIECE_COUNT] = [[0u8; 8]; PIECE_COUNT];

    // create a pseudo random plot of 8 byte piece hashes
    for piece_index in 0..PIECE_COUNT {
      let piece_index_bytes = usize_to_bytes(piece_index); 
      let piece = [&PLOT_SEED[..], &piece_index_bytes[..]].concat();
      hash_array[piece_index] = digest_sha_256_truncated(&piece[..]);
    }

    println!("Plotted {} pieces", PIECE_COUNT);

    for challenge_index in 0..CHALLENGE_COUNT {
      let mut quality_array: [usize; 65] = [0usize; 65];
      let challenge = digest_sha_256_truncated(&[
        &CHALLENGE_SEED[..],
        &usize_to_bytes(challenge_index)[..]
      ].concat());

      let mut best_quality = 0;

      for piece_index in 0..PIECE_COUNT {
        let piece_hash = hash_array[piece_index];
        let quality = measure_xor_distance(&challenge[..], &piece_hash[..]);
        quality_array[quality as usize] += 1;
        if quality > best_quality {
          best_quality = quality;
        }
      }

      // print the quality distribution 
      println!("\nQuality Distribution for this trial is... \n");
      for quality in 0..32 {
        println!("Quality {}:\t {}", quality, quality_array[quality as usize]);
      }
    }
}
