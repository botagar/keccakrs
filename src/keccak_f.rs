use constants::*;
use round::*;

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
