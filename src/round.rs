use constants::*;

macro_rules! index {
  ($x:expr, $y:expr) => {
      ((($x) % 5) + 5 * (($y) % 5))
  };
}

pub fn round(mut state: &mut Vec<u64>, round_constant: u64) {
  
  theta(&mut state);
  // println!("State θ: {:?}", state);
  rho(&mut state);
  pi(&mut state);
  // println!("State ρπ: {:?}", state);
  chi(&mut state);
  // println!("State χ: {:?}", state);
  iota(&mut state, round_constant);
  // println!("State ι: {:?}", state);

}

fn theta(state: &mut Vec<u64>) {
  // θ
  let mut c = vec![0u64; 5];
  let mut d = vec![0u64; 5];

  for x in 0..5 {
    c[x] = state[index!(x,0)] ^ state[index!(x,1)] ^ state[index!(x,2)] ^ state[index!(x,3)] ^ state[index!(x,4)];
  }

  for x in 0..5 {
    d[x] = c[(x+4)%5] ^ c[(x+1)%5].rotate_left(1);
  }

  for x in 0..5 {
    for y in 0..5 {
      state[index!(x,y)] ^= d[x];
    }
  }
}

fn rho(state: &mut Vec<u64>) {
  // ρ
  for x in 0..5 {
    for y in 0..5 {
        state[index!(x,y)] = state[index!(x,y)].rotate_left(ROTATION_CONSTANTS[x][y]);
    }
  }
}

fn pi(mut state: &mut Vec<u64>) {
  // π
  let mut temp_state = init_slice();

  for x in 0..5 {
    for y in 0..5 {
      temp_state[index!(x,y)] = state[index!(x,y)].clone();
    }
  }

  for x in 0..5 {
    for y in 0..5 {
      state[index!((0*x + 1*y),(2*x + 3*y))] = temp_state[index!(x,y)].clone();
    }
  }
}

fn chi(mut state: &mut Vec<u64>) {
  // χ
  let mut temp_state = init_slice();

  for x in 0..5 {
    for y in 0..5 {
      temp_state[index!(x,y)] = state[index!(x,y)];
    }
  }

  for x in 0..5 {
    for y in 0..5 {
      state[index!(x,y)] = temp_state[index!(x,y)] ^ (!temp_state[index!(x+1,y)] & temp_state[index!(x+2,y)]);
    }
  }
}

fn iota(mut state: &mut Vec<u64>, rc: u64) {
  //ι
  state[0] ^= rc;
}

fn init_slice() -> Vec<u64> {
  vec![0u64; 25]
}