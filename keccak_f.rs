use constants::*;
use round::*;

pub struct KeccakF {
  width: usize,
  rate: usize,
  capacity: usize,
  l: usize,
  w: usize,
  n: usize
}

impl KeccakF {
  pub fn new(width: usize, rate: usize, capacity: usize) -> KeccakF {
    let l_val: usize = 6; //((64 as f32).log10()/(2 as f32).log10()) as u32;
    KeccakF {
      width: width,
      rate: rate,
      capacity: capacity,
      l: l_val,
      w: 2usize.pow(l_val as u32),
      n: 12 + (2*l_val)
    }
  }

  pub fn process(&self, mut state: &mut Vec<u64>) {
    // println!("Processing with parameters: b={} c={}, r={}, l={}, w={}, n={}",self.width,self.capacity,self.rate,self.l,self.w,self.n);

    for i in 0..self.n {
      round(&mut state, ROUND_CONSTANTS[i]);
    }
  }
}
