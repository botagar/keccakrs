#![crate_type = "lib"]
#![crate_name = "keccakrs"]

mod constants;
mod round;
mod keccak_f;
mod sponge;
mod padder;

use constants::*;
use round::*;
use keccak_f::*;
use sponge::*;
use padder::*;

pub fn new_keccak1600_224() -> Keccak { Keccak::new(1152, 448) }
pub fn new_keccak1600_256() -> Keccak { Keccak::new(1088, 512) }
pub fn new_keccak1600_384() -> Keccak { Keccak::new(832, 768) }
pub fn new_keccak1600_512() -> Keccak { Keccak::new(576, 1024) }

pub struct Keccak {
  state: Vec<u64>,
  data_queue: Vec<u8>,
  sponge: Sponge,
  input_padder: Padder
}

impl Keccak {

  pub fn new(rate: usize, capacity: usize) -> Keccak {
    Keccak {
      state: Keccak::init_state(),
      sponge: Sponge::new(rate, capacity),
      input_padder: Padder::new(rate),
      data_queue: vec![]
    }
  }

  pub fn injest(&mut self, input: &mut String) {
    let mut input_as_bytes: Vec<u8> = input.clone().into_bytes();
    self.input_padder.pad(&mut input_as_bytes);
    self.data_queue = input_as_bytes;
  }

  pub fn hash(&mut self) -> Vec<u8> {
    self.sponge.absorb(&mut self.state, self.data_queue.clone());
    self.sponge.squeeze(&mut self.state)
  }

  pub fn get_internal_state(&mut self) -> Vec<u64> {
    self.state.clone()
  }

  fn init_state() -> Vec<u64> {
    vec![0u64; 25]
  }
}