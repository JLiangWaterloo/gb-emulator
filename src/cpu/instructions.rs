pub enum Instruction {
    ADD(ArithmeticTarget),
    BIT(u8, BitSource),
    CALL,
    DEC(DecrementTarget),
    INC(IncTarget),
    JR(JumpCondition),
    LD(LoadTarget, LoadSource),
    LDH(LoadHTarget, LoadHSource),
    LDN16(LoadTypeN16),
    POP(PopTarget),
    PUSH(PushTarget),
    RET,
    RL(RotateTarget),
    RLA,
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

pub enum DecrementTarget {
    B,
}

pub enum IncTarget {
    C,
    HL,
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
    HLI,
}

pub enum LoadSource {
    A,
    DE_,
    N8,
}

pub enum LoadHTarget {
    C_,
    N8_,
}

pub enum LoadHSource {
    A,
}

pub enum LoadTypeN16 {
    DE,
    SP,
    HL,
}

pub enum RotateTarget {
    C,
}

pub enum PopTarget {
    BC,
}

pub enum PushTarget {
    BC,
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
            0x11 => Some(Instruction::RL(RotateTarget::C)),
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
            0x5 => Some(Instruction::DEC(DecrementTarget::B)),
            0x6 => Some(Instruction::LD(LoadTarget::B, LoadSource::N8)),
            0xc => Some(Instruction::INC(IncTarget::C)),
            0xe => Some(Instruction::LD(LoadTarget::C, LoadSource::N8)),
            0x11 => Some(Instruction::LDN16(LoadTypeN16::DE)),
            0x17 => Some(Instruction::RLA),
            0x1a => Some(Instruction::LD(LoadTarget::A, LoadSource::DE_)),
            0x20 => Some(Instruction::JR(JumpCondition::NZ)),
            0x21 => Some(Instruction::LDN16(LoadTypeN16::HL)),
            0x22 => Some(Instruction::LD(LoadTarget::HLI, LoadSource::A)),
            0x23 => Some(Instruction::INC(IncTarget::HL)),
            0x31 => Some(Instruction::LDN16(LoadTypeN16::SP)),
            0x32 => Some(Instruction::LD(LoadTarget::HLD, LoadSource::A)),
            0x3e => Some(Instruction::LD(LoadTarget::A, LoadSource::N8)),
            0x4f => Some(Instruction::LD(LoadTarget::C, LoadSource::A)),
            0x77 => Some(Instruction::LD(LoadTarget::HL_, LoadSource::A)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A, ArithmeticSource::A)),
            0xc1 => Some(Instruction::POP(PopTarget::BC)),
            0xc5 => Some(Instruction::PUSH(PushTarget::BC)),
            0xc9 => Some(Instruction::RET),
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
