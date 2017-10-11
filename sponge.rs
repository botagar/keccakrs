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
      keccak1600: KeccakF::new(1600,rate,capacity)
    }
  }

  pub fn absorb(&self, mut state: &mut Vec<u64>, data: Vec<u8>) {
    assert!(data.len() % self.rate_in_bytes == 0, "Data padding error!");

    let number_of_blocks: usize = data.len() / self.rate_in_bytes;

    // Cut u8 array into u64 chunks
    let lanes: Vec<u64> = Sponge::cut_into_lanes(data);

    for block_index in 0..number_of_blocks {
      let block_offset: usize = block_index * 17;
      for i in 0..17 {
        state[i] ^= lanes[block_offset + i];
      }
      self.keccak1600.process(&mut state);
    }
  }

  pub fn squeeze(&self, state: &mut Vec<u64>) -> Vec<u8> {
    let no_of_output_bytes: usize = (self.capacity / 2) / 8;
    let mut output_bytes: Vec<u8> = vec![];

    for lane in state {
      // Cut u64 into 8 u8's
      for i in (0..8).rev() {
        let shift_constant: usize = (7-i)*8;
        let bitmask: u64 = 0xFF << shift_constant;
        let long_word: u64 = *lane & bitmask;
        let word = (long_word >> shift_constant) as u8;
        output_bytes.push(word);
      }
      if output_bytes.len() == no_of_output_bytes { break }
    }

    output_bytes
  }

  fn cut_into_lanes(data: Vec<u8>) -> Vec<u64> {
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
