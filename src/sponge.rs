use keccak_f::*;

pub struct Sponge {
  rate_in_bytes: usize,
  capacity: usize,
  keccak1600: KeccakF
}

impl Sponge {
  pub fn new(rate: usize, capacity: usize) -> Sponge {
    Sponge {
      rate_in_bytes: rate/8,
      capacity: capacity,
      keccak1600: KeccakF::new(1600)
    }
  }

  pub fn absorb(&self, mut state: &mut [u64], data: &[u8]) {
    assert!(data.len() % self.rate_in_bytes == 0, "Data padding error!");

    let number_of_blocks: usize = data.len() / self.rate_in_bytes;
    let overflow: usize = data.len() % 8;

    // // Cut u8 array into u64 chunks
    let lanes: Vec<u64> = Sponge::cut_into_lanes(data);

    for block_index in 0..number_of_blocks {
      let block_offset: usize = block_index * (self.rate_in_bytes/8);
      for i in 0..(self.rate_in_bytes/8) {
        state[i] ^= lanes[block_offset + i];
      }
      self.keccak1600.process(&mut state);
    }
  }

  pub fn squeeze(&self, state: &mut [u64]) -> Vec<u8> {
    let no_of_output_bytes: usize = (self.capacity / 2) / 8;
    let mut output_bytes: Vec<u8> = vec![];
    let no_of_lanes: usize = no_of_output_bytes / 8;
    let overlow_bytes: usize = no_of_output_bytes % 8;

    let mut lanes_used: usize = 0;
    for lane in state {
      if lanes_used == no_of_lanes {
        if overlow_bytes != 0 {
          output_bytes.append(&mut Sponge::lane_to_bytes_with_offset(&lane, overlow_bytes));
        }
        break
      }
      output_bytes.append(&mut Sponge::lane_to_bytes_with_offset(&lane, 0));
      lanes_used += 1;
    }

    output_bytes
  }

  fn lane_to_bytes_with_offset(lane: &u64, offset: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    for i in (0..(8)).rev() {
      let shift_constant: usize = (7-i)*8;
      let bitmask: u64 = 0xFF << shift_constant;
      let long_word: u64 = *lane & bitmask;
      let word = (long_word >> shift_constant) as u8;
      bytes.push(word);
      if i-offset == 0 { break }
    }
    bytes
  }

  fn cut_into_lanes(data: &[u8]) -> Vec<u64> {
    let mut lanes: Vec<u64> = vec![];
    let mut lane: u64 = 0;
    let mut next_8_chars: Vec<u8>;
    let no_of_lanes = data.len() / 8;

    for i in 0..(no_of_lanes) {
      next_8_chars = data[(i*8)..((i*8)+8)].to_vec();
      for j in (0..8).rev() {
        lane <<= 8;
        lane |= next_8_chars[j] as u64;
      }
      lanes.push(lane);
    }
    lanes
  }
}
