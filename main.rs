mod constants;
mod round;
mod padder;
mod sponge;
mod keccak;
mod keccak_f;

use keccak::*;

fn main() {
  let mut keccak = Keccak::new(1088usize, 512usize);

  let input = &mut String::from("");
  
  keccak.injest(input);
  let hash_as_vec: Vec<u8> = keccak.hash();
  let state: Vec<u64> = keccak.get_internal_state();


  let s: String = to_hex_string(hash_as_vec.clone());
  println!("Output Hash: {:?}", s);
  println!("Hash as vec: {:?}", hash_as_vec);
  println!("State: {:?}", state);
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
  strs.join("")
}

pub fn to_binary_from_hex(hex_string: String) -> String {
  let no_of_u64_words: usize = hex_string.len() / 16;
  let mut binary_string: String = String::from("");
  for i in 0..no_of_u64_words {
    let n = u64::from_str_radix(&hex_string[(16*i)..(16*i + 16)], 16).unwrap();
    let bin_string = format!("{:064b}", n);
    binary_string = format!("{}{}",binary_string, bin_string);
  }
  binary_string
}
