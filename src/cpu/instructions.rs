pub enum Instruction {
  ADD(ArithmeticTarget),
  LDN16(LoadTypeN16),
  XOR(ArithmeticTarget, ArithmeticSource),
}

pub enum ArithmeticTarget {
  A, B, C, D, E, H, L
}

pub enum ArithmeticSource {
  A, B, C, D, E, H, L, HLI
}

pub enum LoadTypeN16 {
	SP
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
    	0x31 => Some(Instruction::LDN16(LoadTypeN16::SP)),
    	0xAF => Some(Instruction::XOR(ArithmeticTarget::A, ArithmeticSource::A)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}
