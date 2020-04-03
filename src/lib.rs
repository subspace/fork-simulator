use ring::digest;
use std::io::Write;
use bitintr::Lzcnt;


// return 8 byte hash of a slice 
pub fn digest_sha_256_truncated(data: &[u8]) -> [u8; 8] {
  let mut array = [0u8; 8];
  let hash = digest::digest(&digest::SHA256, data).as_ref().to_vec();
  array.copy_from_slice(&hash[0..8]);
  array
}

// convert a usize integer to 16 bytes
pub fn usize_to_bytes(number: usize) -> [u8; 16] {
  let mut iv = [0u8; 16];
  iv.as_mut()
      .write_all(&(number as u32).to_be_bytes())
      .unwrap();
  iv
}

pub fn measure_quality(tag: &[u8]) -> usize {
  let mut quality: u8 = 0;
  for byte in tag.iter() {
      let zero_bits = byte.lzcnt();
      quality += zero_bits;
      if zero_bits < 8 {
          break;
      }
  }
  quality as usize
}

pub fn measure_xor_distance(a: &[u8], b: &[u8]) -> usize {
  let mut quality: u8 = 0;
  for byte_index in 0..8 {
    // xor a and b 
    let xor = a[byte_index] ^ b[byte_index];
    let zero_bits = xor.lzcnt();
    quality += zero_bits;
    if zero_bits < 8 {
        break;
    }
  }
  quality as usize
}