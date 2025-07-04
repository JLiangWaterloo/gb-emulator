pub mod flag_registers;
pub mod instructions;
pub mod memory_bus;
pub mod registers;

use flag_registers::FlagsRegister;
use instructions::Instruction;
use memory_bus::MemoryBus;
use registers::Registers;

pub struct CPU {
    registers: Registers,
    sp: u16,
    pc: u16,
    flags_register: FlagsRegister,
    bus: MemoryBus,
}

impl CPU {
    pub fn step(&mut self) {
        let old_pc = self.pc;
        let mut instruction_byte = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc);
            self.pc = self.pc.wrapping_add(1);
        }
        println!("Running instruction {:x} @ {}", instruction_byte, old_pc);
        if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description)
        };
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::BIT(number, source) => {
                let source_value = match source {
                    instructions::BitSource::H => self.registers.h,
                    _ => {
                        panic!("TODO: implement other sources")
                    }
                };
                self.flags_register.zero = (source_value >> number) & 1 == 0;
                self.flags_register.subtract = false;
                self.flags_register.half_carry = true;
            }
            Instruction::CALL => {
                let least_significant_byte = self.bus.read_byte(self.pc) as u16;
                let most_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                let value = (most_significant_byte << 8) | least_significant_byte;

                self.pc = self.pc.wrapping_add(2);

                self.sp = self.sp.wrapping_sub(1);
                self.bus.write_byte(self.sp, (self.pc >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.bus.write_byte(self.sp, (self.pc & 0xff) as u8);

                println!("Calling 0x{:x}", value);
                self.pc = value;
            }
            Instruction::CP => {
                let n8 = self.bus.read_byte(self.pc);
                self.pc = self.pc.wrapping_add(1);

                self.flags_register.zero = self.registers.a == n8;
                self.flags_register.subtract = true;
                self.flags_register.half_carry = (self.registers.a & 0xf) < (n8 & 0xf);
                self.flags_register.carry = self.registers.a < n8;
            }
            Instruction::DEC(target) => {
                match target {
                    instructions::DecrementTarget::A => {
                        self.flags_register.half_carry = self.registers.a == 0;
                        self.registers.a = self.registers.a.wrapping_sub(1);
                        self.flags_register.zero = self.registers.a == 0;
                        self.flags_register.subtract = true;
                    }
                    instructions::DecrementTarget::B => {
                        self.flags_register.half_carry = self.registers.b == 0;
                        self.registers.b = self.registers.b.wrapping_sub(1);
                        self.flags_register.zero = self.registers.b == 0;
                        self.flags_register.subtract = true;
                    }
                    instructions::DecrementTarget::C => {
                        self.flags_register.half_carry = self.registers.c == 0;
                        self.registers.c = self.registers.c.wrapping_sub(1);
                        self.flags_register.zero = self.registers.c == 0;
                        self.flags_register.subtract = true;
                    }
                    _ => {
                        panic!("TODO: implement other targets")
                    }
                };
            }
            Instruction::INC(target) => {
                match target {
                    instructions::IncTarget::B => {
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.flags_register.zero = self.registers.b == 0;
                        self.flags_register.subtract = false;
                        self.flags_register.half_carry = self.registers.b == 0x10;
                    }
                    instructions::IncTarget::C => {
                        self.registers.c = self.registers.c.wrapping_add(1);
                        self.flags_register.zero = self.registers.c == 0;
                        self.flags_register.subtract = false;
                        self.flags_register.half_carry = self.registers.c == 0x10;
                    }
                    instructions::IncTarget::DE => {
                        let de = self.registers.get_de();
                        self.registers.set_de(de.wrapping_add(1));
                    }
                    instructions::IncTarget::HL => {
                        let hl = self.registers.get_hl();
                        self.registers.set_hl(hl.wrapping_add(1));
                    }
                    _ => {
                        panic!("TODO: implement other targets")
                    }
                };
            }
            Instruction::JR(condition) => {
                let condition_value = match condition {
                    instructions::JumpCondition::Always => true,
                    instructions::JumpCondition::NZ => !self.flags_register.zero,
                    instructions::JumpCondition::Z => self.flags_register.zero,
                    _ => {
                        panic!("TODO: implement other sources")
                    }
                };
                if condition_value {
                    let distance = self.bus.read_signed_byte(self.pc);
                    println!("Jumping {}", distance);
                    self.pc = self.pc.wrapping_add(1).wrapping_add_signed(distance.into());
                } else {
                    println!("Not jumping");
                    // Optimization: skip reading the next byte since we are not jumping, still need to update pc counter though.
                    self.pc = self.pc.wrapping_add(1);
                }
            }
            Instruction::LD(target, source) => {
                let source_value = match source {
                    instructions::LoadSource::A => self.registers.a,
                    instructions::LoadSource::E => self.registers.e,
                    instructions::LoadSource::DE_ => self.bus.read_byte(self.registers.get_de()),
                    instructions::LoadSource::N8 => {
                        let old_pc = self.pc;
                        self.pc = self.pc.wrapping_add(1);
                        self.bus.read_byte(old_pc)
                    }
                    _ => {
                        panic!("TODO: implement other sources")
                    }
                };
                match target {
                    instructions::LoadTarget::A => {
                        println!("Setting a={}", source_value);
                        self.registers.a = source_value;
                    }
                    instructions::LoadTarget::B => {
                        println!("Setting b={}", source_value);
                        self.registers.b = source_value;
                    }
                    instructions::LoadTarget::C => {
                        println!("Setting c={}", source_value);
                        self.registers.c = source_value;
                    }
                    instructions::LoadTarget::D => {
                        println!("Setting d={}", source_value);
                        self.registers.d = source_value;
                    }
                    instructions::LoadTarget::E => {
                        println!("Setting e={}", source_value);
                        self.registers.e = source_value;
                    }
                    instructions::LoadTarget::H => {
                        println!("Setting h={}", source_value);
                        self.registers.h = source_value;
                    }
                    instructions::LoadTarget::L => {
                        println!("Setting l={}", source_value);
                        self.registers.l = source_value;
                    }
                    instructions::LoadTarget::HL_ => {
                        let hl = self.registers.get_hl();
                        self.bus.write_byte(hl, source_value);
                        println!("Setting {:x}={}", hl, source_value);
                    }
                    instructions::LoadTarget::HLD => {
                        let hl = self.registers.get_hl();
                        self.bus.write_byte(hl, source_value);
                        println!("Setting {:x}={}", hl, source_value);
                        self.registers.set_hl(hl - 1);
                    }
                    instructions::LoadTarget::HLI => {
                        let hl = self.registers.get_hl();
                        self.bus.write_byte(hl, source_value);
                        println!("Setting {:x}={}", hl, source_value);
                        self.registers.set_hl(hl + 1);
                    }
                    instructions::LoadTarget::N16_ => {
                        let least_significant_byte = self.bus.read_byte(self.pc) as u16;
                        let most_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                        let value = (most_significant_byte << 8) | least_significant_byte;

                        self.pc = self.pc.wrapping_add(2);

                        println!("Setting {:x}={}", value, source_value);
                        self.bus.write_byte(value, source_value);
                    }
                    _ => {
                        panic!("TODO: implement other targets")
                    }
                };
            }
            Instruction::LDH(target, source) => {
                let source_value = match source {
                    instructions::LoadHSource::A => self.registers.a,
                    _ => {
                        panic!("TODO: implement other sources")
                    }
                };
                match target {
                    instructions::LoadHTarget::C_ => {
                        println!("LDH 0xff{:x}={}", self.registers.c, source_value);
                        self.bus
                            .write_byte(0xff00 + (self.registers.c as u16), source_value);
                    }
                    instructions::LoadHTarget::N8_ => {
                        let n8 = self.bus.read_byte(self.pc);
                        println!("LDH 0xff{:x}={}", n8, source_value);
                        self.bus.write_byte(0xff00 + (n8 as u16), source_value);
                        self.pc = self.pc.wrapping_add(1);
                    }
                    _ => {
                        panic!("TODO: implement other targets")
                    }
                };
            }
            Instruction::LDN16(target) => {
                let least_significant_byte = self.bus.read_byte(self.pc) as u16;
                let most_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                let value = (most_significant_byte << 8) | least_significant_byte;
                self.pc = self.pc.wrapping_add(2);

                match target {
                    instructions::LoadTypeN16::DE => {
                        self.registers.set_de(value);
                    }
                    instructions::LoadTypeN16::SP => {
                        self.sp = value;
                    }
                    instructions::LoadTypeN16::HL => {
                        self.registers.set_hl(value);
                    }
                    _ => {
                        panic!("Unknown target.");
                    }
                }
            }
            Instruction::POP(target) => match target {
                instructions::PopTarget::BC => {
                    self.registers.c = self.bus.read_byte(self.sp);
                    self.sp = self.sp.wrapping_add(1);
                    self.registers.b = self.bus.read_byte(self.sp);
                    self.sp = self.sp.wrapping_add(1);
                }
                _ => {
                    panic!("Unknown target.");
                }
            },
            Instruction::PUSH(target) => match target {
                instructions::PushTarget::BC => {
                    self.sp = self.sp.wrapping_sub(1);
                    self.bus.write_byte(self.sp, self.registers.b);
                    self.sp = self.sp.wrapping_sub(1);
                    self.bus.write_byte(self.sp, self.registers.c);
                }
                _ => {
                    panic!("Unknown target.");
                }
            },
            Instruction::RET => {
                let least_significant_byte = self.bus.read_byte(self.sp) as u16;
                self.sp = self.sp.wrapping_add(1);
                let most_significant_byte = self.bus.read_byte(self.sp) as u16;
                self.sp = self.sp.wrapping_add(1);
                self.pc = (most_significant_byte << 8) | least_significant_byte;

                println!("Returning 0x{:x}", self.pc);
            }
            Instruction::RL(target) => match target {
                instructions::RotateTarget::C => {
                    let highest_bit = self.registers.c & 0x80 != 0;
                    self.registers.c <<= 1;
                    if self.flags_register.carry {
                        self.registers.c |= 0x1;
                    }
                    self.flags_register.zero = self.registers.c == 0;
                    self.flags_register.subtract = false;
                    self.flags_register.half_carry = false;
                    self.flags_register.carry = highest_bit;
                }
            },
            Instruction::RLA => {
                let highest_bit = self.registers.a & 0x80 != 0;
                self.registers.a <<= 1;
                if self.flags_register.carry {
                    self.registers.a |= 0x1;
                }
                self.flags_register.zero = false;
                self.flags_register.subtract = false;
                self.flags_register.half_carry = false;
                self.flags_register.carry = highest_bit;
            }
            Instruction::XOR(target, source) => {
                let source_value = match source {
                    instructions::ArithmeticSource::A => self.registers.a,
                    _ => {
                        panic!("TODO: implement other sources")
                    }
                };
                match target {
                    instructions::ArithmeticTarget::A => self.registers.a ^= source_value,
                    _ => {
                        panic!("TODO: implement other targets")
                    }
                };
            }
            _ => {}
        }
    }

    pub fn load_bootstrap(&mut self, bootstrap_bin: &[u8]) {
        self.bus.write_array(0, bootstrap_bin);
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            flags_register: FlagsRegister::default(),
            sp: 0,
            pc: 0,
            bus: MemoryBus::default(),
        }
    }
}
