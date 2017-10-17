
pub struct Padder {
  rate_in_bytes: usize
}

impl Padder {
  pub fn new(rate: usize) -> Padder {
    Padder {
      rate_in_bytes: rate/8
    }
  }

  pub fn pad(&self, message: &mut Vec<u8>) {
    let no_of_padding_bytes = self.rate_in_bytes - (message.len() % self.rate_in_bytes);
    if no_of_padding_bytes == 1 {
      message.push(0x81 as u8);
    } else {
      message.push(0x01 as u8);
      for _ in 0..(no_of_padding_bytes-2) {
        message.push(0x00 as u8);
      }
      message.push(0x80 as u8);
    }
  }
}