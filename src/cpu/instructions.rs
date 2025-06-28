pub enum Instruction {
  ADD(ArithmeticTarget),
  BIT(u8, BitSource),
  JR(JumpCondition),
  LD(LoadTarget, LoadSource),
  LDH(LoadHTarget, LoadHSource),
  LDN16(LoadTypeN16),
  XOR(ArithmeticTarget, ArithmeticSource),
}

pub enum ArithmeticTarget {
  A, B, C, D, E, H, L
}

pub enum ArithmeticSource {
  A, B, C, D, E, H, L, HLI
}

pub enum BitSource {
	H
}

pub enum JumpCondition {
	NZ
}

pub enum LoadTarget {
	A, C, HLD
}

pub enum LoadSource {
	A, N8
}

pub enum LoadHTarget {
	C_ 
}

pub enum LoadHSource {
	A
}

pub enum LoadTypeN16 {
	SP, HL
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
      0x7c => Some(Instruction::BIT(7, BitSource::H)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
    	0xe => Some(Instruction::LD(LoadTarget::C, LoadSource::N8)),
    	0x20 => Some(Instruction::JR(JumpCondition::NZ)),
    	0x21 => Some(Instruction::LDN16(LoadTypeN16::HL)),
    	0x31 => Some(Instruction::LDN16(LoadTypeN16::SP)),
    	0x32 => Some(Instruction::LD(LoadTarget::HLD, LoadSource::A)),
    	0x3e => Some(Instruction::LD(LoadTarget::A, LoadSource::N8)),
    	0xAF => Some(Instruction::XOR(ArithmeticTarget::A, ArithmeticSource::A)),
    	0xe2 => Some(Instruction::LDH(LoadHTarget::C_, LoadHSource::A)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}
