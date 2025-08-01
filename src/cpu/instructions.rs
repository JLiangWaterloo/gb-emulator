pub enum Instruction {
    ADD(ArithmeticSource),
    BIT(u8, BitSource),
    CALL,
    CP(CompareSource),
    DEC(DecrementTarget),
    INC(IncTarget),
    JR(JumpCondition),
    LD(LoadTarget, LoadSource),
    LDH(LoadHTarget, LoadHSource),
    LDN16(LoadTypeN16),
    NOP,
    POP(PopTarget),
    PUSH(PushTarget),
    RET,
    RL(RotateTarget),
    RLA,
    SUB(ArithmeticSource),
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
    HL_,
    HLI,
}

pub enum BitSource {
    H,
}

pub enum CompareSource {
    HL_,
    N8,
}

pub enum DecrementTarget {
    A,
    B,
    C,
    D,
    E,
}

pub enum IncTarget {
    B,
    C,
    H,
    DE,
    HL,
}

pub enum JumpCondition {
    Always,
    NZ,
    Z,
}

pub enum LoadTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL_,
    HLD,
    HLI,
    N16_,
}

pub enum LoadSource {
    A,
    B,
    E,
    H,
    L,
    DE_,
    N8,
}

pub enum LoadHTarget {
    A,
    C_,
    N8_,
}

pub enum LoadHSource {
    A,
    N8_,
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
            0x0 => Some(Instruction::NOP),
            0x4 => Some(Instruction::INC(IncTarget::B)),
            0x5 => Some(Instruction::DEC(DecrementTarget::B)),
            0x6 => Some(Instruction::LD(LoadTarget::B, LoadSource::N8)),
            0xc => Some(Instruction::INC(IncTarget::C)),
            0xd => Some(Instruction::DEC(DecrementTarget::C)),
            0xe => Some(Instruction::LD(LoadTarget::C, LoadSource::N8)),
            0x11 => Some(Instruction::LDN16(LoadTypeN16::DE)),
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            0x15 => Some(Instruction::DEC(DecrementTarget::D)),
            0x16 => Some(Instruction::LD(LoadTarget::D, LoadSource::N8)),
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR(JumpCondition::Always)),
            0x1a => Some(Instruction::LD(LoadTarget::A, LoadSource::DE_)),
            0x1d => Some(Instruction::DEC(DecrementTarget::E)),
            0x1e => Some(Instruction::LD(LoadTarget::E, LoadSource::N8)),
            0x20 => Some(Instruction::JR(JumpCondition::NZ)),
            0x21 => Some(Instruction::LDN16(LoadTypeN16::HL)),
            0x22 => Some(Instruction::LD(LoadTarget::HLI, LoadSource::A)),
            0x23 => Some(Instruction::INC(IncTarget::HL)),
            0x24 => Some(Instruction::INC(IncTarget::H)),
            0x28 => Some(Instruction::JR(JumpCondition::Z)),
            0x2e => Some(Instruction::LD(LoadTarget::L, LoadSource::N8)),
            0x31 => Some(Instruction::LDN16(LoadTypeN16::SP)),
            0x32 => Some(Instruction::LD(LoadTarget::HLD, LoadSource::A)),
            0x3d => Some(Instruction::DEC(DecrementTarget::A)),
            0x3e => Some(Instruction::LD(LoadTarget::A, LoadSource::N8)),
            0x4f => Some(Instruction::LD(LoadTarget::C, LoadSource::A)),
            0x57 => Some(Instruction::LD(LoadTarget::D, LoadSource::A)),
            0x67 => Some(Instruction::LD(LoadTarget::H, LoadSource::A)),
            0x77 => Some(Instruction::LD(LoadTarget::HL_, LoadSource::A)),
            0x78 => Some(Instruction::LD(LoadTarget::A, LoadSource::B)),
            0x7b => Some(Instruction::LD(LoadTarget::A, LoadSource::E)),
            0x7c => Some(Instruction::LD(LoadTarget::A, LoadSource::H)),
            0x7d => Some(Instruction::LD(LoadTarget::A, LoadSource::L)),
            0x86 => Some(Instruction::ADD(ArithmeticSource::HL_)),
            0x90 => Some(Instruction::SUB(ArithmeticSource::B)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A, ArithmeticSource::A)),
            0xbe => Some(Instruction::CP(CompareSource::HL_)),
            0xc1 => Some(Instruction::POP(PopTarget::BC)),
            0xc5 => Some(Instruction::PUSH(PushTarget::BC)),
            0xc9 => Some(Instruction::RET),
            0xcd => Some(Instruction::CALL),
            0xe0 => Some(Instruction::LDH(LoadHTarget::N8_, LoadHSource::A)),
            0xe2 => Some(Instruction::LDH(LoadHTarget::C_, LoadHSource::A)),
            0xea => Some(Instruction::LD(LoadTarget::N16_, LoadSource::A)),
            0xf0 => Some(Instruction::LDH(LoadHTarget::A, LoadHSource::N8_)),
            0xfe => Some(Instruction::CP(CompareSource::N8)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
