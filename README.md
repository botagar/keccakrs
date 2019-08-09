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

This package is under development. Use at own risk.

The primary goal of this code base is to present a easy to read and understand implementation of Keccak.

(This is not a particularly fast implementation of Keccak, but it utilises NO unsafe code)

    test perf_k_256_empty_input ... bench:       1,244 ns/iter (+/- 99)
    test perf_k_256_long_input  ... bench:       3,400 ns/iter (+/- 196)
    test perf_k_512_empty_input ... bench:       1,400 ns/iter (+/- 109)
    test perf_k_512_long_input  ... bench:       5,713 ns/iter (+/- 731)
