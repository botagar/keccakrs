#![feature(test)]

extern crate test;
use test::Bencher;
extern crate keccakrs;
use keccakrs::*;

const LONG_INPUT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[bench]
fn a_stabilising_wait(b: &mut Bencher) {
  b.iter(|| {
    1 == 2
  });
}

#[bench]
fn perf_k_256_empty_input(b: &mut Bencher) {
  let mut keccak = new_keccak1600_256();
  
  b.iter(|| {
    keccak.injest(&mut String::from(""));
    keccak.hash();
  });
}

#[bench]
fn perf_k_256_long_input(b: &mut Bencher) {
  let mut keccak = new_keccak1600_256();
  
  b.iter(|| {
    keccak.injest(&mut String::from(LONG_INPUT));
    keccak.hash();
  });
}

#[bench]
fn perf_k_512_empty_input(b: &mut Bencher) {
  let mut keccak = new_keccak1600_512();
  
  b.iter(|| {
    keccak.injest(&mut String::from(""));
    keccak.hash();
  });
}

#[bench]
fn perf_k_512_long_input(b: &mut Bencher) {
  let mut keccak = new_keccak1600_512();
  
  b.iter(|| {
    keccak.injest(&mut String::from(LONG_INPUT));
    keccak.hash();
  });
}