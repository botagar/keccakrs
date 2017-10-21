#![crate_type = "lib"]
#![crate_name = "keccakrs"]

#[macro_use] extern crate crunchy;

macro_rules! index {
  ($x:expr, $y:expr) => {
      ((($x) % 5) + 5 * (($y) % 5))
  };
}

static ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082,
    0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088,
    0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B,
    0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080,
    0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080,
    0x0000000080000001, 0x8000000080008008,
];

static ROTATION_CONSTANTS: [[u32; 5]; 5] = [
  [0, 36, 3, 41, 18],
  [1, 44, 10, 45, 2],
  [62, 6, 43, 15, 61],
  [28, 55, 25, 21, 56],
  [27, 20, 39, 8, 14]
];

pub fn new_keccak1600_224() -> Keccak { Keccak::new(1152, 448) }
pub fn new_keccak1600_256() -> Keccak { Keccak::new(1088, 512) }
pub fn new_keccak1600_384() -> Keccak { Keccak::new(832, 768) }
pub fn new_keccak1600_512() -> Keccak { Keccak::new(576, 1024) }

pub struct Keccak {
  state: [u64; 25],
  injestion_rate_in_bytes: usize,
  sponge: Sponge,
}

impl Keccak {

  pub fn new(rate: usize, capacity: usize) -> Keccak {
    Keccak {
      state: [0; 25],
      injestion_rate_in_bytes: rate / 8,
      sponge: Sponge::new(rate, capacity),
    }
  }

  pub fn injest(&mut self, input: &mut String) {
    let mut input_as_bytes: &[u8] = input.as_bytes();
    let padding_bytes: Vec<u8> = Keccak::generate_padding_bytes(input_as_bytes, self.injestion_rate_in_bytes);
    let data_to_injest: &[u8] = &[input_as_bytes, &padding_bytes].concat();
    self.sponge.absorb(&mut self.state, data_to_injest);
  }

  pub fn hash(&mut self) -> Vec<u8> {
    self.sponge.squeeze(&mut self.state)
  }

  pub fn get_internal_state(&mut self) -> &[u64] {
    &self.state
  }

  fn generate_padding_bytes(input: &[u8], rate: usize) -> Vec<u8> {
    let no_of_padding_bytes = rate - (input.len() % rate);
    let mut padding_bytes = vec![0u8; no_of_padding_bytes];

    if no_of_padding_bytes == 1 {
      padding_bytes[0] = 0x81 as u8;
    } else {
      padding_bytes[0] = 0x01 as u8;
      for i in 1..(no_of_padding_bytes-2) {
        padding_bytes[i] = 0x00 as u8;
      }
      padding_bytes[no_of_padding_bytes-1] = 0x80 as u8;
    }

    padding_bytes
  }
}


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

pub struct KeccakF {
  n: usize
}

impl KeccakF {
  pub fn new(width: usize) -> KeccakF {
    let w = width / 25;
    let l_val: usize = ((w as f32).log10()/(2 as f32).log10()) as usize; // From eqn: "2.pow(l) = w"
    KeccakF {
      n: 12 + (2*l_val)
    }
  }

  pub fn process(&self, mut state: &mut [u64]) {
    assert!(self.n == 24, "n val has changed");
    unroll! {
      for i in 0..24 {
        round(&mut state, ROUND_CONSTANTS[i]);
      }
    }
  }
}

#[inline]
pub fn round(mut state: &mut [u64], round_constant: u64) {
  
  theta(&mut state);
  rho(&mut state);
  pi(&mut state);
  chi(&mut state);
  iota(&mut state, round_constant);

}

#[inline]
fn theta(state: &mut [u64]) {
  // θ
  let mut c = [0u64; 5];
  let mut d = [0u64; 5];

  unroll! {
    for x in 0..5 {
      c[x] = state[index!(x,0)] ^ state[index!(x,1)] ^ state[index!(x,2)] ^ state[index!(x,3)] ^ state[index!(x,4)];
    }
  }

  unroll! {
    for x in 0..5 {
      d[x] = c[(x+4)%5] ^ c[(x+1)%5].rotate_left(1);
    }
  }

  unroll! {
    for x in 0..5 {
      unroll! {
        for y in 0..5 {
          state[index!(x,y)] ^= d[x];
        }
      }
    }
  }
}

#[inline]
fn rho(state: &mut [u64]) {
  // ρ
  unroll! {
    for x in 0..5 {
      unroll! {
        for y in 0..5 {
            state[index!(x,y)] = state[index!(x,y)].rotate_left(ROTATION_CONSTANTS[x][y]);
        }
      }
    }
  }
}

#[inline]
fn pi(state: &mut [u64]) {
  // π
  let mut temp_state = [0u64; 25];

  for x in 0..5 {
    for y in 0..5 {
      temp_state[index!(x,y)] = state[index!(x,y)];
    }
  }

  unroll! {
    for x in 0..5 {
      unroll! {
        for y in 0..5 {
          state[index!((0*x + 1*y),(2*x + 3*y))] = temp_state[index!(x,y)];
        }
      }
    }
  }
}

#[inline]
fn chi(state: &mut [u64]) {
  // χ
  let mut temp_state = [0u64; 25];

  for x in 0..5 {
    for y in 0..5 {
      temp_state[index!(x,y)] = state[index!(x,y)];
    }
  }

  unroll! {
    for x in 0..5 {
      unroll! {
        for y in 0..5 {
          state[index!(x,y)] = temp_state[index!(x,y)] ^ (!temp_state[index!(x+1,y)] & temp_state[index!(x+2,y)]);
        }
      }
    }
  }
  
}

#[inline]
fn iota(state: &mut [u64], rc: u64) {
  //ι
  state[0] ^= rc;
}