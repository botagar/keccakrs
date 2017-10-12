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

const LONG_INPUT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec non faucibus arcu, ut congue velit.  \
 Mauris suscipit posuere rutrum. Donec vel volutpat mi. Donec vulputate velit eu erat semper, eget feugiat leo tempus. \
   Donec in eros nec mi venenatis scelerisque eget in erat. Curabitur nec pharetra nulla. Aenean fermentum risus id ipsum gravida mattis.";
const EXPECTED_RESULTS_LONG_INPUT: [&'static str; 4] = [
    "af43ba64d02c30606c4159824c0c963c59062a66df21cd80c49800a7",
    "9317b02a982da9ce0736d51914b0948613738b1c26ae3f5d3eb6e9d729c1ae85",
    "c9846d5a5c4f27100671b76b32f15ca1b11b8e2a4707b9e04d230dab879cb62c097fc780fe44b7cf6c07691913eedaf2",
    "25cdf5af94b2e97b8ca9d4282b025598852497336e27d0861a1c7a572c78996881bd6f0ee85a321a1ab96ab4e4821ac36c1179286d89cdb04575152133024957"
];

#[test]
fn k_224_empty_input() {
    let mut keccak = Keccak::new_keccak1600_224();
    keccak.injest(&mut String::from(EMPTY_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[0]));
}

// #[test]
// fn k_224_short_input() {
//     let mut keccak = Keccak::new_keccak1600_224();
//     keccak.injest(&mut String::from(SHORT_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[0]));
// }

// #[test]
// fn k_224_long_input() {
//     let mut keccak = Keccak::new_keccak1600_224();
//     keccak.injest(&mut String::from(LONG_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[0]));
// }

#[test]
fn k_256_empty_input() {
    let mut keccak = Keccak::new_keccak1600_256();
    keccak.injest(&mut String::from(EMPTY_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[1]));
}

#[test]
fn k_256_short_input() {
    let mut keccak = Keccak::new_keccak1600_256();
    keccak.injest(&mut String::from(SHORT_INPUT));
    assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[1]));
}

// #[test]
// fn k_256_long_input() {
//     let mut keccak = Keccak::new_keccak1600_256();
//     keccak.injest(&mut String::from(LONG_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[1]));
// }

// #[test]
// fn k_384_empty_input() {
//     let mut keccak = Keccak::new_keccak1600_384();
//     keccak.injest(&mut String::from(EMPTY_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[2]));
// }

// #[test]
// fn k_384_short_input() {
//     let mut keccak = Keccak::new_keccak1600_384();
//     keccak.injest(&mut String::from(SHORT_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[2]));
// }

// #[test]
// fn k_384_long_input() {
//     let mut keccak = Keccak::new_keccak1600_384();
//     keccak.injest(&mut String::from(LONG_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[2]));
// }

// #[test]
// fn k_512_empty_input() {
//     let mut keccak = Keccak::new_keccak1600_512();
//     keccak.injest(&mut String::from(EMPTY_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_EMPTY_INPUT[3]));
// }

// #[test]
// fn k_512_short_input() {
//     let mut keccak = Keccak::new_keccak1600_512();
//     keccak.injest(&mut String::from(SHORT_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_SHORT_INPUT[3]));
// }

// #[test]
// fn k_512_long_input() {
//     let mut keccak = Keccak::new_keccak1600_512();
//     keccak.injest(&mut String::from(LONG_INPUT));
//     assert_eq!(vec_to_hex_string(keccak.hash()), String::from(EXPECTED_RESULTS_LONG_INPUT[3]));
// }

fn vec_to_hex_string(bytes: Vec<u8>) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
  strs.join("")
}