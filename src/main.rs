#![allow(dead_code)]

mod lib;
use lib::*;
use s_tree::STree;
use std::thread::sleep;
use std::time::Duration;

pub const SEED: &[u8] = "SUBSPACE".as_bytes();
pub const PIECE_COUNT: usize = 256 * 1_000;
pub const SOLUTION_COUNT: usize = 3;
pub const COMMON_BITS: usize = 15;
pub const QUALITY_TARGET: usize = 18;
pub const BASE_TIME: usize = 300;
pub const SCALE_FACTOR: usize = 4;

// ToDo
  // print heads on interval
    // sort the heads by the three chains with the highest quality
    // can you later prune heads to reduce memory load?
  // add an async timeout on a shared mutable data structure
    // have to make s-tree read only 
    // blocks must be shared mutable
    // heads must be shared mutable
  

#[derive(Clone, Debug)]
struct Block {
  // index (get from vec)
  parent: usize, // index of parent block in the block_vecs
  quality: usize, // quality of this block for challenge (hash of parent block)
}

struct Head {
  // index (get from vec)
  parent: usize,  // index of parent block in block_vecs
  quality: usize, // total quality of all blocks in this branch
}

type BST = STree<u64, u32>;

fn main() {
    println!("Hello, Subspace!");

    // init empty b-tree for storing encoding hashes and indices
    let mut s_tree = BST::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut heads: Vec<Head> = Vec::new();

    // create a pseudo random plot for desired number of pieces
    for piece_index in 0..PIECE_COUNT {
      let piece_index_bytes = usize_to_bytes(piece_index); 
      let piece = [&SEED[..], &piece_index_bytes[..]].concat();
      let piece_hash = digest_sha_256_truncated(&piece[..]);
      s_tree.insert(u64::from_be_bytes(piece_hash), piece_index as u32);
    }
    
    println!("Completed plotting!");

    // let target =  u64::from_be_bytes(digest_sha_256_truncated(&usize_to_bytes(0)));
    // solve(&target, &s_tree, &mut blocks, &mut heads);

    let genesis_block = Block {
      parent: 0,
      quality: 0,
    };

    blocks.push(genesis_block.clone());

    heads.push(Head {
      parent: 0,
      quality: 0,
    });

    println!("Created the genesis block and head");
  
    let challenge = digest_sha_256_truncated(&[
      &SEED[..],
      &genesis_block.parent.to_be_bytes()
    ].concat());

    solve(&u64::from_be_bytes(challenge), &s_tree, &mut blocks, &mut heads);

    // solve the genesis challenge
      // take top three solutions
      // add three blocks
      // create two new heads
      // for each head
        // take top three solutions
        // add three new blocks
        // add two new heads

    
}

/// find the best solutions for each challenge and extend the chain/s
fn solve(target: &u64, s_tree: & BST, blocks: &mut Vec<Block>, heads: &mut Vec<Head>) {

  // find all solutions for the N highest qualities
  let mut quality_sets = s_tree.find_best_grouped(target, COMMON_BITS as u64);

  for solution_index in 0..SOLUTION_COUNT {
    match quality_sets.pop() {
      None => {
        print!("Unable to extend the chain at solution index {}", solution_index);
        break;
      },
      Some(quality_set) => {
        for solution in quality_set.iter() {

          // calculate the quality and delay
          let quality = measure_xor_distance(&target.to_be_bytes(), &solution.0.to_be_bytes());
          println!("Found solution to block {} with quality {}", target, quality);

          // have to call await here or use call back
          let delay = prove(quality);
          println!("Computed proof of time in {} ms", delay);

          let block = Block {
            parent: *target as usize,
            quality: quality as usize
          };

          // add the new block
          blocks.push(block.clone());

          println!("Added block {} to chain", blocks.len());

          // add or extend head
          let base_head = heads.get_mut(0).unwrap();
          base_head.parent = *target as usize;
          base_head.quality += block.quality;

          println!("Quality of the main branch is {}", base_head.quality);
          
          // call solve again with new target
          let challenge = digest_sha_256_truncated(&[
            &SEED[..],
            &block.parent.to_be_bytes()
          ].concat());

          println!("Solving the next block\n");

          solve(&u64::from_be_bytes(challenge), &s_tree, blocks, heads);

          break;
        }
      }
    }
  }

  // get the heads
  // for each head
    // derive the challenge
    // solve


  
  let mut challenges: Vec<[u8; 8]> = Vec::new();
  for head in heads.iter() {
    let challenge = digest_sha_256_truncated(&[
      &SEED[..],
      &head.parent.to_be_bytes()
    ].concat());
    challenges.push(challenge);
  }

  for challenge in challenges.iter() {
    solve(&u64::from_be_bytes(*challenge), &s_tree, blocks, heads);
  }  
}

// given a quality and difficulty params, compute the time delay and wait
fn prove(quality: usize) -> usize {

  let mut delay: usize = 0;

  if quality < QUALITY_TARGET {
    // double the base time for each step down in quality
    delay = (QUALITY_TARGET - quality) * BASE_TIME * SCALE_FACTOR;
  } else if quality == QUALITY_TARGET {
    // time is base time
    delay = BASE_TIME;
  } else if quality > QUALITY_TARGET {
    // halve the base time for each step up in quality
    delay = BASE_TIME / ((quality - QUALITY_TARGET) * SCALE_FACTOR)
  }

  // set an async timeout
  // println!("Computing proof of time...");
  // sleep(Duration::from_millis(delay as u64));
  delay
}

fn read() {
  // print a table of all heads and their qualities 
  // this will grow very fast
  // may need to print the top X heads instead
  // must be sure to know their index to see if/how they change
}

fn add_block() {
  // define a block type or struct
  // init an empty array of that type
  // for each new block
    // add the block to that array
    // check best child for that parent
      // if this quality is better, then extend head and solve
      // else add head and solve
}

fn extend_head() {
  // update the lead block for an existing head
  // what to do in case of a tie?
  // take the block closer to the node id of the node (how to simulate)
}

fn add_head() {
  // define a head type or struct
  // init an empty array of that type

  // add a new head to an existing head 
}
