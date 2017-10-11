Add to Cargo.toml

keccakrs = "*"

In your code, use as follow:

    extern crate keccakrs;
    use keccakrs::*;

    fn main() {
      let mut keccak = Keccak::new(1088usize, 512usize);

      let input = &mut String::from("Hello World");
      
      keccak.injest(input);
      let output: Vec<u8> = keccak.hash();
      let state: Vec<u64> = keccak.get_internal_state();
    }
