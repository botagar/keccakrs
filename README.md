# This code is BROKEN! Please do not use just yet!

Add to Cargo.toml

    [dependencies]
    keccakrs = "*"

In your code, use as follow:

    extern crate keccakrs;
    use keccakrs::*;

    fn main() {
      let mut keccak = keccakrs::Keccak::new(1088usize, 512usize);
      /* Other options include:
      *  keccak = keccakrs::new_keccak1600_224();
      *  keccak = keccakrs::new_keccak1600_256();
      *  keccak = keccakrs::new_keccak1600_384();
      *  keccak = keccakrs::new_keccak1600_512();
      */

      let input = &mut String::from("Hello World");
      
      keccak.injest(input);

      let output: Vec<u8> = keccak.hash();
      let state: Vec<u64> = keccak.get_internal_state();
    }

This package is still under heavy development and is ~~untested~~ broken.

The primary goal of this code base is to present a easy to read and understand implementation of Keccak.

(This is not a particularly fast implementation of Keccak, but it utilises NO unsafe code)
