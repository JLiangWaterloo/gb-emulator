pub struct Registers {
  pub a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
  h: u8,
  l: u8,
}

impl Registers {
  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8
    | self.c as u16
  }

  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0xFF) as u8;
  }
}

impl Default for Registers {
	fn default() -> Self {
		Self { a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0 }
	}
}
