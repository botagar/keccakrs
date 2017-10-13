extern crate keccakrs;
use keccakrs::*;

const EMPTY_INPUT: &str = "";
const EXPECTED_RESULTS_EMPTY_INPUT: [&'static str; 4] = [
    "f71837502ba8e10837bdd8d365adb85591895602fc552b48b7390abd",
    "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
    "2c23146a63a29acf99e73b88f8c24eaa7dc60aa771780ccc006afbfa8fe2479b2dd2b21362337441ac12b515911957ff",
    "0eab42de4c3ceb9235fc91acffe746b29c29a8c366b7c60e4e67c466f36a4304c00fa9caf9d87976ba469bcbe06713b435f091ef2769fb160cdab33d3670680e"
];

const SHORT_INPUT: &str = "abc";
const EXPECTED_RESULTS_SHORT_INPUT: [&'static str; 4] = [
    "c30411768506ebe1c2871b1ee2e87d38df342317300a9b97a95ec6a8",
    "4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45",
    "f7df1165f033337be098e7d288ad6a2f74409d7a60b49c36642218de161b1f99f8c681e4afaf31a34db29fb763e3c28e",
    "18587dc2ea106b9a1563e32b3312421ca164c7f1f07bc922a9c83d77cea3a1e5d0c69910739025372dc14ac9642629379540c17e2a65b19d77aa511a9d00bb96"
];

const LONG_INPUT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
const EXPECTED_RESULTS_LONG_INPUT: [&'static str; 4] = [
    "33ce2fc65ac662fae17643d08e48572c72b79922aaea5d36d3943c8c",
    "ab93a89616d979f58adda52aa0c56f39ede988128ef47ab8a4d2c3c4d45c5049",
    "117835763a486a1ae0bd23c8ae131696c814043d9e4af224034daaeebcf00b54a11e65609971b2cbd0560e9ddc0f899e",
    "37bde721ca7cc79432c1c3f51a384ce43557c31a06bb9fd18fcd8bff8ab47c4763478b8a1f69fab6675d8d11bec993afe659ef84d6b79be9f07611513f64e961"
];

#[test]
fn k_224_empty_input() {
    let mut keccak = new_keccak1600_224();
    keccak.injest(&mut String::from(EMPTY_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[0]));
}

#[test]
fn k_224_short_input() {
    let mut keccak = new_keccak1600_224();
    keccak.injest(&mut String::from(SHORT_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[0]));
}

#[test]
fn k_224_long_input() {
    let mut keccak = new_keccak1600_224();
    keccak.injest(&mut String::from(LONG_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[0]));
}

#[test]
fn k_256_empty_input() {
    let mut keccak = new_keccak1600_256();
    keccak.injest(&mut String::from(EMPTY_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[1]));
}

#[test]
fn k_256_short_input() {
    let mut keccak = new_keccak1600_256();
    keccak.injest(&mut String::from(SHORT_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[1]));
}

#[test]
fn k_256_long_input() {
    let mut keccak = new_keccak1600_256();
    keccak.injest(&mut String::from(LONG_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[1]));
}

#[test]
fn k_384_empty_input() {
    let mut keccak = new_keccak1600_384();
    keccak.injest(&mut String::from(EMPTY_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[2]));
}

#[test]
fn k_384_short_input() {
    let mut keccak = new_keccak1600_384();
    keccak.injest(&mut String::from(SHORT_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[2]));
}

#[test]
fn k_384_long_input() {
    let mut keccak = new_keccak1600_384();
    keccak.injest(&mut String::from(LONG_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[2]));
}

#[test]
fn k_512_empty_input() {
    let mut keccak = new_keccak1600_512();
    keccak.injest(&mut String::from(EMPTY_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[3]));
}

#[test]
fn k_512_short_input() {
    let mut keccak = new_keccak1600_512();
    keccak.injest(&mut String::from(SHORT_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[3]));
}

#[test]
fn k_512_long_input() {
    let mut keccak = new_keccak1600_512();
    keccak.injest(&mut String::from(LONG_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[3]));
}

fn vec_to_hex_string(bytes: Vec<u8>) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
  strs.join("")
}