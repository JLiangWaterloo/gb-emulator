pub enum Instruction {
    ADD(ArithmeticTarget),
    BIT(u8, BitSource),
    CALL,
    INC(IncTarget),
    JR(JumpCondition),
    LD(LoadTarget, LoadSource),
    LDH(LoadHTarget, LoadHSource),
    LDN16(LoadTypeN16),
    XOR(ArithmeticTarget, ArithmeticSource),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum ArithmeticSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum BitSource {
    H,
}

pub enum IncTarget {
    C,
}

pub enum JumpCondition {
    NZ,
}

pub enum LoadTarget {
    A,
    B,
    C,
    HL_,
    HLD,
}

pub enum LoadSource {
    A,
    DE_,
    N8,
}

pub enum LoadHTarget {
    C_, N8_,
}

pub enum LoadHSource {
    A,
}

pub enum LoadTypeN16 {
		DE,
    SP,
    HL,
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
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
        	0x6 => Some(Instruction::LD(LoadTarget::B, LoadSource::N8)),
            0xc => Some(Instruction::INC(IncTarget::C)),
            0xe => Some(Instruction::LD(LoadTarget::C, LoadSource::N8)),
            0x11 => Some(Instruction::LDN16(LoadTypeN16::DE)),
            0x1a => Some(Instruction::LD(LoadTarget::A, LoadSource::DE_)),
            0x20 => Some(Instruction::JR(JumpCondition::NZ)),
            0x21 => Some(Instruction::LDN16(LoadTypeN16::HL)),
            0x31 => Some(Instruction::LDN16(LoadTypeN16::SP)),
            0x32 => Some(Instruction::LD(LoadTarget::HLD, LoadSource::A)),
            0x3e => Some(Instruction::LD(LoadTarget::A, LoadSource::N8)),
            0x4f => Some(Instruction::LD(LoadTarget::C, LoadSource::A)),
            0x77 => Some(Instruction::LD(LoadTarget::HL_, LoadSource::A)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A, ArithmeticSource::A)),
            0xcd => Some(Instruction::CALL),
            0xe0 => Some(Instruction::LDH(LoadHTarget::N8_, LoadHSource::A)),
            0xe2 => Some(Instruction::LDH(LoadHTarget::C_, LoadHSource::A)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
