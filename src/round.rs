use constants::*;

macro_rules! index {
  ($x:expr, $y:expr) => {
      ((($x) % 5) + 5 * (($y) % 5))
  };
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